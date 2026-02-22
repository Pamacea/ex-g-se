# EX-G-SE v0.3.2

> **AI-powered shadow logging observability for development sessions**

```
.--------------------------------------------------------------.
|  EX-G-SE  v0.3.2  |  GHOST OBSERVABILITY + AI ANALYSIS       |
'--------------------------------------------------------------'
```

## What

EX-G-SE records your development session and automatically generates:
- **Session analysis** with intent detection
- **Theater-play scripts** with code notes and decisions
- **Video assets** for content creation

## Quick Start

```bash
# 1. Configure AI provider (one-time setup)
npx @oalacea/ex-g-se config

# 2. Start recording (press Ctrl+Shift+X to stop)
npx @oalacea/ex-g-se

# That's it! Analysis is automatic.
```

## 🔐 Security

**Your API keys are NEVER stored in plain text!**

EX-G-SE uses **military-grade encryption**:
- **AES-256-GCM** encryption
- **scrypt** key derivation (memory-hard, resistant to GPU/ASIC attacks)
- **No keys stored on disk** - derived from your master password

**If you lose your master password:** just run `config` again!

## Configuration

### Option 1: Interactive Config (Recommended)

```bash
npx @oalacea/ex-g-se config
```

**Prompts:**
1. Choose AI provider (OpenAI, Anthropic, z.ai, custom)
2. Enter your API key
3. Enter API URL (with defaults)
4. Enter model (with defaults)
5. **Create a master password** (min 12 characters)

**Your master password encrypts your config with AES-256-GCM.**

### Option 2: Environment Variables (for CI/CD)

```bash
export EX_G_SE_PROVIDER=openai
export EX_G_SE_API_KEY=sk-...
npx @oalacea/ex-g-se
```

## How It Works

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Ghost     │────▶│   Raw Logs   │────▶│  Auto Analysis  │
│   Mode      │     │   .json      │     │  + Scripts      │
└─────────────┘     └──────────────┘     └─────────────────┘
      ▲                                       │
      │                                       ▼
      └───────────────────────────────┬─────────────────┐
         Press Ctrl+Shift+X            │  Output Files   │
         (stops recording)            │  .ex-g-se/       │
                                     │  • analysis.json │
                                     │  • script.md     │
                                     │  • scenes.json   │
                                     └─────────────────┘
```

## Usage

### Configure

```bash
npx @oalacea/ex-g-se config
```

**What happens:**
1. Choose your AI provider
2. Enter your API key
3. Create a **master password** (min 12 chars)
4. Config is encrypted with AES-256-GCM
5. Saved to `~/.config/ex-g-se/settings.enc`

**⚠️ IMPORTANT:** Memorize your master password! If you lose it, just run `config` again.

### Record & Analyze

```bash
npx @oalacea/ex-g-se
```

**What happens:**
1. Prompts for master password
2. Decrypts your config
3. Starts ghost mode recording
4. Press `Ctrl+Shift+X` to stop
5. **Automatically analyzes** the session
6. **Automatically generates** scripts and video assets

## Output

```
.ex-g-se/
├── session_analysis.json    # Detected intents, key moments
├── session_script.md        # Theater-play format script
└── video_assets/
    └── scenes.json          # Timeline for video generation
```

## Intent Detection

EX-G-SE detects 8 intent types:

| Intent | Description |
|--------|-------------|
| **Feature Development** | Creating new functionality |
| **Bug Fixing** | Debugging and fixing errors |
| **Refactoring** | Improving existing code |
| **Testing** | Writing/running tests |
| **Documentation** | Writing docs/comments |
| **Learning** | Exploring new code/libs |
| **Deployment** | Publishing/releasing |
| **Configuration** | Setup and config |

## Script Format

Generated scripts use **theater-play format**:

```markdown
# Development Session - Feb 22, 2025

## ACT I - The Investigation

### Scene 1: The Error Appears
**Timestamp**: 2025-02-22T10:32:15Z
**Description**: User encountered null reference error

**Dialogue**:
> **NARRATOR**: At this moment, the developer discovered the bug...
> **DEVELOPER**: "Hmm, this isn't working. Let me debug this issue..."

**Code Notes**:
#### src/auth/login.js:42
**Decision**: Use optional chaining
**Rationale**: Prevents crashes, handles expiry gracefully
```

## Video Assets

The `scenes.json` file contains timeline data for video generation:

```json
{
  "timestamp": "2025-02-22T10:32:15Z",
  "duration_seconds": 30,
  "title": "The Discovery",
  "actions": [
    {"type": "highlight", "target": "line 42", "duration": 3},
    {"type": "typewriter", "text": "user?.id", "duration": 2},
    {"type": "fade_out", "duration": 1}
  ],
  "voiceover": "At this moment, the developer discovered..."
}
```

## Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS    | Apple Silicon | ✅ Stable |
| Windows  | x64         | ✅ Stable |
| Linux    | x64         | ❌ Unsupported (requires system deps: libudev, wayland) |

**Note:** Linux support is temporarily disabled. Contributions welcome!

## Requirements

### macOS
Grant Accessibility and Screen Recording permissions to your terminal.

### Windows
No additional requirements.

## Security Details

**Encryption:**
- Algorithm: AES-256-GCM (military grade)
- Key Derivation: scrypt (memory-hard)
- Memory Cost: 64 MB
- Time Cost: 3 iterations
- Salt: 128 bits (random per encryption)

**Security Level:** ⭐⭐⭐⭐⭐

**Attack Resistance:**
- Brute force on 12-char password: ~10^14 years
- Memory-hard (resistant to GPU/ASIC)
- No keys stored on disk

## Examples

```bash
# First time setup
npx @oalacea/ex-g-se config

# Start recording
npx @oalacea/ex-g-se
# ... work on your project ...
# Press Ctrl+Shift+X to stop

# View results
cat .ex-g-se/session_script.md
```

## Troubleshooting

**"Mot de passe incorrect"**
- You entered the wrong master password
- If you forgot it: run `npx @oalacea/ex-g-se config` again

**"Aucune configuration trouvée"**
- Run `npx @oalacea/ex-g-se config` first
- Or set environment variables

**"Aucune donnée de session trouvée"**
- The Rust binary didn't create `raw_logs.json`
- Check if you have the correct binary for your platform

## License

MIT

---

**Oalacea** | Observability for Development Workflows
