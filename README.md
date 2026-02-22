# EX-G-SE v0.2.0

Shadow logging observability tool for Oalacea.

```
.--------------------------------------------------------------.
|  EX-G-SE  v0.2.0  |  GHOST OBSERVABILITY FOR DEVELOPMENT WORKFLOWS  |
'--------------------------------------------------------------'
```

## What

EX-G-SE is a zero-configuration shadow logging tool that captures:
- File system changes
- Clipboard activity
- Screenshots
- Keyboard triggers

## Quick Start

```bash
npx @oalacea/ex-g-se
```

Press `Ctrl+Shift+X` to stop and generate session logs.

## How It Works

```
                    EX-G-SE
      .-----------------------------------.
      |                                   |
      v                                   v
[Ghost Mode]                      [Trigger Key]
  .--------.                         .------.
  | FS Watch|   Clipboard Capture    |Ctrl+X|
  |--------|   [Screenshot]          |------|
  |         |   >---------<           |     |
  '--------'   '---------'            '-----'
      |                                   |
      '-----------> [OUTPUT] <-------------'
                   |
                   v
            raw_logs.json
```

## Philosophy

- **Zero Config**: Just run `npx` and go
- **Brutalist**: ASCII-only, minimal output
- **Privacy First**: Logs stay local until you decide
- **Cross-Platform**: Linux, macOS (Intel/ARM), Windows

## Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux    | x64         | Stable |
| Windows  | x64         | Stable |
| macOS    | Intel       | Stable |
| macOS    | Apple Silicon | Stable |

## Usage

```bash
# Start shadow logging
npx @oalacea/ex-g-se

# Or install globally
npm install -g @oalacea/ex-g-se
ex-g-se
```

### Stopping a Session

Press `Ctrl+Shift+X` or `Ctrl+C` to stop. You'll be prompted to:
- Open logs in Oalacea (browser)
- Call Oalacea API directly (requires `OALACEA_KEY`)
- Save logs to file
- Exit without saving

## Configuration

EX-G-SE requires no configuration. Optional environment variables:

```bash
# API key for direct Oalacea integration
export OALACEA_KEY=your_key_here

# Custom data directory
export EXGSE_DIR=/custom/path
```

## Platform Requirements

### Linux
```bash
sudo apt-get install libx11-dev libxtst-dev libxkbcommon-dev
```

### macOS
Grant Accessibility and Screen Recording permissions to your terminal.

### Windows
No additional requirements.

## Output

Logs are written to `./raw_logs.json` in the current directory:

```json
{
  "start_time": "2025-02-22T10:30:00Z",
  "end_time": "2025-02-22T11:45:00Z",
  "events": [
    {
      "timestamp": "2025-02-22T10:30:15Z",
      "event_type": "fs_change",
      "data": {
        "path": "./src/main.rs",
        "action": "modified"
      }
    },
    {
      "timestamp": "2025-02-22T10:35:22Z",
      "event_type": "clipboard",
      "data": {
        "content": "function newFeature() {...}"
      }
    }
  ]
}
```

## Development

```bash
git clone https://github.com/oalacea/ex-g-se.git
cd ex-g-se

# Install Node dependencies
npm install

# Build Rust binaries
npm run build:rust

# Run locally
npm run dev
```

## License

MIT

---

**Oalacea** | Observability for Development Workflows
