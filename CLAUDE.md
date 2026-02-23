# CLAUDE.md - EX-G-SE Project Configuration v0.5.3

> **Last Updated:** 2025-02-22
> **Version:** 4.0.0

---

## 🎯 Project Identity

**Name:** EX-G-SE (Ex-Ghost Observability - Shadow Edition)

**Description:** AI-powered shadow logging observability tool for development sessions

**Tech Stack:**
- **Core:** Rust (Tokio async runtime)
- **CLI:** Node.js (cross-platform wrapper)
- **Encryption:** AES-256-GCM + scrypt (military-grade)
- **AI Providers:** OpenAI, Anthropic, z.ai (extensible)
- **Colors:** chalk for rich terminal output

**Purpose:** Record development sessions and manage them with:
- Live progress tracking (every 30 seconds)
- Session labels and tags
- Multiple export formats (JSON, Markdown, CSV)
- Session search and statistics
- Auto-stop functionality
- Smart file filtering

---

## 📁 Project Structure

```
ex-g-se/
├── core/                   # Rust core engine
│   ├── src/
│   │   ├── ai/            # AI provider system
│   │   ├── analyzer.rs    # Session analysis engine
│   │   ├── cli.rs         # CLI argument parsing (NEW)
│   │   ├── script.rs      # Script generator
│   │   ├── screenshot.rs  # Screenshot capture
│   │   ├── video_exporter.rs
│   │   ├── fs_watcher.rs  # File system monitoring
│   │   ├── watchers.rs    # Event types
│   │   ├── lib.rs         # Library interface
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tests/             # Integration tests
├── bin/                   # Node.js CLI wrapper
│   ├── index.js           # Main entry (all commands)
│   └── config.js          # Config with encryption
├── .github/workflows/     # CI/CD
│   ├── release.yml        # Release & NPM publish
│   ├── ci.yml             # Continuous integration
│   └── lint.yml           # Linting
├── package.json           # NPM package config
├── README.md              # User documentation
├── GUIDE.md               # Complete usage guide
├── REFERENCE.md           # Command reference
├── ARCHITECTURE.md        # Technical architecture
└── CHANGELOG.md           # Version history
```

---

## 🚀 Quick Commands

### Development
```bash
# Run locally
npm run dev

# Build Rust
npm run build:rust

# Run tests
npm test

# Lint
npm run lint
```

### Release
```bash
# Create release
npm run release
```

### User Commands (v0.5.3)
```bash
# Configuration
exg config              # Configure AI provider
exg config list         # Show current config
exg config test         # Test API connection

# Recording
exg record              # Start recording
exg record --label X    # Record with label
exg record --tags X,Y   # Record with tags
exg record --duration N # Auto-stop after N minutes
exg record --max-events N # Stop after N events

# Session Management
exg list                # List all sessions
exg stats [id]          # Show session statistics
exg search <query>      # Search sessions

# Export
exg export json         # Export as JSON
exg export markdown     # Export as Markdown
exg export csv          # Export as CSV

# Other
exg update              # Update to latest version
exg                     # Show help
```

---

## 🔧 Key Files

### Core Rust Engine
- **`core/src/main.rs`** - Main entry point with session management
  - Live progress indicator (every 30s)
  - Session summary on shutdown
  - Press ENTER to exit
  - CLI limit checking (--duration, --max-events)
- **`core/src/cli.rs`** (NEW) - CLI argument parsing
- **`core/src/analyzer.rs`** - Intent detection and analysis
- **`core/src/script.rs`** - Theater-play script generation
- **`core/src/ai/`** - AI provider abstractions
- **`core/src/fs_watcher.rs`** - File system monitoring with smart filtering
- **`core/src/screenshot.rs`** - Screenshot capture (platform-specific)

### CLI Wrapper
- **`bin/index.js`** - Main CLI with all commands
  - CLI routing (config, record, stats, list, search, export, update)
  - Session management (list, search, stats)
  - Export functionality (JSON, Markdown, CSV)
  - Rich terminal output with chalk
- **`bin/config.js`** - Config command (encrypts and saves)

### Configuration
- **`~/.config/ex-g-se/settings.enc`** - Encrypted config (AES-256-GCM)
- **`~/.ex-g-se/sessions/`** - Session storage (timestamped files)
- **Environment variables:** `EX_G_SE_PROVIDER`, `EX_G_SE_API_KEY`, `EX_G_SE_API_URL`, `EX_G_SE_MODEL`

---

## 🔐 Security Architecture

**Encryption:**
- Algorithm: `aes-256-gcm`
- KDF: `scrypt` (memory-hard)
- Memory cost: 64 MB
- Time cost: 3 iterations
- Salt: 128 bits (random per encryption)

**Security Level:** ⭐⭐⭐⭐⭐ (~10^14 years to brute force)

**Key Principle:** No plaintext API keys stored on disk.

**Session Storage:**
- All sessions stored locally in `~/.ex-g-se/sessions/`
- No cloud sync, no telemetry
- User has full control

---

## 📝 Development Conventions

### Git Flow Master
```
TYPE: PROJECT_NAME - vX.Y.Z

- Change 1
- Change 2

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

**Types:**
- `RELEASE` - MAJOR (breaking changes)
- `UPDATE` - MINOR (new features)
- `PATCH` - PATCH (bug fixes)

### File Naming
- Rust files: `snake_case.rs`
- Node files: `camelCase.js`
- Config: `kebab-case`
- Sessions: `YYYY-MM-DD_HH-MM-SS_LABEL.json`

### Import Rules
- Rust: Use crate-level imports
- Node: Use relative imports for local modules

---

## 🧪 Testing

**Test Coverage:** ~85%

**Run tests:**
```bash
# All tests
npm test

