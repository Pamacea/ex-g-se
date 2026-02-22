# GitHub Actions Workflows

This directory contains CI/CD workflows for the EX-G-SE project.

## Workflows

### 1. `release.yml` - Release Workflow

Triggers on version tags (`v*`).

**Jobs:**
- **build-release**: Builds Rust binaries for multiple platforms
  - Linux x64 (ubuntu-latest)
  - macOS x64 (macos-13)
  - macOS ARM64 (macos-latest)
  - Windows x64 (windows-latest)
- **create-release**: Creates GitHub release with all binaries and checksums
- **publish-npm**: Publishes package to NPM

**Features:**
- Cross-platform binary compilation
- System dependencies installation for Linux (X11, Xtst, Xkbcommon)
- Artifact naming by platform
- SHA256 checksums generation
- Release notes from commit messages
- Provenance-enabled NPM publishing

### 2. `ci.yml` - Continuous Integration

Triggers on pull requests and pushes to main/develop branches.

**Jobs:**
- **test-rust**: Tests Rust core on all platforms
- **test-cli**: Tests Node.js CLI with multiple Node versions
- **lint-rust**: Rust formatting (fmt) and Clippy
- **lint-node**: ESLint, Prettier, TypeScript type checking
- **security**: Rust and Node.js security audits
- **build-verify**: Verifies build succeeds on all platforms

**Matrix Testing:**
- OS: Ubuntu, macOS, Windows
- Node.js: 18, 20, 22
- Rust: stable

**Caching:**
- Cargo registry, index, and build artifacts
- npm cache for faster installs

### 3. `lint.yml` - Fast Linting

Triggers on pull requests and pushes to main/develop branches.

**Jobs:**
- **rust-format**: Checks Rust code formatting
- **rust-clippy**: Runs Clippy linter
- **node-eslint**: Runs ESLint
- **node-format**: Checks Prettier formatting
- **node-types**: TypeScript type checking
- **license-check**: Validates license compliance
- **commit-lint**: Validates commit messages (PRs only)

**Features:**
- Cancels in-progress runs for same branch
- Faster feedback than full CI
- Continue-on-error for optional checks

## Required Secrets

Configure these in your repository settings (`Settings > Secrets and variables > Actions`):

| Secret | Description | Required For |
|--------|-------------|--------------|
| `NPM_TOKEN` | NPM authentication token | Publishing to NPM |
| `GITHUB_TOKEN` | GitHub token (automatic) | Creating releases |

## Setup Instructions

### 1. Create NPM Token

```bash
# Login to NPM
npm login

# Create a new token
# Visit: https://www.npmjs.com/settings/tokens
# Choose "Automation" type
# Copy the token
```

### 2. Add Secret to GitHub

1. Go to repository Settings
2. Navigate to "Secrets and variables" > "Actions"
3. Click "New repository secret"
4. Name: `NPM_TOKEN`
5. Paste your token value
6. Click "Add secret"

### 3. Configure Repository Settings

**Branch protection (recommended):**
1. Settings > Branches
2. Add rule for `main`
3. Require status checks to pass
4. Require branches to be up to date

### 4. Create a Release

```bash
# Bump version in package.json
npm version patch|minor|major

# Push tag
git push --tags

# Or manually:
git tag v1.0.0
git push origin v1.0.0
```

## Workflow Status Badges

Add these to your README.md:

```markdown
![CI](https://github.com/YOUR_USERNAME/ex-g-se/actions/workflows/ci.yml/badge.svg)
![Lint](https://github.com/YOUR_USERNAME/ex-g-se/actions/workflows/lint.yml/badge.svg)
![Release](https://github.com/YOUR_USERNAME/ex-g-se/actions/workflows/release.yml/badge.svg)
```

## Platform Support Matrix

| Platform | Architecture | Status | Runner |
|----------|--------------|--------|--------|
| Linux | x64 | ✅ | ubuntu-latest |
| macOS | x64 | ✅ | macos-13 |
| macOS | ARM64 | ✅ | macos-latest |
| Windows | x64 | ✅ | windows-latest |

## Troubleshooting

### Linux build fails with X11 errors
The workflow automatically installs required dependencies. If issues persist:
```yaml
- name: Install additional dependencies
  run: sudo apt-get install -y libxrandr-dev libxinerama-dev
```

### macOS ARM64 build not running
Ensure the job uses `macos-latest` runner for ARM64 builds.

### NPM publish fails
- Verify `NPM_TOKEN` is set correctly
- Ensure package.json has correct version
- Check if package name is available on NPM
- Verify `npm whoami` returns correct user

### Clippy warnings fail build
Clippy is configured with `-D warnings`. To allow specific warnings:
```bash
cargo clippy --manifest-path=core/Cargo.toml -- -D warnings -A clippy::warning_name
```
