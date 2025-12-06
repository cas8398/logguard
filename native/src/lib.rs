//! LogGuard Native FFI Library
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use lazy_static::lazy_static;
use regex::bytes::Regex;

// Minimal regex set for complex patterns only
lazy_static! {
    // Email - supports international domains
    static ref EMAIL_RE: Regex = Regex::new(
        r#"(?i)\b[A-Z0-9._%+\-]{1,64}@[A-Z0-9.\-]{1,255}\.(?:[A-Z]{2,63}|xn--[A-Z0-9]+)\b"#
    ).unwrap();
    
    // Credit card - pattern only (fast), Luhn check optional
    static ref CC_PATTERN_RE: Regex = Regex::new(
        r#"\b(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|6(?:011|5[0-9]{2})[0-9]{12}|(?:2131|1800|35\d{3})\d{11})\b"#
    ).unwrap();
    
    // JWT - extended to cover more variants
    static ref JWT_RE: Regex = Regex::new(
        r#"\bey[A-Za-z0-9_-]{10,}\.[A-Za-z0-9._-]{10,}\.[A-Za-z0-9._-]{10,}?\b"#
    ).unwrap();
}

/// Scanner with UTF-8 awareness and streaming support
fn apply_production_scan(input: &str, streaming_threshold: usize) -> String {
    // For very large inputs, use chunked processing
    if input.len() > streaming_threshold {
        return scan_in_chunks(input, streaming_threshold);
    }
    
    // Normal scanning for smaller inputs
    let mut result = String::with_capacity(input.len().saturating_mul(2).min(2_000_000)); // Cap at 2MB
    let mut chars = input.char_indices().peekable();
    
    while let Some((i, ch)) = chars.next() {
        let remaining_str = &input[i..];
        let remaining_bytes = remaining_str.as_bytes();
        let mut matched = false;
        
        // FAST PATH 1: Common prefixes with UTF-8 awareness
        if remaining_str.len() >= 8 {
            let prefix = &remaining_str[..8.min(remaining_str.len())];
            let prefix_lower = prefix.to_lowercase();
            
            if prefix_lower.starts_with("password") || 
               prefix_lower.starts_with("passwd:") ||
               prefix_lower.starts_with("secret") || 
               prefix_lower.starts_with("pass") || 
               prefix_lower.starts_with("pwd=") {
                
                // Find the separator
                let mut j = i + prefix.len();
                let mut separator_found = false;
                
                // Advance through characters until separator
                for (next_i, next_ch) in input[i + prefix.len()..].char_indices() {
                    if next_ch == '=' || next_ch == ':' {
                        j = i + prefix.len() + next_i + 1;
                        separator_found = true;
                        break;
                    }
                    if !next_ch.is_whitespace() && next_ch != '=' && next_ch != ':' {
                        break;
                    }
                }
                
                if separator_found {
                    // Copy prefix + separator
                    result.push_str(&input[i..j]);
                    
                    // Skip any whitespace after separator - use a simple approach
                    let mut whitespace_count = 0;
                    while let Some((_, next_ch)) = chars.peek() {
                        if next_ch.is_whitespace() {
                            result.push(*next_ch);
                            whitespace_count += 1;
                            // Consume it
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    // Mask the value
                    result.push_str("********");
                    
                    // Skip the actual value
                    while let Some((_, next_ch)) = chars.peek() {
                        if !next_ch.is_whitespace() && *next_ch != '\r' && *next_ch != '\n' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    matched = true;
                }
            }
        }
        
        // FAST PATH 2: Bearer/Basic tokens with flexible whitespace
        if !matched && remaining_str.len() >= 6 {
            let token_type = if remaining_bytes.starts_with(b"Bearer") || remaining_bytes.starts_with(b"bearer") {
                Some(("Bearer", 6))
            } else if remaining_bytes.starts_with(b"Basic") || remaining_bytes.starts_with(b"basic") {
                Some(("Basic", 5))
            } else {
                None
            };
            
            if let Some((token_name, token_len)) = token_type {
                // Check for separator after token
                let after_token = &remaining_str[token_len..];
                if !after_token.is_empty() {
                    let next_char = after_token.chars().next().unwrap();
                    if next_char == ' ' || next_char == ':' || next_char == '=' {
                        result.push_str(token_name);
                        if next_char == ':' || next_char == '=' {
                            result.push(next_char);
                        } else {
                            result.push(' ');
                        }
                        
                        // Skip token header
                        for _ in 0..token_len {
                            chars.next();
                        }
                        // Skip separator
                        chars.next();
                        
                        // Mask the token
                        result.push_str("[MASKED]");
                        
                        // Skip the actual token
                        while let Some((_, next_ch)) = chars.peek() {
                            if !next_ch.is_whitespace() && *next_ch != '\r' && *next_ch != '\n' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        matched = true;
                    }
                }
            }
        }
        
        // FAST PATH 3: AWS Key (case sensitive, exact format)
        if !matched && remaining_bytes.len() >= 20 {
            if remaining_bytes.starts_with(b"AKIA") {
                // Verify remaining characters are alnum
                let mut is_valid = true;
                for &c in &remaining_bytes[4..20] {
                    if !c.is_ascii_alphanumeric() {
                        is_valid = false;
                        break;
                    }
                }
                
                if is_valid {
                    result.push_str("[MASKED]");
                    // Skip 20 bytes (4 chars already consumed)
                    for _ in 0..16 {
                        chars.next();
                    }
                    matched = true;
                }
            }
        }
        
        // FAST PATH 4: UUID (case insensitive hex)
        if !matched && remaining_str.len() >= 36 {
            let uuid_candidate = &remaining_str[..36];
            if is_uuid_like(uuid_candidate) {
                result.push_str("[MASKED]");
                // Skip 36 characters (current one already consumed)
                for _ in 0..35 {
                    chars.next();
                }
                matched = true;
            }
        }
        
        // FAST PATH 5: Hex hashes (32-128 chars, case insensitive)
        if !matched && ch.is_ascii_hexdigit() {
            let mut hex_count = 0;
            let mut temp_chars = input[i..].chars().peekable();
            
            while let Some(next_ch) = temp_chars.next() {
                if next_ch.is_ascii_hexdigit() {
                    hex_count += 1;
                    if hex_count > 128 { break; }
                } else {
                    break;
                }
            }
            
            if hex_count >= 32 && hex_count <= 128 {
                result.push_str("[MASKED]");
                // Skip hex characters (current one already consumed)
                for _ in 0..hex_count - 1 {
                    chars.next();
                }
                matched = true;
            }
        }
        
        // FAST PATH 6: Phone numbers
        if !matched && (ch == '+' || ch.is_ascii_digit()) {
            let phone_candidate = extract_phone_candidate(&input[i..]);
            if phone_candidate.len() >= 10 && is_phone_like(&phone_candidate) {
                result.push_str("[MASKED]");
                // Skip phone number (current char already consumed)
                for _ in 0..phone_candidate.chars().count() - 1 {
                    chars.next();
                }
                matched = true;
            }
        }
        
        // SLOW PATH: Regex for complex patterns
        if !matched {
            // Check word boundaries
            let is_word_start = i == 0 || !input[..i].chars().last().unwrap().is_alphanumeric();
            
            if is_word_start {
                // Email regex
                if let Some(mat) = EMAIL_RE.find(remaining_bytes) {
                    if mat.start() == 0 {
                        result.push_str("[MASKED]");
                        // Skip email - convert bytes to char count
                        let matched_bytes = &remaining_bytes[mat.start()..mat.end()];
                        let matched_str = String::from_utf8_lossy(matched_bytes);
                        // Skip matched characters (current one already consumed)
                        for _ in 0..matched_str.chars().count() - 1 {
                            chars.next();
                        }
                        matched = true;
                    }
                }
                
                // Credit card pattern
                if !matched {
                    if let Some(mat) = CC_PATTERN_RE.find(remaining_bytes) {
                        if mat.start() == 0 {
                            // Optional: Add Luhn check here for accuracy
                            let matched_bytes = &remaining_bytes[mat.start()..mat.end()];
                            let cc_str = String::from_utf8_lossy(matched_bytes);
                            if is_possible_credit_card(&cc_str) {
                                result.push_str("[MASKED]");
                                // Skip credit card characters
                                for _ in 0..cc_str.chars().count() - 1 {
                                    chars.next();
                                }
                                matched = true;
                            }
                        }
                    }
                }
                
                // JWT regex (extended)
                if !matched {
                    if let Some(mat) = JWT_RE.find(remaining_bytes) {
                        if mat.start() == 0 {
                            result.push_str("[MASKED]");
                            let matched_bytes = &remaining_bytes[mat.start()..mat.end()];
                            let matched_str = String::from_utf8_lossy(matched_bytes);
                            // Skip JWT characters
                            for _ in 0..matched_str.chars().count() - 1 {
                                chars.next();
                            }
                            matched = true;
                        }
                    }
                }
            }
        }
        
        // Default: copy character
        if !matched {
            result.push(ch);
        }
    }
    
    result
}

/// Chunked scanning for very large inputs
fn scan_in_chunks(input: &str, chunk_size: usize) -> String {
    let mut result = String::with_capacity(input.len().min(2_000_000));
    let mut last_idx = 0;
    
    // Process in chunks, preserving word boundaries
    let mut chunk_end = chunk_size;
    while last_idx < input.len() {
        // Adjust chunk end to not split words
        let adjusted_end = if chunk_end < input.len() {
            // Find the next whitespace or natural boundary
            input[chunk_end..]
                .char_indices()
                .find(|(_, ch)| ch.is_whitespace() || *ch == ',' || *ch == '.' || *ch == ';')
                .map(|(offset, _)| chunk_end + offset + 1)
                .unwrap_or(input.len())
        } else {
            input.len()
        };
        
        let chunk = &input[last_idx..adjusted_end];
        let scanned_chunk = apply_production_scan(chunk, usize::MAX); // No further chunking
        
        result.push_str(&scanned_chunk);
        last_idx = adjusted_end;
        chunk_end = last_idx + chunk_size;
    }
    
    result
}

/// Check if string is UUID-like (case insensitive)
fn is_uuid_like(s: &str) -> bool {
    if s.len() != 36 { return false; }
    
    let chars: Vec<char> = s.chars().collect();
    let dash_positions = [8, 13, 18, 23];
    
    // Check dashes
    for &pos in &dash_positions {
        if chars[pos] != '-' { return false; }
    }
    
    // Check hex characters
    for (i, &ch) in chars.iter().enumerate() {
        if !dash_positions.contains(&i) && !ch.is_ascii_hexdigit() {
            return false;
        }
    }
    
    true
}

/// Extract potential phone number candidate
fn extract_phone_candidate(s: &str) -> String {
    let mut result = String::new();
    let mut digit_count = 0;
    
    for ch in s.chars().take(30) { // Reasonable phone length limit
        if ch.is_ascii_digit() {
            result.push(ch);
            digit_count += 1;
            if digit_count >= 15 { break; } // Max phone digits
        } else if ch == '+' && result.is_empty() {
            result.push(ch);
        } else if ch == '-' || ch == '.' || ch == '(' || ch == ')' || ch == ' ' {
            result.push(ch);
        } else {
            break;
        }
    }
    
    result
}

/// Check if string looks like a phone number
fn is_phone_like(s: &str) -> bool {
    let mut digit_count = 0;
    let mut total_len = 0;
    
    for ch in s.chars() {
        total_len += 1;
        if ch.is_ascii_digit() {
            digit_count += 1;
        } else if !(ch == '+' || ch == '-' || ch == '.' || ch == '(' || ch == ')' || ch == ' ') {
            return false;
        }
    }
    
    digit_count >= 7 && digit_count <= 15 && total_len >= 10 && total_len <= 30
}

/// Basic credit card validation (pattern + simple checks)
fn is_possible_credit_card(s: &str) -> bool {
    // Remove separators
    let clean: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if clean.len() < 13 || clean.len() > 19 {
        return false;
    }
    
    // Optional: Add Luhn check here for production
    // For now, just check pattern matched by regex
    true
}

/// Initialize the log sanitizer
#[no_mangle]
pub extern "C" fn logguard_init() {
    // Force initialization of regex patterns
    lazy_static::initialize(&EMAIL_RE);
    lazy_static::initialize(&CC_PATTERN_RE);
    lazy_static::initialize(&JWT_RE);
}

/// Sanitize input string (main entry point)
#[no_mangle]
pub extern "C" fn logguard_sanitize(input: *const c_char) -> *mut c_char {
    // Null check
    if input.is_null() {
        return ptr::null_mut();
    }

    // Convert C string to Rust string
    let input_str = unsafe {
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => {
                // Return empty string on error
                return match CString::new("") {
                    Ok(c) => c.into_raw(),
                    Err(_) => ptr::null_mut(),
                };
            }
        }
    };

    // Short circuit for small inputs
    if input_str.len() < 10 {
        return match CString::new(input_str) {
            Ok(c) => c.into_raw(),
            Err(_) => ptr::null_mut(),
        };
    }

    // Apply production scanning with 10KB chunk threshold
    let result = apply_production_scan(input_str, 10_000);

    // Convert back to C string
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Alternative: Streaming scanner for very large inputs
#[no_mangle]
pub extern "C" fn logguard_sanitize_streaming(input: *const c_char, chunk_size: usize) -> *mut c_char {
    if input.is_null() {
        return ptr::null_mut();
    }

    let input_str = unsafe {
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => {
                return match CString::new("") {
                    Ok(c) => c.into_raw(),
                    Err(_) => ptr::null_mut(),
                };
            }
        }
    };

    // Use chunked scanning with custom chunk size
    let safe_chunk_size = chunk_size.max(1024).min(100_000); // Between 1KB and 100KB
    let result = scan_in_chunks(input_str, safe_chunk_size);

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free memory allocated by Rust
#[no_mangle]
pub extern "C" fn logguard_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}