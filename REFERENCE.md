# EX-G-SE Command Reference v0.5.3

Quick reference for all EX-G-SE commands.

```
.--------------------------------------------------------------.
|              EX-G-SE v0.5.3 - COMMAND REFERENCE             |
'--------------------------------------------------------------'
```

## Quick Command Index

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

## Configuration Commands

### exg config

Launch interactive configuration wizard.

```bash
exg config
```

**Prompts:**
1. Choose provider (1-4):
   - 1. openai (GPT-4, GPT-4o)
   - 2. anthropic (Claude Opus, Sonnet)
   - 3. z.ai
   - 4. custom

2. Enter API key (masked input)

3. Create master password (min 12 characters)

**Config Location:** `~/.config/ex-g-se/settings.enc`

**Encryption:** AES-256-GCM + scrypt

### exg config list

Show current configuration.

```bash
exg config list
```

**Output:**
```
📋 Current Configuration

  Encrypted: Yes
  File: /home/user/.config/ex-g-se/settings.enc
  Size: 512 B
  Created: 2/22/2025, 2:30:15 PM
  Modified: 2/22/2025, 2:30:15 PM

🔐 Enter master password to view details: ••••••••••••

  Provider: openai
  Model: gpt-4o
  API URL: https://api.openai.com/v1
  API Key: ••••••••••••••abcd
```

### exg config test

Test API connection.

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

---

## Recording Commands

### exg record

Start a new recording session.

```bash
exg record
```

**What gets recorded:**
- File system changes (smart filtering)
- Clipboard content (truncated to 500 chars)
- Screenshots (every 30 seconds)
- Global keyboard input (trigger detection only)

**How to stop:**
- Press Ctrl+C
- Press Ctrl+Shift+X

**What happens after stopping:**
1. Session saved to `~/.ex-g-se/sessions/TIMESTAMP_LABEL.json`
2. Session summary displayed
3. Press ENTER to exit

### exg record --label <text>

Record with custom label.

```bash
exg record --label "Fixing bug #123"
```

**Used for:**
- Descriptive session names
- Task identification
- Easier session search

### exg record --tags <tag1,tag2>

Add tags to session.

```bash
exg record --tags bugfix,payment,urgent
```

**Used for:**
- Categorization
- Multi-label sessions
- Search filters

### exg record --duration <minutes>

Auto-stop after N minutes.

```bash
exg record --duration 30
```

**Used for:**
- Time-boxed sessions
- Pomodoro-style work
- Meeting recordings

**Note:** Can still stop early with Ctrl+C

### exg record --max-events <count>

Stop after N events.

```bash
exg record --max-events 1000
```

**Used for:**
- Event-limited sessions
- Testing workflows
- Small task recordings

**Note:** Can still stop early with Ctrl+C

### Combined Options

```bash
# Full example
exg record \
  --label "Refactoring payment module" \
  --tags refactor,payment \
  --duration 60 \
  --max-events 500
```

**During recording:**

```
[CLI] Session label: Refactoring payment module
[CLI] Tags: refactor, payment
[CLI] Auto-stop after: 3600s
[CLI] Max events: 500

▶ Starting Recording Session

  Label: Refactoring payment module
  Tags: refactor, payment
  Auto-stop: 60 minutes
  Max events: 500
  Platform: win32-x64

⚠️  Press Ctrl+Shift+X or Ctrl+C to stop

⏸️  Session will be saved and you can press ENTER to exit
```

**Progress during recording:**

```
[EX-G-SE] Recording... (1m 30s | Events:  45 | FS: 12 | Clips:  3 | Shots:  2)
[EX-G-SE] Recording... (2m  0s | Events:  89 | FS: 25 | Clips:  7 | Shots:  4)
```

---

## Session Management Commands

### exg list

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

**Session ID format:** `YYYY-MM-DD_HH-MM-SS_LABEL`

### exg stats

Show statistics for latest session.

```bash
exg stats
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

### exg stats <session_id>

Show statistics for specific session.

```bash
exg stats 2025-02-22_16-45-22_refactoring
```

**Use case:** Analyze past sessions

### exg search <query>

Search through all sessions.

```bash
exg search "payment"
exg search "bugfix"
exg search "refactor"
```

**Searches in:**
- Session labels
- Session tags
- Event content (file paths, clipboard text, etc.)

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

---

## Export Commands

### exg export json

Export latest session as JSON.

```bash
exg export json
```

**Output:** Full session data as JSON

**Save to file:**
```bash
exg export json > session.json
```

### exg export markdown

Export as Markdown report.

```bash
exg export markdown
```

**Output format:**
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

**Save to file:**
```bash
exg export markdown > report.md
```

### exg export csv

Export as CSV for data analysis.

```bash
exg export csv
```

**Output format:**
```csv
timestamp,type,data
2025-02-22T18:20:15.123Z,fs_change,"{""path"":""src/payment.js"",""action"":""modify""}"
2025-02-22T18:20:30.456Z,clipboard,"{""content"":""payment"",""length"":7}"
```

**Save to file:**
```bash
exg export csv > data.csv
```

**Use with:**
- Excel
- Google Sheets
- Python pandas
- R dataframes
- Any CSV tool

### exg export <format> [session_id]

Export specific session.

```bash
exg export json 2025-02-22_18-20-00_bugfix
exg export markdown 2025-02-22_16-45-22_refactoring > old_report.md
```

---

## Other Commands

### exg update

Update to latest version.

```bash
exg update
```

**What it does:**
- Runs `npm update -g @oalacea/ex-g-se`
- Downloads latest version
- Installs globally

**Output:**
```
🔄 Checking for updates...

