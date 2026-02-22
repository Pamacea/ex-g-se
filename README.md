# EX-G-SE v0.3.8

> **AI-powered shadow logging observability for development sessions**

```
.--------------------------------------------------------------.
|  EX-G-SE  v0.3.8  |  GHOST OBSERVABILITY + AI ANALYSIS   |
'--------------------------------------------------------------'
```

## What

EX-G-SE records your development session and automatically generates:
- **Session analysis** with intent detection
- **Theater-play scripts** with code notes and decisions
- **Video assets** for content creation

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
exg              Show help
exg config       Configure AI provider
exg record       Start recording session
exg update       Update to latest version
```

## How It Works

1. **Ghost Mode Recording:**
   - Monitors file system changes
   - Tracks clipboard content
   - Takes screenshots every 30s
   - Listens for keyboard trigger (Ctrl+Shift+X)

2. **Automatic Analysis:**
   - Detects development intents (BugFixing, Feature Development, Refactoring, etc.)
   - Identifies key moments in your session
   - Generates structured outputs

3. **Outputs:**
   - `.ex-g-se/session_analysis.json` - Complete analysis
   - `.ex-g-se/session_script.md` - Theater-play format
   - `.ex-g-se/video_assets/scenes.json` - Video editing timeline

## License

MIT
