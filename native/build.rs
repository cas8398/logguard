fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Platform-specific linker flags
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-dead_strip");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    }
}