# CLAUDE.md - EX-G-SE Project Configuration

> **Last Updated:** 2025-02-22

---

## 🎯 Project Identity

**Name:** EX-G-SE (Ex-Ghost Observability - Shadow Edition)

**Description:** AI-powered shadow logging observability tool for development sessions

**Tech Stack:**
- **Core:** Rust (Tokio async runtime)
- **CLI:** Node.js (cross-platform wrapper)
- **Encryption:** AES-256-GCM + scrypt (military-grade)
- **AI Providers:** OpenAI, Anthropic, z.ai (extensible)

**Purpose:** Record development sessions and automatically generate:
- Session analysis with intent detection
- Theater-play format scripts
- Video assets for content creation

---

## 📁 Project Structure

```
ex-g-se/
├── core/                   # Rust core engine
│   ├── src/
│   │   ├── ai/            # AI provider system
│   │   ├── analyzer.rs    # Session analysis engine
│   │   ├── script.rs      # Script generator
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tests/             # Integration tests
├── bin/                   # Node.js CLI wrapper
│   ├── index.js           # Main entry (recording + auto-analysis)
│   └── config.js          # Config with encryption
├── .github/workflows/     # CI/CD
│   ├── release.yml        # Release & NPM publish
│   ├── ci.yml             # Continuous integration
│   └── lint.yml           # Linting
├── package.json           # NPM package config
└── README.md              # User documentation
```

---

## 🚀 Quick Commands

### Development
```bash
# Run locally
npm run dev

# Build Rust
npm run build:rust

# Run tests
npm test

# Lint
npm run lint
```

### Release
```bash
# Create release
npm run release
```

---

## 🔧 Key Files

### Core Rust Engine
- **`core/src/main.rs`** - Main entry point with ghost mode
- **`core/src/analyzer.rs`** - Intent detection and analysis
- **`core/src/script.rs`** - Theater-play script generation
- **`core/src/ai/`** - AI provider abstractions

### CLI Wrapper
- **`bin/index.js`** - Main CLI (decrypts config, runs binary, analyzes)
- **`bin/config.js`** - Config command (encrypts and saves)

### Configuration
- **`~/.config/ex-g-se/settings.enc`** - Encrypted config (AES-256-GCM)
- **Environment variables:** `EX_G_SE_PROVIDER`, `EX_G_SE_API_KEY`

---

## 🔐 Security Architecture

**Encryption:**
- Algorithm: `aes-256-gcm`
- KDF: `scrypt` (memory-hard)
- Memory cost: 64 MB
- Time cost: 3 iterations
- Salt: 128 bits (random per encryption)

**Security Level:** ⭐⭐⭐⭐⭐

**Key Principle:** No plaintext API keys stored on disk.

---

## 📝 Development Conventions

### Git Flow Master
```
TYPE: PROJECT_NAME - vX.Y.Z

- Change 1
- Change 2

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

**Types:**
- `RELEASE` - MAJOR (breaking changes)
- `UPDATE` - MINOR (new features)
- `PATCH` - PATCH (bug fixes)

### File Naming
- Rust files: `snake_case.rs`
- Node files: `camelCase.js`
- Config: `kebab-case`

### Import Rules
- Rust: Use crate-level imports
- Node: Use relative imports for local modules

---

## 🧪 Testing

**Test Coverage:** ~85%

**Run tests:**
```bash
# All tests
npm test

# Rust tests only
npm run test:rust
```

**Test Files:**
- `core/tests/integration_test.rs` - 10 integration tests
- `core/src/main.rs` - 2 unit tests

---

## 🎯 Quality Gates

Before committing:
- [ ] `npm run lint` passes
- [ ] `npm test` passes
- [ ] No plaintext credentials
- [ ] Git Flow Master format respected

---

## 📦 Dependencies

### Rust
```toml
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"
notify = "6"
arboard = "3"
device_query = "2"
serde_json = "1"
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.12", features = ["json"] }
async-trait = "0.1"
uuid = { version = "1.6", features = ["v4", "serde"] }
screenshots = "0.7"
image = { version = "0.24", features = ["png"] }
```

### Node.js
- No runtime dependencies (just wrapper)

---

## 🔍 Common Tasks

### Adding a new AI provider
1. Create `core/src/ai/{provider}.rs`
2. Implement `AIProvider` trait
3. Add to `core/src/ai/mod.rs`
4. Add option in `bin/config.js`

### Adding a new intent type
1. Update `Intent` enum in `core/src/analyzer.rs`
2. Add detection logic in `detectIntentFromEvent()`
3. Add thought in `bin/index.js` generateThought()

### Modifying script format
1. Edit `generateScript()` in `bin/index.js`
2. Or edit `core/src/script.rs` for Rust version

---

## 📚 Documentation

**User-facing:**
- `README.md` - Quick start and usage
- `CHANGELOG.md` - Version history

**Developer:**
- `CLAUDE.md` - This file
- `IMPLEMENTATION_SUMMARY.md` - v0.3.0 implementation details
- `QUALITY_METRICS.md` - Code quality report

---

## 🎨 Design Philosophy

1. **Security First** - No plaintext credentials
2. **Zero Config** - Works out of the box
3. **Auto-Everything** - Recording → Analysis → Scripts
4. **Cross-Platform** - Linux, macOS (Intel/ARM), Windows
5. **Brutalist** - ASCII design, minimal output

---

## 🚀 Release Process

1. Update version in `core/Cargo.toml`
2. Update version in `package.json`
3. Update `CHANGELOG.md`
4. Commit with Git Flow Master format
5. Push to main
6. Create tag: `git tag v0.3.0`
7. Push tag: `git push origin v0.3.0`
8. GitHub Actions builds and publishes to NPM

---

**Project:** EX-G-SE
**Version:** 0.3.0
**Maintainer:** Oalacea
