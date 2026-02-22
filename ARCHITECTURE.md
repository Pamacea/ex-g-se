# EX-G-SE Architecture

Complete architectural overview of the EX-G-SE shadow logging system.

```
.--------------------------------------------------------------.
|              EX-G-SE - GHOST OBSERVABILITY ENGINE            |
|                                                              |
|  Shadow logging for development workflows                    |
'--------------------------------------------------------------'
```

## System Overview

EX-G-SE is a **hybrid Rust/Node.js application** designed for zero-configuration shadow logging.

## Technology Stack

### Rust Core

| Component | Crate | Purpose |
|-----------|-------|---------|
| Runtime | tokio | Async runtime |
| FS Watching | notify | File system events |
| Clipboard | arboard | Clipboard access |
| Screenshots | captrs | Screen capture |
| Keyboard | device_query | Global hooks |
| Serialization | serde_json | JSON output |

### Platform Matrix

| Target | Triple | Binary Name |
|--------|--------|-------------|
| Linux x64 | x86_64-unknown-linux-gnu | ex-g-se-linux |
| Windows x64 | x86_64-pc-windows-msvc | ex-g-se-win.exe |
| macOS Intel | x86_64-apple-darwin | ex-g-se-macos-intel |
| macOS ARM | aarch64-apple-darwin | ex-g-se-macos-silicon |

## Data Flow

1. User runs: `npx @oalacea/ex-g-se`
2. Node detects platform → Spawns Rust binary
3. Rust monitors FS, clipboard, screenshots, keyboard
4. User triggers Ctrl+C → Graceful shutdown
5. Writes raw_logs.json
6. Node prompts for next action

## Platform Requirements

**Linux:** `sudo apt-get install libx11-dev libxtst-dev libxkbcommon-dev`
**macOS:** Grant Accessibility + Screen Recording to terminal
**Windows:** No additional requirements

---

**Oalacea** | Observability for Development Workflows
