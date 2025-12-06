## 0.1.0 – Initial Release

- Introduced `logguard` Flutter plugin for secure log sanitization.
- Masks sensitive information such as passwords, API keys, JWTs, emails, UUIDs, and credit card numbers.
- Hybrid scanning approach: fast manual ASCII scanning + minimal regex for complex patterns.
- Fully compatible with Android (API 21+). iOS support planned for future releases.
- Provides FFI bindings for native performance and low memory overhead.
- Includes safe Rust memory management for C strings returned to Flutter.
