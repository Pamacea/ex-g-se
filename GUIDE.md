# EX-G-SE Guide

Architecture and development workflow for EX-G-SE.

```
.--------------------------------------------------------------.
|                    ARCHITECTURE OVERVIEW                     |
'--------------------------------------------------------------'
```

## Architecture

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
                    │  - Platform det │
                    │  - Binary launch│
                    │  - Post-session │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  RUST CORE      │
                    │  - FS Watcher   │
                    │  - Clipboard    │
                    │  - Screenshots  │
                    │  - Keyboard     │
                    └─────────────────┘
```

## Components

### 1. Rust Core (/core)

The heart of EX-G-SE. Handles all system-level monitoring.

**Key modules:**
- main.rs: Entry point, orchestrates all watchers
- File system monitoring via notify
- Clipboard capture via arboard
- Screen capture via captrs
- Global keyboard listener via device_query

### 2. Node Wrapper (/cli)

User-facing CLI that manages the Rust binary.

**Responsibilities:**
- Detect platform and architecture
- Locate or download correct binary
- Execute with proper error handling
- Post-session workflow (Inquirer prompts)

### 3. Build Scripts (/scripts)

Automation for development and release.

| Script | Purpose |
|--------|---------|
| build-all.sh | Cross-compile for all platforms |
| dev.sh | Build and run locally |
| release.sh | Tag and trigger release |

## Platform-Specific Notes

### Linux
- Requires X11 libraries
- libx11-dev, libxtst-dev, libxkbcommon-dev

### macOS
- Requires Accessibility permissions
- Requires Screen Recording permissions

### Windows
- Works out of the box

---

**Oalacea** | Shadow Logging for Developers
