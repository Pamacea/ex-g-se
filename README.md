# EX-G-SE v0.6.0

> **AI-powered shadow logging observability for development sessions**

```
.--------------------------------------------------------------.
|  EX-G-SE  v0.5.3  |  GHOST OBSERVABILITY + AI ANALYSIS   |
'--------------------------------------------------------------'
```

## What

EX-G-SE records your development session and automatically generates:
- **Session analysis** with intent detection
- **Theater-play scripts** with code notes and decisions
- **Video assets** for content creation
- **Multiple export formats** (JSON, Markdown, CSV)

## Quick Start

```bash
# Install globally
npm install -g @oalacea/ex-g-se

# 1. Configure AI provider (one-time setup)
exg config

# 2. Start recording (press Ctrl+Shift+X to stop)
exg record

# That's it! Analysis is automatic.
```

## 🔐 Security

**Your API keys are NEVER stored in plain text!**

EX-G-SE uses **military-grade encryption**:
- **AES-256-GCM** encryption
- **scrypt** key derivation (memory-hard, resistant to GPU/ASIC attacks)
- **No keys stored on disk** - derived from your master password

**If you lose your master password:** just run `exg config` again!

## Configuration

### Option 1: Interactive Config (Recommended)

```bash
exg config
```

**Prompts:**
1. Choose AI provider (OpenAI, Anthropic, z.ai, custom)
2. Enter API key (masked input)
3. Create master password (min 12 characters)
4. Done! Config saved to `~/.config/ex-g-se/settings.enc`

### Option 2: Environment Variables

```bash
export EX_G_SE_PROVIDER=openai
export EX_G_SE_API_KEY=sk-...
exg record
```

## Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS    | Apple Silicon | ✅ Stable |
| Windows  | x64         | ✅ Stable |
| Linux    | x64         | ❌ Unsupported (system deps: libudev, wayland) |

**Note:** Linux support temporarily disabled. Contributions welcome!

## Commands

```bash
# Configuration
exg config              Configure AI provider
exg config list         Show current configuration
exg config test         Test API connection

# Recording
exg record              Start recording session
exg record --label "Fixing bug #123"
exg record --duration 30                Auto-stop after 30 minutes
exg record --max-events 1000            Stop after 1000 events
exg record --tags bugfix,payment        Add tags to session

# Session Management
exg list                List all sessions
exg stats [session_id]  Show session statistics
exg search <query>      Search through sessions

# Export
exg export json         Export latest session as JSON
exg export markdown     Export as Markdown
exg export csv          Export as CSV

# Other
exg update              Update to latest version
exg                     Show help message
```

## Features

### 📊 Live Progress Indicator

During recording, see live statistics every 30 seconds:

```
[EX-G-SE] Recording... (1m 30s | Events: 45 | FS: 12 | Clips: 3 | Shots: 2)
```

### ✅ Session Summary

Detailed summary when recording stops:

```
✅ SESSION SAVED SUCCESSFULLY!

📊 Session Summary:
   ⏱️  Duration: 5m 23s
   📁 Total Events: 127
   📋 Clipboard Changes: 8
   📂 File Changes: 95
   🖼️  Screenshots: 10

💾 Saved to: raw_logs.json
   Analysis will run automatically...
```

### 🏷️ Labels & Tags

Organize your sessions with labels and tags:

```bash
exg record --label "Refactoring payment module"
exg record --tags bugfix,payment,urgent
```

### ⏱️ Auto-Stop

Automatically stop recording after a duration or event count:

```bash
exg record --duration 30      # Stop after 30 minutes
exg record --max-events 1000  # Stop after 1000 events
```

### 🔍 Search

Search through all your sessions:

```bash
exg search "payment"
exg search "bugfix"
```

### 📤 Export

Export sessions in different formats:

```bash
exg export json       # JSON format
exg export markdown   # Markdown report
exg export csv        # CSV for data analysis
```

### 📋 Stats

View detailed statistics about a session:

```bash
exg stats              # Latest session stats
exg stats 2025-02-22_14-30-15_fixing_bug  # Specific session
```

## Session Storage

Sessions are now stored in `~/.ex-g-se/sessions/` with timestamps:

```
~/.ex-g-se/sessions/
├── 2025-02-22_14-30-15_session.json
├── 2025-02-22_16-45-22_refactoring.json
└── 2025-02-22_18-20-00_bugfix.json
```

Each session includes:
- Start/end timestamps
- Label and tags
- All events (clipboard, file changes, screenshots)
- Metadata (event counts, duration)

## What Gets Recorded

EX-G-SE records (in "Ghost Mode"):

- **File system changes** (smart filtering: ignores node_modules, .git, target, dist, etc.)
- **Clipboard content** (truncated to 500 chars)
- **Screenshots** (every 30 seconds)
- **Global keyboard input** (for Ctrl+Shift+X trigger detection)

## Smart File Filtering

EX-G-SE automatically ignores common noise:

- `node_modules/`, `.git/`, `target/`, `dist/`
- `.next/`, `coverage/`
- `*.log`, `*.tmp` files
- Custom patterns via configuration

## Output Files

After recording stops, EX-G-SE generates:

```
.ex-g-se/
├── session_analysis.json    # Intent detection, key moments
├── session_script.md        # Theater-play format
└── video_assets/
    └── scenes.json          # Video timeline with actions
```

## Examples

### Record a bug fix session

```bash
exg record --label "Fixing payment bug" --tags bugfix,payment --duration 60
```

### Search all bug fix sessions

```bash
exg search "bugfix"
```

### Export session for documentation

```bash
exg export markdown > bugfix_report.md
```

### View session statistics

```bash
exg stats
```

## Development

```bash
# Clone repo
git clone https://github.com/oalacea/ex-g-se.git
cd ex-g-se

# Install dependencies
npm install

# Build Rust core
npm run build:rust

# Run tests
npm test

# Lint
npm run lint

# Development mode
npm run dev
```

## License

MIT

## Support

- **Issues:** https://github.com/oalacea/ex-g-se/issues
- **Docs:** https://github.com/oalacea/ex-g-se/blob/main/README.md

---

**Version:** 4.0.0 | **Maintainer:** Oalacea
