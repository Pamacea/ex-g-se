# EX-G-SE Guide v4.0.0

Complete guide for EX-G-SE - AI-powered shadow logging observability tool.

```
.--------------------------------------------------------------.
|             EX-G-SE v4.0.0 - COMPLETE GUIDE                 |
'--------------------------------------------------------------'
```

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Quick Start Guide](#quick-start-guide)
3. [Command Reference](#command-reference)
4. [Workflow Examples](#workflow-examples)
5. [Platform-Specific Notes](#platform-specific-notes)
6. [Development Guide](#development-guide)
7. [Troubleshooting](#troubleshooting)

---

## Architecture Overview

EX-G-SE is a hybrid application combining Rust performance with Node.js distribution.

```
                    ┌─────────────────┐
                    │  NPM PACKAGE    │
                    │  @oalacea/     │
                    │   ex-g-se       │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  Node Wrapper   │
                    │  (bin/index.js) │
                    │  - CLI Router   │
                    │  - Config Mgmt  │
                    │  - Session Mgmt │
                    │  - Export       │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  RUST CORE      │
                    │  - FS Watcher   │
                    │  - Clipboard    │
                    │  - Screenshots  │
                    │  - Keyboard     │
                    │  - Session Save │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │ SESSION STORAGE │
                    │ ~/.ex-g-se/     │
                    │ sessions/       │
                    └─────────────────┘
```

## Components

### 1. Rust Core (`/core`)

The heart of EX-G-SE. Handles all system-level monitoring.

**Key modules:**
- `main.rs`: Entry point, orchestrates all watchers, session management
- `cli.rs`: Command-line argument parsing
- `fs_watcher.rs`: File system monitoring via notify
- `clipboard.rs` (in main): Clipboard capture via arboard
- `screenshot.rs`: Screen capture (platform-specific)
- `keyboard.rs` (in main): Global keyboard listener via rdev
- `ai/`: AI provider abstractions (OpenAI, Anthropic, z.ai)
- `analyzer.rs`: Session analysis engine
- `script.rs`: Theater-play script generation
- `video_exporter.rs`: Video assets export

**Features:**
- Async tokio runtime for efficient concurrent operations
- Cross-platform support (macOS, Windows)
- Smart file filtering (ignores node_modules, .git, target, dist, etc.)
- Live progress indicator (stats every 30 seconds)
- Session summary on shutdown
- PRESS ENTER to exit (clear user feedback)

### 2. Node Wrapper (`/bin`)

User-facing CLI that manages the Rust binary.

**Responsibilities:**
- CLI routing (config, record, stats, list, search, export, update)
- Platform and architecture detection
- Binary location and execution
- Configuration management (encryption/decryption)
- Session management (list, search, stats)
- Export functionality (JSON, Markdown, CSV)
- Rich terminal output with chalk colors

### 3. Session Storage

Sessions are stored in `~/.ex-g-se/sessions/` with timestamps:

```
~/.ex-g-se/sessions/
├── 2025-02-22_14-30-15_session.json
├── 2025-02-22_16-45-22_refactoring.json
└── 2025-02-22_18-20-00_bugfix.json
```

Each session file contains:
- `session_id`: Unique identifier
- `start_time`: Session start timestamp
- `end_time`: Session end timestamp
- `label`: Optional session label
- `tags`: Optional session tags
- `events`: Array of all captured events
- Metadata (event counts, breakdown by type)

---

## Quick Start Guide

### Installation

```bash
# Install globally via npm
npm install -g @oalacea/ex-g-se

# Verify installation
exg --version
```

### Initial Configuration

```bash
# Configure AI provider (one-time setup)
exg config
```

**You'll be prompted for:**
1. AI provider choice (OpenAI, Anthropic, z.ai, Custom)
2. API key (masked input)
3. Master password (min 12 characters)
4. Optional: Custom API URL and model

**Alternative: Environment Variables**

```bash
export EX_G_SE_PROVIDER=openai
export EX_G_SE_API_KEY=sk-...
export EX_G_SE_API_URL=https://api.openai.com/v1  # Optional
export EX_G_SE_MODEL=gpt-4o                        # Optional
```

### First Recording

```bash
# Start recording
exg record

# The Rust core will:
# - Monitor file changes (smart filtering)
# - Track clipboard content
# - Capture screenshots every 30s
# - Listen for Ctrl+Shift+X trigger
# - Show live progress every 30s
# - Save session when stopped
# - Display session summary
# - Wait for ENTER to exit
```

**During recording you'll see:**

```
[EX-G-SE] Recording... (1m 30s | Events:  45 | FS: 12 | Clips:  3 | Shots:  2)
```

**After stopping (Ctrl+C or Ctrl+Shift+X):**

```
============================================================
✅ SESSION SAVED SUCCESSFULLY!
============================================================

📁 Session file: C:\Users\Yanis\.ex-g-se\sessions\2025-02-22_14-30-15_session.json
📊 Events captured: 127

📊 Session Summary:
   ⏱️  Duration: 5m 23s
   📁 Total Events: 127
   📋 Clipboard Changes: 8
   📂 File Changes: 95
   🖼️  Screenshots: 10

============================================================

⏸️  Press ENTER to exit...
```

---

## Command Reference

### Configuration Commands

#### `exg config`
Launch interactive configuration wizard.

```bash
exg config
```

**Prompts:**
- Provider selection (1-4)
- API key input (masked)
- Master password creation (min 12 chars)
- Confirmation

#### `exg config list`
Show current configuration details.

```bash
exg config list
```

**Output:**
```
📋 Current Configuration

  Encrypted: Yes
  File: C:\Users\Yanis\.config\ex-g-se\settings.enc
  Size: 512 B
  Created: 2/22/2025, 2:30:15 PM
  Modified: 2/22/2025, 2:30:15 PM

🔐 Enter master password to view details: ••••••••••••

  Provider: openai
  Model: gpt-4o
  API URL: https://api.openai.com/v1
  API Key: ••••••••••••••abcd
```

#### `exg config test`
Test API connection with current configuration.

```bash
exg config test
```

**Output:**
```
🧪 Testing API Connection...

✅ API connection successful!

  Provider: openai
  Model: gpt-4o
```

### Recording Commands

#### `exg record`
Start a new recording session.

```bash
exg record
```

#### `exg record --label <text>`
Record with a custom label.

```bash
exg record --label "Fixing payment bug"
```

#### `exg record --tags <tag1,tag2>`
Record with tags.

```bash
exg record --tags bugfix,payment,urgent
```

#### `exg record --duration <minutes>`
Auto-stop after N minutes.

```bash
exg record --duration 30
```

#### `exg record --max-events <count>`
Stop after N events.

```bash
exg record --max-events 1000
```

#### Combined Options

```bash
exg record --label "API refactoring" --tags refactor,api --duration 60 --max-events 500
```

### Session Management Commands

#### `exg list`
List all recorded sessions.

```bash
exg list
```

**Output:**
```
📚 Sessions:

  2025-02-22_18-20-00_bugfix
    Label: Fixing payment bug
    Tags: bugfix, payment
    Date: 2/22/2025, 6:20:00 PM
    Events: 127

  2025-02-22_16-45-22_refactoring
    Label: (no label)
    Date: 2/22/2025, 4:45:22 PM
    Events: 89

  2025-02-22_14-30-15_session
    Label: (no label)
    Date: 2/22/2025, 2:30:15 PM
    Events: 45
```

#### `exg stats [session_id]`
Show detailed statistics for a session.

```bash
# Latest session
exg stats

# Specific session
exg stats 2025-02-22_18-20-00_bugfix
```

**Output:**
```
📊 Session Statistics

  ID: 2025-02-22_18-20-00_bugfix
  Label: Fixing payment bug
  Tags: bugfix, payment
  Duration: 5m 23s
  Start: 2/22/2025, 6:20:00 PM
  End: 2/22/2025, 6:25:23 PM

  Total Events: 127

  Event Breakdown:
    fs_change: 95
    clipboard: 8
    screenshot: 10
    keyboard: 14

  Disk Usage: 45.2 KB
```

#### `exg search <query>`
Search through all sessions.

```bash
exg search "payment"
exg search "bugfix"
exg search "refactor"
```

**Output:**
```
🔍 Searching for: "payment"

Found 2 session(s):

  2025-02-22_18-20-00_bugfix
    Matched in: label, tags, events
    Label: Fixing payment bug
    Tags: bugfix, payment

  2025-02-22_14-30-15_session
    Matched in: events
    Label: (no label)
```

### Export Commands

#### `exg export json`
Export latest session as JSON.

```bash
exg export json

# Or save to file
exg export json > session.json
```

#### `exg export markdown`
Export as Markdown report.

```bash
exg export markdown > session_report.md
```

**Sample output:**
```markdown
# Session: 2025-02-22_18-20-00_bugfix

**Label:** Fixing payment bug
**Start:** 2/22/2025, 6:20:00 PM
**End:** 2/22/2025, 6:25:23 PM
**Tags:** bugfix, payment

---

## Events

### 1. fs_change
**Time:** 2025-02-22T18:20:15.123Z

```json
{
  "path": "src/payment.js",
  "action": "modify"
}
```

### 2. clipboard
**Time:** 2025-02-22T18:20:30.456Z

```json
{
  "content": "payment calculation fix",
  "length": 24
}
```

...
```

#### `exg export csv`
Export as CSV for data analysis.

```bash
exg export csv > session_data.csv
```

**Sample output:**
```csv
timestamp,type,data
2025-02-22T18:20:15.123Z,fs_change,"{""path"":""src/payment.js"",""action"":""modify""}"
2025-02-22T18:20:30.456Z,clipboard,"{""content"":""payment calculation"",""length"":18}"
```

#### `exg export <format> [session_id]`
Export specific session.

```bash
exg export json 2025-02-22_18-20-00_bugfix
exg export markdown 2025-02-22_16-45-22_refactoring > report.md
```

### Other Commands

#### `exg update`
Update to latest version from npm.

```bash
exg update
```

#### `exg`
Show help message with all commands.

```bash
exg
```

---

## Workflow Examples

### Example 1: Record and Analyze Bug Fix

```bash
# 1. Start recording with label
exg record --label "Fixing payment calculation bug" --tags bugfix,payment

# 2. Work on your fix (make changes, copy code, etc.)

# 3. Press Ctrl+C when done

# 4. Session is automatically saved with stats shown

# 5. View session statistics
exg stats

# 6. Export as report
exg export markdown > bugfix_report.md

# 7. Search for similar sessions later
exg search "payment"
```

### Example 2: Time-Boxed Refactoring Session

```bash
# 1. Start 30-minute refactoring session
exg record --label "Refactor authentication module" --duration 30 --tags refactor

# 2. Work until auto-stop (or stop early with Ctrl+C)

# 3. Review what was accomplished
exg stats

# 4. List all refactoring sessions
exg list | grep refactor
```

### Example 3: Event-Limited Testing Session

```bash
# 1. Record until 100 events captured
exg record --label "Unit test development" --max-events 100 --tags testing

# 2. Write tests until auto-stop

# 3. Export test data for analysis
exg export csv > test_session.csv
```

### Example 4: Learning Session with Notes

```bash
# 1. Record learning session
exg record --label "Learning Rust async" --tags learning,rust

# 2. Take notes, copy code examples, work through tutorials

# 3. Export as Markdown for documentation
exg export markdown > rust_async_notes.md
```

### Example 5: Multi-Session Project Work

```bash
# Session 1: Initial implementation
exg record --label "Feature X implementation" --tags feature-x
# ... work ...

# Session 2: Bug fixes
exg record --label "Fixing Feature X bugs" --tags feature-x,bugfix
# ... work ...

# Session 3: Documentation
exg record --label "Documenting Feature X" --tags feature-x,docs
# ... work ...

# Search all Feature X sessions
exg search "feature-x"

# Export all sessions
exg list | grep feature-x | awk '{print $1}' | while read id; do
  exg export markdown "$id" > "${id}.md"
done
```

---

## Platform-Specific Notes

### macOS

**Requirements:**
- macOS 10.15 (Catalina) or later
- Accessibility permissions for keyboard monitoring
- Screen Recording permissions for screenshots

**Granting Permissions:**
1. System Preferences → Security & Privacy → Privacy
2. Accessibility → Add Terminal (or your shell)
3. Screen Recording → Add Terminal

**Known Issues:**
- First run may require permission grant - restart recording after granting

### Windows

**Requirements:**
- Windows 10 or later
- No special permissions required

**Known Issues:**
- None - works out of the box

### Linux

**Status:** ❌ Unsupported

**Reason:** System dependencies (libudev, wayland, X11) cause build issues.

**Future:** Contributions welcome!

---

## Development Guide

### Project Structure

```
ex-g-se/
├── core/                   # Rust core engine
│   ├── src/
│   │   ├── ai/            # AI provider abstractions
│   │   ├── analyzer.rs    # Session analysis
│   │   ├── cli.rs         # CLI argument parsing
│   │   ├── script.rs      # Script generation
│   │   ├── screenshot.rs  # Screenshot capture
│   │   ├── video_exporter.rs
│   │   ├── fs_watcher.rs  # File system monitoring
│   │   ├── watchers.rs    # Event types
│   │   ├── lib.rs         # Library interface
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tests/             # Integration tests
├── bin/                   # Node.js CLI wrapper
│   ├── index.js           # Main entry point
│   └── config.js          # Configuration wizard
├── scripts/               # Build scripts
│   ├── build-all.sh       # Cross-compilation
│   ├── dev.sh             # Local development
│   └── release.sh         # Release automation
├── .github/workflows/     # CI/CD
│   ├── ci.yml
│   ├── lint.yml
│   └── release.yml
├── package.json           # NPM package config
├── README.md              # User documentation
├── GUIDE.md               # This file
├── REFERENCE.md           # Command reference
├── ARCHITECTURE.md        # Technical architecture
└── CHANGELOG.md           # Version history
```

### Development Workflow

#### Local Development

```bash
# Clone repository
git clone https://github.com/oalacea/ex-g-se.git
cd ex-g-se

# Install dependencies
npm install

# Build Rust core
npm run build:rust

# Run in development mode
npm run dev

# Run tests
npm test

# Lint
npm run lint
```

#### Adding a New AI Provider

1. Create provider file: `core/src/ai/<provider>.rs`
2. Implement `AIProvider` trait
3. Add to `core/src/ai/mod.rs`
4. Update `bin/config.js` provider options
5. Test with `exg config test`

#### Adding a New Intent Type

1. Update `Intent` enum in `core/src/analyzer.rs`
2. Add detection logic in `detectIntentFromEvent()`
3. Add thought generation in `bin/index.js` (if needed)
4. Test with recordings

### Build and Release

#### Cross-Compilation

```bash
# Build for all platforms
npm run build

# This builds:
# - macOS (Apple Silicon & Intel)
# - Windows (x64)
# - Linux (removed in v4.0.0)
```

#### Creating a Release

```bash
# Run release script
npm run release

# This:
# 1. Updates version numbers
# 2. Commits changes
# 3. Creates git tag
# 4. Pushes to GitHub
# 5. Triggers GitHub Actions to build and publish to NPM
```

### Testing

#### Running Tests

```bash
# All tests
npm test

# Rust tests only
npm run test:rust

# Specific test
cd core && cargo test test_name
```

#### Test Coverage

Current coverage: ~85%

- Unit tests: All modules
- Integration tests: Full workflow scenarios
- Mock tests: AI providers, file system operations

---

## Troubleshooting

### Common Issues

#### Issue: "No configuration found"

**Solution:**
```bash
exg config
```

Or set environment variables:
```bash
export EX_G_SE_PROVIDER=openai
export EX_G_SE_API_KEY=sk-...
```

#### Issue: "API key invalid"

**Solution:**
```bash
# Test API connection
exg config test

# Reconfigure if needed
exg config
```

#### Issue: "Permission denied" on macOS

**Solution:**
1. System Preferences → Security & Privacy → Privacy
2. Add Terminal to Accessibility
3. Add Terminal to Screen Recording
4. Restart recording

#### Issue: "Session not found"

**Solution:**
```bash
# List all sessions
exg list

# Use exact session ID
exg stats 2025-02-22_18-20-00_bugfix
```

#### Issue: Recording doesn't stop

**Solution:**
1. Try Ctrl+C
2. Try Ctrl+Shift+X
3. If frozen, kill process: `pkill -f ex-g-se`

#### Issue: No events captured

**Solution:**
```bash
# Check if in correct directory
pwd

# Make sure you're in the project directory you want to monitor

# Start recording again
exg record
```

### Getting Help

- **Documentation:** README.md, GUIDE.md, REFERENCE.md
- **Issues:** https://github.com/oalacea/ex-g-se/issues
- **Changelog:** CHANGELOG.md (version history)

### Debug Mode

Enable debug output:

```bash
# Rust debug logs
RUST_LOG=debug exg record

# Node.js debug
DEBUG=* exg record
```

---

**Version:** 4.0.0
**Last Updated:** 2025-02-22
**Maintainer:** Oalacea