✅ Updated successfully!
```

### exg

Show help message.

```bash
exg
```

**Shows:**
- All commands
- Usage examples
- Quick start guide

---

## Event Types

### fs_change

File system change event.

**Fields:**
- `path`: File path
- `action`: create, modify, delete, rename

**Example:**
```json
{
  "path": "src/payment.js",
  "action": "modify"
}
```

**Smart Filtering:** Ignores node_modules, .git, target, dist, .next, coverage, *.log, *.tmp

### clipboard

Clipboard content change event.

**Fields:**
- `content`: Clipboard text (truncated to 500 chars)
- `length`: Full content length

**Example:**
```json
{
  "content": "function calculatePayment(amount) {...",
  "length": 45
}
```

### screenshot

Screenshot capture event.

**Fields:**
- `path`: Screenshot file path
- `width`: Image width in pixels
- `height`: Image height in pixels
- `size`: File size in bytes

**Example:**
```json
{
  "path": "/path/to/screenshot_2025-02-22_18-20-30.png",
  "width": 1920,
  "height": 1080,
  "size": 245678
}
```

**Interval:** Every 30 seconds during recording

---

## File Locations

### Configuration

**Location:** `~/.config/ex-g-se/settings.enc`

**Encrypted:** Yes (AES-256-GCM)

**Contains:**
- Provider
- API URL
- Model
- API key (encrypted)

### Sessions

**Location:** `~/.ex-g-se/sessions/`

**Format:** `TIMESTAMP_LABEL.json`

**Example:** `2025-02-22_18-20-00_bugfix.json`

**Contains:**
- Session metadata (ID, label, tags, timestamps)
- All events (fs_change, clipboard, screenshot, keyboard)
- Event counts and breakdown

### Screenshots

**Location:** Saved with session data

**Format:** PNG

**Naming:** `screenshot_TIMESTAMP.png`

---

## Environment Variables

### EX_G_SE_PROVIDER

AI provider to use.

```bash
export EX_G_SE_PROVIDER=openai
```

**Values:** `openai`, `anthropic`, `z.ai`, or custom

### EX_G_SE_API_KEY

API key for provider.

```bash
export EX_G_SE_API_KEY=sk-...
```

**Priority:** Overrides encrypted config

### EX_G_SE_API_URL

Custom API endpoint.

```bash
export EX_G_SE_API_URL=https://api.openai.com/v1
```

**Optional:** Uses default if not set

### EX_G_SE_MODEL

Model to use.

```bash
export EX_G_SE_MODEL=gpt-4o
```

**Optional:** Uses provider default if not set

---

## Keyboard Shortcuts

### During Recording

- **Ctrl+C**: Stop recording
- **Ctrl+Shift+X**: Manual trigger (currently just logs)

### After Recording

- **ENTER**: Exit and return to terminal

---

## Tips & Tricks

### 1. Use Labels for Organization

```bash
# Good
exg record --label "Fixing payment calculation bug"

# Bad
exg record
```

### 2. Use Tags for Searchability

```bash
exg record --tags bugfix,payment,urgent
exg search "payment"
exg search "bugfix"
```

### 3. Time-Box Your Work

```bash
# 25-minute Pomodoro session
exg record --duration 25 --label "Pomodoro 1: Feature X"
```

### 4. Event-Limited Testing

```bash
# Record first 100 events while testing
exg record --max-events 100 --label "Test run 1"
```

### 5. Export for Documentation

```bash
# Record then export as Markdown
exg record --label "API documentation"
# ... work ...
exg export markdown > API_docs.md
```

### 6. Search Your Work

```bash
# Find all sessions about a topic
exg search "authentication"

# Find all bugfix sessions
exg search "bugfix"
```

### 7. Combine Filters

```bash
# List all sessions, then grep
exg list | grep "payment"
```

### 8. Batch Export

```bash
# Export all sessions as Markdown
for session in $(exg list | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}_[0-9]{2}-[0-9]{2}-[0-9]{2}_[^ ]+'); do
  exg export markdown "$session" > "${session}.md"
done
```

---

## Platform Matrix

| Platform | Architecture | Binary Name | Status |
|----------|-------------|-------------|--------|
| macOS    | Apple Silicon (ARM64) | ex-g-se-macos-silicon | ✅ Stable |
| macOS    | Intel (x64) | ex-g-se-macos-intel | ✅ Stable |
| Windows  | x64 | ex-g-se-win.exe | ✅ Stable |
| Linux    | x64 | ex-g-se-linux | ❌ Unsupported |

---

## Exit Codes

- **0**: Success
- **1**: Error (invalid command, missing config, etc.)

---

## Version Information

```bash
exg --version
```

**Output:** `EX-G-SE v0.5.3`

---

**Version:** 4.0.0
**Last Updated:** 2025-02-22
**Maintainer:** Oalacea
