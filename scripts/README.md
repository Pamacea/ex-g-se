# EX-G-SE Build Automation Scripts

This directory contains cross-platform build automation scripts for the EX-G-SE project.

## Overview

The scripts provide automated workflows for:
- **Cross-compilation** - Build Rust binaries for multiple platforms
- **Packaging** - Prepare NPM packages with compiled binaries
- **Development** - Local development and testing
- **Releases** - Version bumping and GitHub release automation

## Available Scripts

### Unix/Linux/macOS (`.sh`)

| Script | Purpose |
|--------|---------|
| `build-all.sh` | Cross-compile for all platforms (Linux, Windows, macOS) |
| `package.sh` | Prepare NPM package with binaries |
| `dev.sh` | Local development build and run |
| `release.sh` | Bump version, tag, and trigger release |

### Windows (`.bat`)

| Script | Purpose |
|--------|---------|
| `build-all.bat` | Cross-compile for all platforms (Windows native) |
| `dev.bat` | Local development build and run |

## Usage

### Build for All Platforms

```bash
# Unix/Linux/macOS
./scripts/build-all.sh

# Windows
scripts\build-all.bat

# Dry run (preview only)
./scripts/build-all.sh --dry-run
```

**Supported Platforms:**
- Linux x86_64 (`x86_64-unknown-linux-gnu`)
- Windows x86_64 (`x86_64-pc-windows-msvc`)
- macOS Intel (`x86_64-apple-darwin`)
- macOS ARM64 (`aarch64-apple-darwin`) - *Requires macOS host*

**Cross-Compilation Limitations:**
- macOS ARM64 builds must be performed on macOS or using the `cross` tool
- Windows builds on Linux require MinGW or appropriate toolchain
- For complete cross-platform builds, use GitHub Actions

### Local Development

```bash
# Unix/Linux/macOS
./scripts/dev.sh

# With options
./scripts/dev.sh --skip-build    # Use existing binary
./scripts/dev.sh --watch         # Watch mode (requires cargo-watch)

# Windows
scripts\dev.bat
scripts\dev.bat --skip-build
```

### Prepare NPM Package

```bash
# Build and package
./scripts/package.sh

# Use existing binaries
./scripts/package.sh --skip-build

# Dry run validation
./scripts/package.sh --dry-run
```

### Create Release

```bash
# Create release v1.0.0
./scripts/release.sh 1.0.0

# Preview release without executing
./scripts/release.sh 1.0.0 --dry-run
```

**Release Process:**
1. Updates versions in `Cargo.toml` and `package.json`
2. Commits version changes
3. Creates git tag (e.g., `v1.0.0`)
4. Pushes to remote
5. Triggers GitHub Actions release workflow

**Version Format:** Semantic Versioning (MAJOR.MINOR.PATCH)
- MAJOR: Incompatible API changes
- MINOR: New backwards-compatible functionality
- PATCH: Bug fixes

## Requirements

### For Building

- **Rust toolchain** - Install from https://rustup.rs/
- **cargo** - Comes with Rust
- **rustup** - For managing targets

### For Packaging

- **npm** - Node.js package manager
- **Node.js 18+** - Required for CLI wrapper

### For Releases

- **git** - Version control
- **GitHub CLI** (optional) - For creating releases
- **Clean working directory** - No uncommitted changes

## Project Structure

```
ex-g-se/
├── scripts/
│   ├── build-all.sh       # Cross-compilation (Unix)
│   ├── build-all.bat      # Cross-compilation (Windows)
│   ├── package.sh         # NPM package preparation
│   ├── dev.sh             # Development (Unix)
│   ├── dev.bat            # Development (Windows)
│   └── release.sh         # Release automation
├── core/                  # Rust core library
│   ├── Cargo.toml
│   └── src/
├── cli/                   # Node.js CLI wrapper
│   ├── package.json
│   └── bin/
└── bin/                   # Compiled binaries output
    ├── ex-g-se-linux
    ├── ex-g-se-win.exe
    ├── ex-g-se-macos-x64
    └── ex-g-se-macos-arm64
```

## Environment Variables

### Build Scripts

- `REMOTE` - Git remote name (default: `origin`)
- `BRANCH` - Main branch name (default: `main`)

### Release Script

```bash
# Custom remote/branch
REMOTE=upstream BRANCH=develop ./scripts/release.sh 1.0.0
```

## Error Handling

All scripts include:
- **Prerequisite checks** - Verify required tools are installed
- **Validation** - Check for uncommitted changes, existing tags
- **Clear error messages** - Descriptive output for debugging
- **Dry-run mode** - Preview changes without executing

## Exit Codes

- `0` - Success
- `1` - Error (check output for details)

## Troubleshooting

### Build Failures

**Problem:** Target not found
```
error: target not found: x86_64-pc-windows-msvc
```
**Solution:**
```bash
rustup target add x86_64-pc-windows-msvc
```

### Permission Denied

**Problem:** Scripts not executable
```
bash: ./scripts/build-all.sh: Permission denied
```
**Solution:**
```bash
chmod +x scripts/*.sh
```

### Cross-Compilation Issues

**Problem:** Cannot build for target on current platform
```
error: linker not found
```
**Solution:** Use GitHub Actions or install cross-compilation toolchain:
```bash
cargo install cross
cross build --release --target aarch64-apple-darwin
```

### Release Errors

**Problem:** Tag already exists
```
Error: Tag v1.0.0 already exists
```
**Solution:**
```bash
# Delete local and remote tag
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
```

## CI/CD Integration

These scripts are designed to work with GitHub Actions:

```yaml
# .github/workflows/release.yml
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
          - os: macos-latest
            target: x86_64-apple-darwin
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          target: ${{ matrix.target }}
      - run: cargo build --release --target ${{ matrix.target }}
```

## Contributing

When adding new scripts:
1. Include proper error handling
2. Add documentation header
3. Support dry-run mode where applicable
4. Follow existing naming conventions
5. Update this README

## License

MIT License - See LICENSE file in project root.
