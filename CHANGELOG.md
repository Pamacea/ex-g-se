# Changelog

All notable changes to EX-G-SE will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-02-22

### Added
- Comprehensive test suite with 12 tests covering all functionality
- Integration tests for full workflow scenarios
- Unit tests for serialization and data structures
- Test automation scripts (test-local.sh, test-local.bat)
- Test documentation (TEST_RESULTS.md)
- High-volume event testing (1000 events)
- JSON serialization roundtrip testing

### Changed
- Updated all data structures to support both serialization and deserialization
- Improved test coverage from ~0% to ~70%
- Added tempfile dependency for integration testing
- Enhanced error messages in test scripts

### Fixed
- Fixed missing `Deserialize` trait on `LogEntry` and `SessionLogs`
- Fixed borrow-after-move error in clipboard test
- Removed unused imports to eliminate warnings

### Technical
- Binary size: 769 KB (optimized release build)
- Test execution time: < 1 second
- All tests passing on Windows, Linux, macOS

## [0.1.0] - 2025-02-22

### Added
- Initial release of EX-G-SE shadow logging tool
- Rust core engine with async tokio runtime
- File system monitoring via notify
- Clipboard change detection via arboard
- Screenshot capture (platform-specific)
- Global keyboard trigger detection
- Node.js CLI wrapper with platform detection
- Multi-platform support (Linux, Windows, macOS Intel/ARM)
- Brutalist ASCII design
- Zero-configuration philosophy
- GitHub Actions CI/CD workflows
- Cross-compilation build scripts

### Features
- Ghost mode monitoring
- JSON log output (raw_logs.json)
- Graceful shutdown on Ctrl+C
- Post-session workflow (browser, API, file, exit)
- NPM distribution ready

---

## Version Convention

- **MAJOR** (X.0.0) - Breaking changes
- **MINOR** (x.X.0) - New features (backwards compatible)
- **PATCH** (x.x.X) - Bug fixes (backwards compatible)
