## 0.1.1

- Added optional enable parameter to runApp() and runGuarded() methods
- Added runtime control methods: enable(), disable(), and toggle()
- Fixed conditional FFI initialization to only run when LogGuard is enabled
- Improved performance by skipping sanitization when LogGuard is disabled
- Added global enabled static flag for status checking
- Enhanced example with practical usage demonstration

## 0.1.0 – Initial Release

- Introduced `logguard` Flutter plugin for secure log sanitization.
- Masks sensitive information such as passwords, API keys, JWTs, emails, UUIDs, and credit card numbers.
- Hybrid scanning approach: fast manual ASCII scanning + minimal regex for complex patterns.
- Fully compatible with Android (API 21+). iOS support planned for future releases.
- Provides FFI bindings for native performance and low memory overhead.
- Includes safe Rust memory management for C strings returned to Flutter.
