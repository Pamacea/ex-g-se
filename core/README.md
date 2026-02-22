# EX-G-SE Core Engine v0.3.0

The Rust core engine that runs in "Ghost Mode" for shadow logging.

## Platform Requirements

### Linux
```bash
# Debian/Ubuntu
sudo apt-get install libx11-dev libxtst-dev libxrandr-dev

# Fedora/RHEL
sudo dnf install libX11-devel libXtst-devel libXrandr-devel

# Arch
sudo pacman -S libx11 libxtst libxrandr
```

### macOS
- Requires Accessibility permissions for keyboard hooks
- Requires Screen Recording permissions for screenshots

### Windows
- Works out of the box (no additional dependencies required)

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run

# Run release binary
cargo run --release
```

## Usage

The core engine monitors:

1. **File System Changes** - Watches current directory recursively
2. **Clipboard** - Detects clipboard content changes
3. **Screenshots** - Captures at 30-second intervals (optional)
4. **Keyboard Triggers** - Listens for Ctrl+Shift+X

Press `Ctrl+C` to stop and save logs to `raw_logs.json`.

## Output Format

```json
{
  "start": "2026-02-22T14:30:00Z",
  "end": "2026-02-22T14:35:00Z",
  "events": [
    {
      "ts": "2026-02-22T14:30:05Z",
      "type": "fs_change",
      "data": {
        "path": "/path/to/file",
        "kind": "Modify(SystemTime)"
      }
    },
    {
      "ts": "2026-02-22T14:30:10Z",
      "type": "clipboard",
      "data": {
        "content": "clipboard text",
        "length": 15
      }
    }
  ]
}
```

## Development

Run tests:
```bash
cargo test
```

Run with logging:
```bash
RUST_LOG=debug cargo run
```