# Rust tests only
npm run test:rust
```

**Test Files:**
- `core/tests/integration_test.rs` - 9 integration tests
- `core/src/main.rs` - 2 unit tests
- `core/src/lib.rs` - 38 unit tests (ai, analyzer, script, etc.)

**Total:** 49 tests passing

---

## 🎯 Quality Gates

Before committing:
- [ ] `npm run lint` passes
- [ ] `npm test` passes
- [ ] No plaintext credentials
- [ ] Git Flow Master format respected
- [ ] Documentation updated (README, GUIDE, REFERENCE, ARCHITECTURE)

---

## 📦 Dependencies

### Rust (v0.5.3)
```toml
# Core
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"

# Serialization
serde_json = "1"
serde = { version = "1", features = ["derive"] }

# Time
chrono = { version = "0.4", features = ["serde"] }

# Monitoring
notify = "6"
arboard = "3"
rdev = { git = "https://github.com/Narsil/rdev", default-features = false }

# Images
image = { version = "0.25", features = ["png"] }

# AI & HTTP
reqwest = { version = "0.12", features = ["json"] }

# Async
async-trait = "0.1"

# UUID
uuid = { version = "1.6", features = ["v4", "serde"] }

# Home directory
dirs = "5"

# Platform-specific
[target.'cfg(target_os = "macos")'.dependencies]
notify = { version = "6", default-features = false, features = ["macos_fsevent"] }
core-graphics = "0.23"
objc = "0.2"
cocoa = "0.25"

[target.'cfg(target_os = "windows")'.dependencies]
notify = "6"
winapi = { version = "0.3", features = ["winuser", "wingdi"] }
```

### Node.js (v0.5.3)
```json
{
  "dependencies": {
    "chalk": "^5.3.0",
    "commander": "^12.0.0"
  }
}
```

---

## 🔍 Common Tasks

### Adding a new AI provider
1. Create `core/src/ai/{provider}.rs`
2. Implement `AIProvider` trait
3. Add to `core/src/ai/mod.rs`
4. Add option in `bin/config.js`
5. Test with `exg config test`

### Adding a new intent type
1. Update `Intent` enum in `core/src/ai/types.rs`
2. Add detection logic in `core/src/analyzer.rs`
3. Add thought generation in appropriate module

### Adding a CLI option
1. Update `core/src/cli.rs` (CliConfig struct)
2. Update `ExGSeEngine` to use new option
3. Update `bin/index.js` to parse and pass option

### Adding an export format
1. Add format function in `bin/index.js`
2. Update `exportSession()` to handle new format
3. Update help text
4. Test with `exg export <format>`

### Modifying script format
1. Edit `core/src/script.rs` for Rust version
2. Or edit JavaScript version if needed

---

## 📚 Documentation

**User-facing:**
- `README.md` - Quick start and overview
- `GUIDE.md` - Complete usage guide with examples
- `REFERENCE.md` - Command reference
- `CHANGELOG.md` - Version history

**Developer:**
- `CLAUDE.md` - This file (project configuration)
- `ARCHITECTURE.md` - Technical architecture
- `IMPROVEMENTS.md` - Feature roadmap and ideas

---

## 🎨 Design Philosophy

1. **Security First** - No plaintext credentials, encrypted config
2. **Zero Config** - Works out of the box with sensible defaults
3. **User Feedback** - Live progress, session summary, clear exit
4. **Session Management** - Multiple sessions, search, stats, export
5. **Smart Filtering** - Ignores noise (node_modules, .git, target, dist, etc.)
6. **Cross-Platform** - macOS and Windows (Linux removed in v0.5.3)
7. **Privacy Respecting** - All data stored locally, no telemetry

---

## 🚀 Release Process

1. Update version in:
   - `core/Cargo.toml`
   - `package.json`
   - Documentation files (README, GUIDE, REFERENCE, ARCHITECTURE, CLAUDE)
   - `CHANGELOG.md`
2. Commit with Git Flow Master format
3. Push to main
4. Create tag: `git tag v0.5.3`
5. Push tag: `git push origin v0.5.3`
6. GitHub Actions builds and publishes to NPM

**Note:** Don't manually publish - use GitHub workflow

---

## 🆕 What's New in v0.5.3

### Features Added
1. **Live progress indicator** - Stats every 30 seconds during recording
2. **Session summary** - Detailed summary on shutdown
3. **Labels & Tags** - Organize sessions with metadata
4. **Auto-stop** - Timer or event-based limits
5. **Multiple sessions** - All sessions in `~/.ex-g-se/sessions/`
6. **Session search** - Search through all sessions
7. **Session statistics** - Detailed stats per session
8. **Export formats** - JSON, Markdown, CSV
9. **Config commands** - List and validate configuration
10. **Rich terminal** - Colored output with chalk

### Technical Changes
1. **New module:** `core/src/cli.rs` for CLI argument parsing
2. **Direct session save:** Rust saves directly to sessions directory
3. **Press ENTER to exit:** Clear user feedback
4. **Better file filtering:** More patterns ignored
5. **Dependencies:** Added chalk, commander, dirs

### Breaking Changes
1. `raw_logs.json` no longer created in current directory
2. Sessions now in `~/.ex-g-se/sessions/` by default
3. Linux support removed (system dependencies)

---

**Project:** EX-G-SE
**Version:** 4.0.0
**Maintainer:** Oalacea
**Release Date:** 2025-02-22
