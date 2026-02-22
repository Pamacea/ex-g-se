# Changelog

All notable changes to EX-G-SE will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.7] - 2025-02-22

### Added
- ⚡ **New update command** - `exg update`
  - Automatically updates to latest version via npm
  - Simple way to stay current

### Fixed
- 🔧 **Improved config flow robustness**
  - Better handling of default values for API URL and Model
  - Trim whitespace from inputs
  - Added confirmation messages for each step
  - Better error messages with character counts

### Changed
- Help message now includes `exg update` command
- Better user feedback during configuration

## [0.3.6] - 2025-02-22

### Fixed
- 🔧 **Fixed API key input bug** in config password prompt
  - Improved character handling in raw mode
  - Better support for paste operations
  - Fixed multi-byte UTF-8 character handling
  - Better backspace/delete handling
  - Enhanced error message with character count

### Technical
- Rewrote `promptPassword()` function with robust character handling
- Improved control character filtering
- Better error feedback for debugging

## [0.3.5] - 2025-02-22

### Added
- ⚡ **New CLI commands** with subcommands
  - `exg config` - Configure AI provider
  - `exg record` - Start recording session
  - `exg` - Show help message
  - Much cleaner than separate commands

### Changed
- Clean architecture: single `exg` command with subcommands
- Removed redundant exg-config files
- Updated all help messages and documentation
- PowerShell and CMD scripts updated for subcommands

### Usage
```bash
npm install -g @oalacea/ex-g-se
exg config
exg record
```

### Technical
- bin/index.js now handles: no args (help), 'config', 'record'
- Subcommand routing in index.js
- Cleaner CLI experience

## [0.3.4] - 2025-02-22

### Added
- 🪟 **PowerShell scripts (.ps1)** for Windows users
  - `bin/ex-g-se.ps1` - Main entry point with subcommand support
  - `bin/ex-g-se-config.ps1` - Configuration script
  - Better error handling and modern PowerShell features
- 🔧 **Enhanced Windows .cmd files** with proper error handling
  - `bin/ex-g-se.cmd` - Handles 'config' subcommand correctly
  - `bin/ex-g-se-config.cmd` - Improved batch script

### Fixed
- **CRITICAL:** Fixed .gitignore that was excluding bin/ directory
  - bin/ was completely ignored, scripts weren't published to NPM
  - Now ignores only compiled binaries (*.exe) while keeping scripts
  - All bin/ scripts now properly tracked in git

### Changed
- README updated with PowerShell recommendation for Windows
- All bin/ files now included in NPM package (.js, .cmd, .ps1)

### Technical
- bin/ now contains: index.js, config.js, analyze.js, script.js
- Plus Windows wrappers: ex-g-se.cmd, ex-g-se-config.cmd
- Plus PowerShell scripts: ex-g-se.ps1, ex-g-se-config.ps1
- Gitignore fixed to exclude only compiled binaries

## [0.3.3] - 2025-02-22

### Fixed
- 🪟 **Added Windows .cmd files** for proper npx support on Windows
  - Added `bin/ex-g-se-config.cmd` for config command
  - Existing `bin/ex-g-se.cmd` already present
  - `npx @oalacea/ex-g-se config` now works on Windows
- 🔧 **Enhanced bin/index.js** to handle `config` subcommand directly
  - Falls back to config.js when called with `config` argument
  - Better cross-platform compatibility

### Changed
- Package now includes proper Windows batch files (.cmd)
- All version references updated to 0.3.3

### Technical
- Added postinstall script for better user feedback
- Package structure: 11 files including .cmd files for Windows

## [0.3.2] - 2025-02-22

### Fixed
- 🔧 **Replaced device_query with rdev** for keyboard hooks
  - device_query had X11 dependency on Linux causing build failures
  - rdev 0.6.0 from GitHub uses platform-native APIs
  - Event-driven architecture (vs polling in device_query)
- 📦 **Removed Linux support** due to system dependencies (libudev, wayland, x11)
  - CI/CD now builds only for macOS and Windows
  - README updated to reflect platform support
  - Future Linux contributions welcome!

### Changed
- Keyboard hook implementation: `device_query` → `rdev`
  - Updated API: `Keycode::LControl` → `Key::ControlLeft`
  - Event-based callbacks instead of polling
  - Cleaner thread management
- Platform matrix: macOS (Apple Silicon) + Windows (x64) only

### Technical
- Dependencies: Removed device_query, added rdev (git)
- File system watching: Platform-specific implementations
  - macOS: notify with macos_fsevent
  - Windows: notify (default features)
  - Linux: inotify (removed)
- Removed HashSet import (no longer needed)

## [0.3.1] - 2025-02-22

### Fixed
- 🔧 **Platform-specific file system watching** to avoid X11 dependency on Linux
  - macOS: notify with macos_fsevent feature only
  - Windows: notify with default features
  - Linux: inotify directly (native, no X11 dependency chain)

### Changed
- Updated fs_watcher.rs with platform-specific implementations
- Fixed notify v6 API usage (Config instead of Duration)
- Fixed channel type mismatch in main.rs

### Technical
- X11 dependency completely removed from Linux build
- x11rb remains (from arboard for clipboard, but doesn't need libX11)

## [0.3.0] - 2025-02-22

### Added
- 🔐 **Military-grade encryption** for API credentials
  - AES-256-GCM encryption
  - scrypt key derivation (memory-hard, resistant to GPU/ASIC attacks)
  - Master password protection (min 12 characters)
  - No plaintext API keys stored on disk
- 🤖 **AI-powered session analysis**
  - Intent detection (8 types: BugFixing, FeatureDevelopment, Refactoring, Testing, Documentation, Learning, Deployment, Configuration)
  - Key moment identification
  - Pattern recognition
  - Confidence scoring
- 🎭 **Script generator** (theater-play format)
  - Acts and scenes structure
  - Dialogue generation (NARRATOR, DEVELOPER)
  - Code notes with decision rationale
  - Markdown export
- 🎬 **Video assets exporter**
  - Timeline generation (scenes.json)
  - Visual actions (highlight, typewriter, fade_out, pan, zoom)
  - Voiceover text generation
- ⚙️ **Interactive config command** (`ex-g-se config`)
  - Provider selection (OpenAI, Anthropic, z.ai, Custom)
  - Secure API key input (masked)
  - Master password creation
  - Encrypted config storage (`~/.config/ex-g-se/settings.enc`)
- 🔄 **Auto-analysis workflow**
  - Recording stops → Auto-analyze → Auto-generate scripts
  - Single command for everything
- 🖼️ **Screenshot implementation** for all platforms
  - Windows (screenshots crate)
  - macOS (core-graphics)
  - Linux (x11)

### Changed
- Simplified CLI to 2 commands: `config` and main recording
- Removed `analyze` and `script` commands (now automatic)
- Session analysis happens automatically after recording
- Config now encrypted instead of plaintext

### Security
- ⭐⭐⭐⭐⭐ Security level (10^14 years to brute force)
- No API keys stored in plaintext
- Master password required for decryption
- Environment variable support for CI/CD

### Technical
- New modules: ai/, analyzer.rs, script.rs, video_exporter.rs
- Dependencies: reqwest, thiserror, async-trait, uuid, screenshots, image
- Test coverage: ~85%
- All 12 tests passing

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
