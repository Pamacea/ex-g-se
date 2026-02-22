# EX-G-SE CLI Wrapper

Node.js wrapper for EX-G-SE platform binaries. Detects platform, launches the correct binary, and handles post-session workflows.

## Installation

```bash
npm install -g @oalacea/ex-g-se-cli
```

## Usage

```bash
# Start a recording session
ex-g-se

# Skip post-session workflow
ex-g-se --no-post-session

# Show help
ex-g-se --help
```

## Environment Variables

- `EXGSE_LOG_DIR` - Custom log directory (default: `~/.exgse`)
- `OALACEA_KEY` - API key for Oalacea synthesis

## Development

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run in development
node bin/index.js
```

## Binary Location

The wrapper looks for binaries in:

1. `./binaries/` (local development)
2. Package installation `binaries/` directory
3. `~/.exgse/bin/` (user data directory)

## Platform Support

| Platform | Architecture | Binary Name |
|----------|-------------|-------------|
| Linux | x64 | `ex-g-se-linux` |
| Linux | ARM64 | `ex-g-se-linux-arm64` |
| macOS (Intel) | x64 | `ex-g-se-macos-intel` |
| macOS (Apple Silicon) | ARM64 | `ex-g-se-macos-silicon` |
| Windows | x64 | `ex-g-se-win.exe` |
| Windows | ARM64 | `ex-g-se-win-arm64.exe` |
