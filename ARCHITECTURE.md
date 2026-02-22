# EX-G-SE Architecture v4.0.0

Complete architectural overview of the EX-G-SE shadow logging system.

```
.--------------------------------------------------------------.
|           EX-G-SE v4.0.0 - GHOST OBSERVABILITY ENGINE       |
|                                                              |
|        AI-Powered Shadow Logging for Developers             |
'--------------------------------------------------------------'
```

## Table of Contents

1. [System Overview](#system-overview)
2. [Technology Stack](#technology-stack)
3. [Architecture Diagram](#architecture-diagram)
4. [Component Details](#component-details)
5. [Data Flow](#data-flow)
6. [Security Architecture](#security-architecture)
7. [Session Storage](#session-storage)
8. [Platform Support](#platform-support)
9. [Performance Considerations](#performance-considerations)

---

## System Overview

EX-G-SE is a **hybrid Rust/Node.js application** designed for zero-configuration shadow logging with AI-powered session analysis.

**Key Design Principles:**
1. **Performance First** - Rust core for system-level monitoring
2. **Zero Config** - Works out of the box with sensible defaults
3. **Secure by Default** - AES-256-GCM encryption for credentials
4. **Cross-Platform** - macOS and Windows support (Linux removed in v4.0.0)
5. **Privacy Respecting** - All data stored locally

---

## Technology Stack

### Rust Core (v4.0.0)

| Component | Crate | Version | Purpose |
|-----------|-------|---------|---------|
| **Runtime** | tokio | 1.x | Async runtime, full features |
| **Error Handling** | anyhow | 1.x | Error propagation |
| | thiserror | 1.x | Custom error types |
| **Serialization** | serde | 1.x | Serialization framework |
| | serde_json | 1.x | JSON output |
| **Time** | chrono | 0.4 | Timestamps, duration |
| **FS Watching** | notify | 6.x | File system events |
| **Clipboard** | arboard | 3.x | Clipboard access |
| **Screenshots** | image | 0.25 | Image processing |
| | core-graphics | 0.23 | macOS screenshots |
| | winapi | 0.3 | Windows screenshots |
| **Keyboard** | rdev | 0.6 (git) | Global keyboard hooks |
| **HTTP** | reqwest | 0.12 | AI provider API calls |
| **Async** | async-trait | 0.1 | Async trait support |
| **UUID** | uuid | 1.6 | Session IDs |
| **Home Dir** | dirs | 5.x | Platform paths |

### Node.js Wrapper (v4.0.0)

| Component | Package | Version | Purpose |
|-----------|---------|---------|---------|
| **CLI** | commander | 12.x | Command parsing |
| **Colors** | chalk | 5.x | Terminal colors |
| **Crypto** | crypto | builtin | AES-256-GCM encryption |
| | scrypt-sync | builtin | Key derivation |

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER INTERFACE                          │
│                                                                   │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐  │
│  │  exg config    │  │  exg record    │  │  exg stats     │  │
│  │  exg list      │  │  exg search    │  │  exg export    │  │
│  └────────┬───────┘  └────────┬───────┘  └────────┬───────┘  │
└───────────┼──────────────────┼──────────────────┼─────────────┘
            │                  │                  │
            ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                      NODE.JS WRAPPER                             │
│                      (bin/index.js)                             │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ CLI Router                                              │     │
│  │  - Parse commands                                       │     │
│  │  - Route to handlers                                    │     │
│  └────────┬───────────────────────────────────────────────┘     │
│                                                                   │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐   │
│  │ Config Manager │  │ Session Mgr    │  │ Export Engine  │   │
│  │  - Encryption  │  │  - List        │  │  - JSON        │   │
│  │  - Decryption  │  │  - Search      │  │  - Markdown    │   │
│  │  - Validation  │  │  - Stats       │  │  - CSV         │   │
│  └────────────────┘  └────────────────┘  └────────────────┘   │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Binary Launcher                                        │     │
│  │  - Platform detection                                  │     │
│  │  - Binary location                                     │     │
│  │  - Process execution                                   │     │
│  └────────┬───────────────────────────────────────────────┘     │
└───────────┼─────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────┐
│                        RUST CORE ENGINE                          │
│                       (core/src/main.rs)                        │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ ExGSeEngine                                             │     │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │     │
│  │  │ CLI Config   │  │ Event Store  │  │ Filters      │  │     │
│  │  │  - --label   │  │  - Vec<Event>│  │  - Smart list │  │     │
│  │  │  - --tags    │  │              │  │              │  │     │
│  │  │  - --duration│  │              │  │              │  │     │
│  │  │  - --max-evt │  │              │  │              │  │     │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Watchers (Async)                                        │     │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐       │     │
│  │  │ FS Watcher │  │ Clipboard  │  │Screenshot  │       │     │
│  │  │            │  │            │  │            │       │     │
│  │  │ notify::6  │  │ arboard::3 │  │ Platform    │       │     │
│  │  └────────────┘  └────────────┘  └────────────┘       │     │
│  │                                                       │     │
│  │  ┌────────────┐                                        │     │
│  │  │ Keyboard   │                                        │     │
│  │  │            │                                        │     │
│  │  │ rdev::0.6  │                                        │     │
│  │  └────────────┘                                        │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Progress & Summary                                      │     │
│  │  - Live stats every 30s                                 │     │
│  │  - Session summary on shutdown                          │     │
│  │  - Press ENTER to exit                                  │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐     │
│  │ Session Manager                                         │     │
│  │  - Save to ~/.ex-g-se/sessions/                         │     │
│  │  - Timestamped filenames                               │     │
│  │  - Metadata (label, tags)                               │     │
│  └────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────┐
│                      SESSION STORAGE                             │
│                     (~/.ex-g-se/sessions/)                       │
│                                                                   │
│  2025-02-22_14-30-15_session.json                               │
│  2025-02-22_16-45-22_refactoring.json                           │
│  2025-02-22_18-20-00_bugfix.json                                │
│                                                                   │
│  Each file contains:                                             │
│  - session_id, label, tags                                       │
│  - start_time, end_time                                         │
│  - events[] (fs_change, clipboard, screenshot, keyboard)        │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Details

### Rust Core Modules

#### main.rs
**Responsibilities:**
- Application entry point
- ExGSeEngine orchestration
- Session lifecycle management
- Progress tracking (every 30s)
- Session summary generation
- User exit handling (ENTER to exit)

**Key Functions:**
- `ExGSeEngine::new()` - Initialize with CLI config
- `ExGSeEngine::run()` - Main async run loop
- `ExGSeEngine::save_logs()` - Save to sessions directory
- `format_duration()` - Human-readable time format

#### cli.rs (NEW in v4.0.0)
**Responsibilities:**
- Command-line argument parsing
- CLI configuration struct

**Supports:**
- `--label <text>` - Session label
- `--tags <tag1,tag2>` - Session tags
- `--duration <minutes>` - Auto-stop timer
- `--max-events <count>` - Event limit

**Key Functions:**
- `CliConfig::from_args()` - Parse CLI args
- `CliConfig::should_stop()` - Check limits

#### fs_watcher.rs
**Responsibilities:**
- File system monitoring
- Smart filtering (node_modules, .git, target, dist, .next, coverage, *.log, *.tmp)

**Implementation:**
- Uses `notify` crate (platform-specific)
- macOS: `macos_fsevent` feature only
- Windows: default features

#### clipboard (integrated in main.rs)
**Responsibilities:**
- Clipboard content monitoring
- Content truncation (500 chars max)

**Implementation:**
- Uses `arboard` crate
- Polls every 2 seconds
- Detects content changes

#### screenshot.rs
**Responsibilities:**
- Screenshot capture
- Image metadata extraction

**Implementation:**
- Platform-specific implementations
- Windows: Win32 API
- macOS: Core Graphics
- Saves as PNG

#### keyboard (integrated in main.rs)
**Responsibilities:**
- Global keyboard hook
- Trigger detection (Ctrl+Shift+X)
- Graceful shutdown (Ctrl+C)

**Implementation:**
- Uses `rdev` crate (from GitHub, v0.6.0)
- Event-driven callbacks
- Cross-platform key detection

#### ai/ (AI Provider Abstractions)
**Modules:**
- `mod.rs` - AIProvider trait, factory
- `openai.rs` - OpenAI implementation
- `anthropic.rs` - Anthropic implementation
- `z_ai.rs` - z.ai implementation
- `types.rs` - Shared types (Intent, KeyMoment, Script, etc.)
- `error.rs` - AI error types

**Capabilities:**
- Session analysis
- Intent detection
- Script generation
- Video assets export

#### analyzer.rs
**Responsibilities:**
- Intent detection from events
- Key moment identification
- Session analysis generation

#### script.rs
**Responsibilities:**
- Theater-play script generation
- Markdown export
- Dialogue creation

#### video_exporter.rs
**Responsibilities:**
- Video timeline generation
- Scene definition
- Voiceover text generation
- Action sequences

### Node.js Wrapper Modules

#### index.js (Main Entry Point)
**Responsibilities:**
- CLI routing and command parsing
- Binary launcher
- Session management (list, search, stats)
- Export functionality (JSON, Markdown, CSV)
- Rich terminal output with chalk colors

**Commands:**
- `exg config` → Routes to config.js
- `exg record` → Launches Rust binary
- `exg list` → Lists sessions
- `exg stats` → Shows statistics
- `exg search` → Searches sessions
- `exg export` → Exports session
- `exg update` → Updates package

#### config.js (Configuration Wizard)
**Responsibilities:**
- Interactive configuration
- Provider selection
- API key input (masked)
- Master password creation
- Encryption and storage

**Encryption:**
- AES-256-GCM via Node.js crypto
- scrypt key derivation
- Saves to `~/.config/ex-g-se/settings.enc`

---

## Data Flow

### Recording Flow

```
User runs: exg record --label "Bug fix" --tags bugfix

1. Node.js wrapper:
   - Parses CLI arguments
   - Loads config (encrypted or env vars)
   - Detects platform/arch
   - Locates Rust binary
   - Displays session info
   - Launches Rust binary

2. Rust core:
   - Parses CLI args (--label, --tags, --duration, --max-events)
   - Initializes ExGSeEngine
   - Starts async watchers:
     * FS watcher (smart filtering)
     * Clipboard monitor (2s polling)
     * Screenshot capture (30s interval)
     * Keyboard hook (Ctrl+C, Ctrl+Shift+X)

3. During recording:
   - Events collected in memory
   - Progress stats shown every 30s
   - CLI limits checked (--duration, --max-events)
   - User can stop with Ctrl+C

4. On shutdown:
   - Session saved to: ~/.ex-g-se/sessions/TIMESTAMP_LABEL.json
   - Session summary displayed
   - User presses ENTER to exit

5. Back to Node.js:
   - Shows "Recording complete" message
   - Displays help for next commands (list, stats, export)
```

### Configuration Flow

```
User runs: exg config

1. Interactive prompts:
   - Provider selection (1-4)
   - API key input (masked)
   - Master password creation (min 12 chars)
   - Optional: Custom API URL and model

2. Encryption:
   - Generate random salt (128 bits)
   - Derive key from password (scrypt, N=65536, r=4, p=4)
   - Encrypt config with AES-256-GCM
   - Save to: ~/.config/ex-g-se/settings.enc

3. Usage:
   - Node.js loads encrypted config
   - Prompts for master password
   - Derives key, decrypts
   - Passes to Rust binary via env vars
```

### Session Query Flow

```
User runs: exg list

1. Node.js wrapper:
   - Reads ~/.ex-g-se/sessions/ directory
   - Lists all .json files
   - Parses metadata from each file
   - Displays formatted list

User runs: exg search "payment"

2. Node.js wrapper:
   - Loads all session files
   - Searches in: labels, tags, event content
   - Displays matching sessions with context

User runs: exg stats

3. Node.js wrapper:
   - Finds latest session (or specific ID)
   - Loads session file
   - Calculates statistics
   - Displays detailed breakdown
```

### Export Flow

```
User runs: exg export markdown

1. Node.js wrapper:
   - Finds session (latest or specific)
   - Loads session JSON
   - Formats according to export type:
     * JSON: Raw JSON output
     * Markdown: Formatted report with sections
     * CSV: Comma-separated values
   - Outputs to stdout or file
```

---

## Security Architecture

### Encryption (v4.0.0)

**Algorithm:** AES-256-GCM (Galois/Counter Mode)

**Key Derivation:** scrypt
- Memory cost: 64 MB (N = 2^16)
- Time cost: 3 iterations
- Parallelism: 4 threads
- Key length: 256 bits
- Salt length: 128 bits (random per encryption)

**Security Level:** ⭐⭐⭐⭐⭐
~10^14 years to brute force (current technology)

**Process:**
```
1. User creates master password (min 12 chars)
2. Random salt generated (128 bits)
3. Key derived: scrypt(password, salt, N=65536, r=4, p=4, keyLen=32)
4. Config encrypted: AES-256-GCM(key, iv, plaintext)
5. Saved to: ~/.config/ex-g-se/settings.enc
```

**Decryption:**
```
1. User enters master password
2. Read salt and IV from file
3. Derive key: scrypt(password, salt, ...)
4. Decrypt: AES-256-GCM(key, iv, ciphertext)
5. Parse JSON config
```

**Password Recovery:**
- If password lost: Simply run `exg config` again
- Old encrypted file will be overwritten
- No password recovery mechanism (by design)

### API Key Storage

**Never stored in plain text!**

**Options:**
1. **Encrypted file** (default)
   - ~/.config/ex-g-se/settings.enc
   - AES-256-GCM encrypted
   - Master password protected

2. **Environment variables** (CI/CD)
   - EX_G_SE_PROVIDER
   - EX_G_SE_API_KEY
   - EX_G_SE_API_URL (optional)
   - EX_G_SE_MODEL (optional)

### Data Privacy

**All data stored locally:**
- Sessions: ~/.ex-g-se/sessions/
- Config: ~/.config/ex-g-se/settings.enc
- No cloud sync
- No telemetry
- No phone home

**Event content:**
- File paths (local)
- Clipboard text (truncated to 500 chars)
- Screenshots (local files only)
- No code sent to external servers

---

## Session Storage

### File Structure

```
~/.ex-g-se/
├── sessions/
│   ├── 2025-02-22_14-30-15_session.json
│   ├── 2025-02-22_16-45-22_refactoring.json
│   └── 2025-02-22_18-20-00_bugfix.json
└── settings.enc (config)
```

### Session JSON Schema

```json
{
  "session_id": "2025-02-22_18-20-00_bugfix",
  "label": "Fixing payment bug",
  "tags": ["bugfix", "payment"],
  "start_time": "2025-02-22T18:20:00.123Z",
  "end_time": "2025-02-22T18:25:23.456Z",
  "events": [
    {
      "timestamp": "2025-02-22T18:20:15.123Z",
      "type": "fs_change",
      "data": {
        "path": "src/payment.js",
        "action": "modify"
      }
    },
    {
      "timestamp": "2025-02-22T18:20:30.456Z",
      "type": "clipboard",
      "data": {
        "content": "payment calculation fix",
        "length": 24
      }
    },
    {
      "timestamp": "2025-02-22T18:20:45.789Z",
      "type": "screenshot",
      "data": {
        "path": "/path/to/screenshot.png",
        "width": 1920,
        "height": 1080,
        "size": 245678
      }
    }
  ]
}
```

### Filenames

**Format:** `YYYY-MM-DD_HH-MM-SS_LABEL.json`

**Examples:**
- `2025-02-22_14-30-15_session.json` (no label)
- `2025-02-22_16-45-22_refactoring.json` (single word label)
- `2025-02-22_18-20-00_Fixing_payment_bug.json` (multi-word label, spaces → underscores)

### Smart File Filtering

**Ignored patterns:**
- Directories: `node_modules`, `.git`, `target`, `dist`, `.next`, `coverage`, `.ex-g-se`
- Extensions: `*.log`, `*.tmp`

**Rationale:**
- Reduce noise in event logs
- Focus on actual work files
- Improve performance (fewer events)

---

## Platform Support

### Platform Matrix (v4.0.0)

| Platform | Architecture | Triple | Binary Name | Status |
|----------|-------------|--------|-------------|--------|
| macOS | Apple Silicon (ARM64) | aarch64-apple-darwin | ex-g-se-macos-silicon | ✅ Stable |
| macOS | Intel (x64) | x86_64-apple-darwin | ex-g-se-macos-intel | ✅ Stable |
| Windows | x64 | x86_64-pc-windows-msvc | ex-g-se-win.exe | ✅ Stable |
| Linux | x64 | x86_64-unknown-linux-gnu | ex-g-se-linux | ❌ Unsupported |

### Platform-Specific Requirements

#### macOS

**Requirements:**
- macOS 10.15 (Catalina) or later
- **Permissions Required:**
  - Accessibility (for keyboard monitoring)
  - Screen Recording (for screenshots)

**Granting Permissions:**
1. System Preferences → Security & Privacy → Privacy
2. Accessibility → Add Terminal (or your shell)
3. Screen Recording → Add Terminal (or your shell)
4. Restart recording after granting

**Dependencies:**
- `notify` with `macos_fsevent` feature (no X11 dependency)
- `core-graphics` for screenshots
- `objc` and `cocoa` for native APIs

#### Windows

**Requirements:**
- Windows 10 or later
- **No additional permissions required**

**Dependencies:**
- `notify` with default features
- `winapi` for screenshots (user32, gdi32)
- Works out of the box

#### Linux (Unsupported in v4.0.0)

**Reason for Removal:**
- System dependencies: libudev, wayland, X11
- Build failures across different distributions
- Inconsistent behavior

**Previous Issues:**
- `device_query` → X11 dependency
- `rdev` → wayland/libudev requirement
- `notify` → Various backend issues

**Future:**
- Contributions welcome
- Need maintainers for different distros
- Containerized solution possible

---

## Performance Considerations

### Memory Usage

**Typical session (1 hour, ~500 events):**
- Rust binary: ~5-10 MB RSS
- Event storage: ~1-2 MB in memory
- Total: < 20 MB

**Large session (8 hours, ~5000 events):**
- Event storage: ~10-20 MB in memory
- Total: < 50 MB

### CPU Usage

**Idle (monitoring):**
- FS watcher: < 0.1% CPU
- Clipboard: < 0.1% CPU (polling every 2s)
- Screenshot: ~0.5% CPU every 30s
- Keyboard: < 0.1% CPU (event-driven)
- **Total idle:** < 1% CPU

**Active (file changes):**
- Burst CPU: ~2-5% during file operations
- Average: < 2% CPU

### Disk Usage

**Per session:**
- JSON session file: ~100 KB - 1 MB (depending on events)
- Screenshots: ~200-500 KB each (30s interval)
- Total (1 hour): ~5-10 MB

**Scaling:**
- 100 sessions: ~500 MB - 1 GB
- 1000 sessions: ~5-10 GB

**Recommendations:**
- Archive old sessions periodically
- Export to compressed formats
- Delete unnecessary sessions

### Network Usage

**Recording:** 0 KB (offline)
**AI Analysis:** On-demand (not automatic in v4.0.0)

---

## Extension Points

### Adding a New AI Provider

1. Create `core/src/ai/<provider>.rs`
2. Implement `AIProvider` trait
3. Add to `core/src/ai/mod.rs`
4. Update `bin/config.js` provider list
5. Test with `exg config test`

### Adding a New Intent Type

1. Update `Intent` enum in `core/src/ai/types.rs`
2. Add detection logic in `core/src/analyzer.rs`
3. Add thought in `bin/index.js` generateThought()
4. Test with recordings

### Adding a New Export Format

1. Add format option in `bin/index.js`
2. Implement formatter function
3. Update help text
4. Test with `exg export <format>`

---

## Build System

### Cross-Compilation

**GitHub Actions** (`.github/workflows/`):
- `ci.yml` - Continuous integration
- `lint.yml` - Code quality checks
- `release.yml` - Release automation

**Build Matrix:**
- macOS (Intel & ARM)
- Windows (x64)
- Linux (removed in v4.0.0)

### Release Process

1. Update version in:
   - `core/Cargo.toml`
   - `package.json`
   - `CHANGELOG.md`
2. Commit with Git Flow Master format
3. Push to main
4. Create git tag
5. GitHub Actions builds binaries
6. Publishes to NPM

---

## Version History

**v4.0.0** (2025-02-22)
- Major feature release
- Session management (list, search, stats)
- Export formats (JSON, Markdown, CSV)
- CLI options (--label, --tags, --duration, --max-events)
- Rich terminal output
- Better file filtering
- Config validation
- Press ENTER to exit

**v0.3.8** (2025-02-22)
- Fixed stdin raw mode conflict
- Improved password input

**v0.3.0** (2025-02-22)
- AI-powered session analysis
- Encrypted configuration
- Script generation
- Video assets export

---

**Version:** 4.0.0
**Last Updated:** 2025-02-22
**Maintainer:** Oalacea
