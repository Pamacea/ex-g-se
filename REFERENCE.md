# EX-G-SE Reference

Quick reference for EX-G-SE commands and configuration.

```
.--------------------------------------------------------------.
|                      QUICK REFERENCE                          |
'--------------------------------------------------------------'
```

## Installation

```bash
# Npx (one-time use)
npx @oalacea/ex-g-se

# Global install
npm install -g @oalacea/ex-g-se
ex-g-se
```

## CLI Commands

```bash
# Start shadow logging
ex-g-se

# Stop session
Ctrl+C or Ctrl+Shift+X
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| OALACEA_KEY | API key for Oalacea integration | undefined |
| EXGSE_DIR | Custom directory for logs | ~/.exgse |
| RUST_LOG | Rust logging level | info |

## Platform Matrix

| Platform | Architecture | Binary Name |
|----------|-------------|-------------|
| Linux | x64 | ex-g-se-linux |
| Windows | x64 | ex-g-se-win.exe |
| macOS | Intel | ex-g-se-macos-intel |
| macOS | ARM | ex-g-se-macos-silicon |

## Troubleshooting

### Error: Binary not found
```bash
npm run build:rust
```

### Linux: Clipboard not working
```bash
sudo apt-get install libx11-dev libxtst-dev libxkbcommon-dev
```

### macOS: Permissions required
1. System Preferences > Security & Privacy > Privacy
2. Grant Accessibility to terminal
3. Grant Screen Recording to terminal

---

**Oalacea** | Observability for Development
