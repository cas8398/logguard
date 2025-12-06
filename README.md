# 🛡️ LogGuard

A high-performance Flutter plugin that automatically sanitizes sensitive data in logs using Rust FFI with regex-based pattern matching.

[![Pub Version](https://img.shields.io/pub/v/logguard.svg)](https://pub.dev/packages/logguard)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## ⚡ Features

- **Automatic Sanitization**: Intercepts `print()` and `debugPrint()` calls
- **High Performance**: Rust FFI implementation with optimized regex patterns
- **Zero Config**: Works out of the box with sensible defaults
- **UTF-8 Support**: Handles international characters correctly
- **Streaming Support**: Efficiently processes large log messages

## 🔒 What Gets Sanitized?

LogGuard automatically masks the following sensitive patterns:

| Pattern           | Example                                    | Sanitized           |
| ----------------- | ------------------------------------------ | ------------------- |
| **Passwords**     | `password=secret123`                       | `password=********` |
| **Auth Tokens**   | `Bearer abc123token`                       | `Bearer [MASKED]`   |
| **AWS Keys**      | `AKIAIOSFODNN7EXAMPLE`                     | `[MASKED]`          |
| **Credit Cards**  | `4532-1234-5678-9010`                      | `[MASKED]`          |
| **Emails**        | `user@example.com`                         | `[MASKED]`          |
| **Phone Numbers** | `+1-555-123-4567`                          | `[MASKED]`          |
| **UUIDs**         | `550e8400-e29b-41d4-a716-446655440000`     | `[MASKED]`          |
| **JWT Tokens**    | `eyJhbGc...`                               | `[MASKED]`          |
| **Hex Hashes**    | `a94a8fe5ccb19ba61c4c0873d391e987982fbbd3` | `[MASKED]`          |

## 📦 Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  logguard: ^0.1.0
```

Then run:

```bash
flutter pub get
```

## 🚀 Quick Start

### Protect Your Entire App

```dart
import 'package:flutter/material.dart';
import 'package:logguard/logguard.dart';

void main() {
  LogGuard.runApp(
    MaterialApp(
      home: MyHomePage(),
    ),
  );
}
```

That's it! All `print()` and `debugPrint()` calls are now automatically sanitized.

### Protect Specific Code Blocks

```dart
void main() async {
  await LogGuard.runGuarded(() {
    // Your code here
    print('User password: secret123'); // Automatically sanitized
    runApp(MyApp());
  });
}
```

### Manual Sanitization

```dart
import 'package:logguard/logguard.dart';

void logUserInfo(String email, String token) {
  // Manual sanitization
  final safeEmail = LogGuard.sanitize(email);
  print('User email: $safeEmail');

  // Or use extension
  print('Auth token: ${token.sanitized}');

  // Safe logging with levels
  LogGuard.log(
    'User logged in with $email',
    level: LogLevel.info,
    toConsole: true,
    toDeveloper: true,
  );
}
```

### Advanced Usage

```dart
// Custom log levels
LogGuard.log(
  'Critical error occurred',
  level: LogLevel.error,
  error: exception,
  stackTrace: stackTrace,
  name: 'MyService',
);

// Safe print helpers
LogGuard.safePrint('This will be sanitized');
LogGuard.safeDebug('Debug message', wrapWidth: 80);

// Check FFI availability
if (LogGuard.isFFIAvailable) {
  print('Using high-performance Rust sanitizer');
} else {
  print('Using Dart fallback sanitizer');
}

// String extension
final sensitive = 'password=abc123';
final safe = sensitive.sanitized;
print(safe); // Output: password=********
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│         Flutter Application             │
│  (print, debugPrint, custom logs)       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│        LogGuard Dart Layer              │
│  • Zone interception                    │
│  • Hook management                      │
│  • Fallback logic                       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Rust FFI Bridge                 │
│  • Native performance                   │
│  • Optimized regex                      │
│  • UTF-8 aware scanning                 │
│  • Streaming support                    │
└─────────────────────────────────────────┘
```

## 🎯 Platform Support

| Platform   | Status      | Notes               |
| ---------- | ----------- | ------------------- |
| ✅ Android | Supported   | Full FFI support    |
| 🚧 Linux   | Coming Soon | Planned for v0.2.0  |
| 🚧 Windows | Coming Soon | Planned for v0.3.0  |
| ⏳ iOS     | Planned     | Future release      |
| ⏳ macOS   | Planned     | Future release      |
| ⏳ Web     | Planned     | WASM implementation |

## ⚙️ Configuration

LogGuard works with zero configuration, but you can customize behavior:

```dart
// Setup hooks manually
LogGuard.setupHooks();

// Remove hooks when needed
LogGuard.removeHooks();

// Use streaming for very large logs (>10KB)
// Automatically handled internally
```

## 📊 Performance

LogGuard is designed for production use with minimal overhead:

- **Small messages (<10KB)**: Single-pass scanning
- **Large messages (>10KB)**: Automatic chunking with 10KB threshold
- **Regex patterns**: Lazy-loaded and cached
- **UTF-8 aware**: Proper character boundary handling
- **Memory safe**: Capped result buffer (2MB max)

Benchmarks (on Pixel 6):

- 1KB log: ~0.1ms
- 10KB log: ~0.5ms
- 100KB log: ~3ms (chunked)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Roadmap

- [x] Android support with Rust FFI
- [ ] Linux support
- [ ] Windows support
- [ ] iOS support
- [ ] macOS support
- [ ] Web support (WASM)
- [ ] Custom pattern configuration
- [ ] Log encryption option
- [ ] Analytics integration

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for native performance
- Regex patterns optimized using [regex](https://docs.rs/regex/) crate
- Inspired by security best practices from OWASP

## 📞 Support

- 🐛 [Report a bug](https://github.com/cas8398/logguard/issues)
- 💡 [Request a feature](https://github.com/cas8398/logguard/issues)

---

**⚠️ Security Notice**: While LogGuard significantly reduces the risk of sensitive data exposure in logs, it should be used as part of a comprehensive security strategy. Always follow security best practices and never intentionally log sensitive information.
