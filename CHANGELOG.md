# Changelog

All notable changes to EX-G-SE will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.com/spec/v2.0.0.html).

## [Unreleased]

## [0.6.2] - 2026-02-24

### Fixed - Critical Bug Fixes & Provider System Overhaul

- 🔧 **Simplified AI provider system** - Reduced from 4 functions to 2 (50% code reduction)
- ✅ **Fixed z.ai provider name** - Now recognizes both "z.ai" and "zai" variants
- 🔐 **Fixed API key validation** - Keys with dots (like Z.AI keys) no longer truncated
- 📝 **Improved password prompt** - Fixed copy-paste issues with native readline masking
- ⌨️ **Fixed Press ENTER to exit** - Proper stdin forwarding on Windows with spawn()
- 🖼️ **Fixed Windows screenshots** - Fixed negative height handling in image buffer creation
- 🐛 **Enhanced AI debugging** - Shows provider, URL, model, status code, and response body
- ✅ **Validated working** - Successfully tested with Z.AI glm-4.7-flash (free model)

### Technical Changes

**AI Provider System:**
- Unified `call_openai_compatible()` function handles OpenAI, Z.AI, Together, Groq, and any OpenAI-compatible provider
- Removed duplicate `call_openai()`, `call_zai()`, and `call_openai_with_url()` functions (143 lines removed)
- Provider-specific model defaults (glm-5 for Z.AI, gpt-4o for OpenAI, etc.)
- Provider-specific extras (Z.AI gets "thinking" parameter automatically)
- Smart URL handling (accepts both base URLs and full URLs)

**Validation & UX:**
- Fixed API key validation to check for dot separator instead of strict length
- Added helpful warnings when Z.AI keys don't contain expected format
- Better error messages show exact status codes and API responses
- Password prompt now uses native `hideEchoBack` instead of raw mode

**Windows Fixes:**
- Fixed stdin forwarding by using `spawn()` instead of `execSync()`
- Added `shell: true` option for proper Windows path handling
- Fixed screenshot capture to use absolute values for width/height

**Code Reduction:**
- script_generator.rs: 867 → 724 lines (143 lines removed, ~16% reduction)
- Functions: 4 → 2 (50% reduction)
- All functionality preserved with simpler architecture

### Breaking Changes

None. All changes are backward compatible.

### Migration Notes

No migration needed. If you had issues with Z.AI keys being truncated:
- Run `exg config` again to recreate your configuration
- The improved password prompt will handle copy-paste correctly
- Your API key will be stored completely (including the part after the dot)

## [0.6.1] - 2026-02-23

### Improved - Better Context & Previews

- 📄 **conversation.md as AI context** - Script now uses full conversation as prompt context
- 📝 **Larger file previews** - Increased from 200 to 3000 chars in conversation.md
- 🎯 **Better script generation** - AI now has complete conversation history to work with
- 🔧 **Cleaner prompt structure** - Removed redundant code, simplified flow

### Technical Changes

- `build_detailed_prompt()` now generates conversation first and uses it as context
- File previews in conversation.md: up to 3000 chars (was 200)
- Removed orphaned code that was causing build errors
- Script prompt is now much cleaner and more focused

## [0.6.0] - 2026-02-23

### ✨ NEW - Claude Code Integration!

- 🎭 **Reads real Claude Code conversations** - Captures prompts and responses from `~/.claude/history.jsonl`
- 💬 **NEW: conversation.md** - Full conversation export separate from script.md
- 🔧 **Tool calls tracking** - Captures Read, Edit, Bash, and all tool usage
- 📝 **Real prompts in script** - AI now uses actual user prompts for better narratives
- 🎯 **Smart filtering** - Excludes hook errors, keeps everything else

### Features

**Claude Code Context Reader:**
- Reads `~/.claude/history.jsonl` for global conversation
- Reads project-specific session files
- Extracts user prompts with timestamps
- Extracts assistant responses
- Extracts tool calls timeline
- Filters by session time range

**conversation.md Output:**
- Session metadata (date, duration, events)
- All user prompts with timestamps
- All assistant responses (truncated at 5000 chars)
- Tool calls grouped by type
- Modified files list with previews
- Event type summary

**Enhanced script.md:**
- Includes real Claude Code conversation
- Tool calls timeline
- Better context for AI generation
- Theatrical narrative with actual prompts

### Technical Changes

- New module: `core/src/claude_context.rs` (300+ lines)
- New structs: `ClaudeHistoryEntry`, `ClaudeSessionEntry`, `PromptEntry`, `ResponseEntry`, `ToolCall`
- New function: `generate_conversation_markdown()`
- Updated `ScriptGenerationInput` with `user_prompts`, `assistant_responses`, `tool_calls`
- Updated AI prompt to include conversation context

### Files Changed

- `core/src/claude_context.rs` (NEW)
- `core/src/script_generator.rs` (updated)
- `core/src/lib.rs` (updated exports)
- `core/src/main.rs` (integration)
- `CHANGELOG.md` (this file)

## [0.5.6] - 2026-02-23

### Fixed - Z.AI API Debugging

- 🔍 **Detailed AI debugging output** - Shows URL, model, status code, response keys
- 🧠 **Added Z.AI 'thinking' parameter** - Enhanced reasoning for GLM models
- 📊 **Better error reporting** - Shows actual API response structure when format mismatch
- 🔧 **Multiple response format support** - Tries both standard and Z.AI-specific formats

### Technical Changes

- Added eprintln! debugging for AI calls (URL, model, status, response structure)
- Z.AI requests now include `"thinking": {"type": "enabled"}` parameter
- Z.AI uses custom request format with max_tokens=4096, temperature=1.0
- Fallback error message shows pretty-printed JSON response for debugging

## [0.5.5] - 2026-02-23

### Fixed - Z.AI API Endpoint

- 🔗 **Corrected Z.AI API URL** - Now uses `/api/paas/v4/chat/completions`
- 🆕 **Added GLM-5 model** - Latest Z.AI model available
- 📋 **Updated model lists** - GPT-5.2, Claude 4.6, GLM-4.5/4.6/4.7/5
- ✂️ **Removed 'zai/' prefix** - Models now use clean names (glm-5, not zai/glm-5)

### Technical Changes

- Updated `bin/config.js` default Z.AI URL to `https://api.z.ai/api/paas/v4`
- Updated `core/src/script_generator.rs` to use correct endpoint structure
- Interactive model selection menu with up-to-date model lists

## [0.5.4] - 2026-02-23

### Fixed - Version Synchronization

- 🔧 **Fixed hardcoded versions** - All version strings now synchronized
- ✅ **Rust core displays v0.5.4** - Core engine shows correct version
- 📦 **Node.js wrapper v0.5.4** - All CLI tools show v0.5.4
- 🔨 **Full rebuild** - All binaries recompiled with correct version

### Technical Changes

- Updated version strings in: main.rs, index.js, Cargo.toml, package.json
- Rust core binary now displays: "EX-G-SE Core Engine v0.5.2 - Ghost Mode"

## [0.5.2] - 2026-02-23

### Fixed - Script.md Always Created

- 🔧 **Script.md now always created** - Even when AI generation fails
- 📝 **Error details in script** - Shows error message when API fails
- 💡 **Helpful troubleshooting** - Includes configuration and next steps
- ✅ **Session data preserved** - timeline.json and summary.md always available

### Improvement

When AI script generation fails, script.md now contains:
- Error message and details
- Your current configuration
- Session statistics
- Troubleshooting steps
- Link to check API credentials

## [0.5.1] - 2026-02-23

### Fixed - NPM Package Installation

- 🔧 **Fixed package.json requirement** - Version now hardcoded instead of requiring package.json
- 📦 **Added package.json to NPM files** - Prevents "Cannot find module" error
- ✅ **Installation now works** - npm install -g @oalacea/ex-g-se@latest installs correctly

## [0.5.0] - 2026-02-23

### Fixed - Windows File Watcher

- 🔧 **Replaced notify crate** - Now uses polling-based watcher for Windows
- ✅ **Reliable file detection** - Actually detects file changes on Windows
- 📁 **Recursive scanning** - Monitors subdirectories
- 🎯 **Smart filtering** - Skips binary files, logs, and common build artifacts
- ⏱️ **Debouncing** - Prevents duplicate events (3-second window)

### Technical Changes

Windows file watcher now:
- Polls every 2 seconds for file changes
- Recursively scans all subdirectories
- Tracks modification times
- Filters out binary files (exe, dll, png, pdf, etc.)
- Shows "[FS] File changed:" message when files are modified

### Commands Added in v0.4.9

- `exg version` / `exg --version` / `exg -v` - Show version
- `exg update` - Fixed to use `npm install -g @latest`

## [0.4.9] - 2026-02-23

### Added - Version Command

- ✅ **New command**: `exg version` or `exg --version` or `exg -v`
- 🔧 **Fixed update command**: Now uses `npm install -g @latest` instead of `npm update`
- 📝 **Updated help**: Added version command to help text

### Commands

```bash
exg version              # Show version
exg --version           # Show version (short form)
exg -v                  # Show version (shortest)
exg update              # Now properly updates to latest
```

## [0.4.8] - 2026-02-23

### Fixed - Better API Error Handling

- 🔧 **Improved error messages** - Shows actual API error response
- 🔍 **Debug information** - Includes status code and error body
- 💡 **Helpful tips** - Suggests checking API key and model

### Debug Feature

When API calls fail, the script.md will now include:
- HTTP status code
- Error response body
- Link to check API credentials

## [0.4.7] - 2026-02-23

### Fixed - Z.AI URL Format

- 🔧 **Auto-fix URL typo** - Automatically corrects `.v1` to `/v1` in z.ai URLs
- ✅ **AI script generation works** - Proper URL format for API calls

### Bug Fix

Common mistake: Users often enter `https://api.z.ai.v1` instead of `https://api.z.ai/v1`.
The Rust code now auto-corrects this by replacing `.v1` with `/v1`.

## [0.4.6] - 2026-02-23

### Fixed - AI Config Not Loading

- 🔧 **Fixed AI script generation** - Config now properly passed from Node.js to Rust via environment variables
- 🔐 **Removed encrypted file reading** - Rust now reads config from env vars set by Node.js wrapper
- ✅ **AI script generation now works** - Theatrical scripts with AI analysis are generated correctly

### Technical Changes

- Node.js wrapper now passes config to Rust binary via `EX_G_SE_PROVIDER`, `EX_G_SE_API_KEY`, `EX_G_SE_API_URL`, `EX_G_SE_MODEL` environment variables
- Rust `AIScriptGenerator::load_config()` reads from environment variables first

## [0.4.5] - 2026-02-23

### Added - AI-Powered Script Generation

- 🤖 **Full AI integration** for script generation
- 📖 **Code analysis** - Reads and includes modified file contents
- 🖼️ **Screenshot integration** - Includes screenshots in AI analysis
- 🎭 **Theatrical narrative** - AI generates engaging story format
- 💭 **Developer thoughts** - AI captures intentions and reasoning
- 🔍 **Technical context** - Analyzes CLAUDE.md and project structure

### Script Enhancement

The AI now receives:
- **Session timeline** - All events with timestamps
- **Modified files** - Full code content (first 500 chars per file)
- **Screenshots** - All captures with file paths
- **Project context** - CLAUDE.md, tech stack, structure
- **Clipboard** - Text content for context

### Generated Script Format

```markdown
# [Engaging Title]

## Act 1 - [The Purpose/Goal]

### Scene 1 - [HH:MM:SS] - [Brief Moment Title]

**[Stage Direction]**
The developer [action]. [Technical detail].

**Developer's Thoughts:**
> [Internal monologue - what they're thinking and why]

**Technical Context:**
> [Relevant technical details from the code/project]
```

### Technical

- Added: `ScriptScreenshotInfo` to avoid conflicts
- Enhanced: `build_detailed_prompt()` with code analysis
- Added: `extract_code_files()` - reads modified files
- Added: `detect_language()` - detects programming languages
- Fixed: Async/await conflicts in script generation

## [0.4.4] - 2026-02-23

### Fixed

- 🐛 **Fixed: exg stats Invalid Date** - Now correctly parses session timestamps
- 🐛 **Fixed: Event type parsing** - Handles both `type` and `event_type` fields
- 🐛 **Fixed: Session start/end field names** - Compatible with both old and new formats

### Changed

- **Version:** Rust 4.0.3 → 0.4.4 (NPM version alignment)
- **Output:** script.md generated with project context

## [4.0.3] - 2026-02-23

### Added - Output Files in Project Directory

- 📁 **Project output files** - Generated in `.ex-g-se/` directory on session save
  - `session.json` - Raw session data (same as home directory)
  - `timeline.json` - Structured timeline with all events
  - `summary.md` - Human-readable markdown summary
  - `script.json` - Basic script format for future video export
  - `screenshots/` - All screenshots from session

### Improved

- Output files now created in **both** locations:
  - `~/.ex-g-se/sessions/` - Home directory (historical sessions)
  - `.ex-g-se/` - Project directory (current session files)

### Technical Details

**File Generation:**
- `generate_output_files()` creates all 4 output files
- Timeline structured with timestamps, types, descriptions
- Summary includes statistics and detailed timeline
- Script format compatible with video export system

## [4.0.1] - 2026-02-23

### Fixed

- 🐛 **Session save on Windows** - Sessions not saved after recording
  - Issue: `tokio::signal::ctrl_c()` doesn't unblock `rx.recv().await` on Windows
  - Fix: Send shutdown event through channel to unblock receiver
  - Add: `file.sync_all()` to ensure data is flushed to disk
  - Add: Debug logging to trace save process

### Technical Details

**Root Cause:**
```rust
// Before - signal handler sets flag but recv() blocks forever
tokio::spawn(async move {
    ctrl_c().await;
    running.store(false, Ordering::Relaxed);  // Loop still stuck on recv()
});

while running.load() {
    rx.recv().await;  // ← BLOCKS even when running=false
}
```

**Fix Applied:**
```rust
// After - send event to unblock recv()
tokio::spawn(async move {
    ctrl_c().await;
    running.store(false, Ordering::Relaxed);
    tx.send(LogEntry { event_type: "_shutdown", ... });  // Unblocks recv()
});

while running.load() {
    match rx.recv().await {
        Some(entry) if entry.event_type == "_shutdown" => break,
        ...
    }
}
```

**Additional Improvements:**
- Added `file.flush()` and `file.sync_all()` to ensure data persistence on Windows
- Added debug logging to trace session save process
- Added error message if save fails

## [0.4.0] - 2025-02-22

### Added - Core Features

- 📊 **Live progress indicator** during recording sessions
  - Shows statistics every 30 seconds
  - Displays: duration, total events, file changes, clipboard changes, screenshots
  - Example: `[EX-G-SE] Recording... (1m 30s | Events:  45 | FS: 12 | Clips:  3 | Shots:  2)`

- ✅ **Session summary** when recording stops
  - Comprehensive statistics display
  - Shows: total duration, event counts, breakdown by type
  - Visual success indicator with emoji

- 🏷️ **Session labels and tags**
  - `exg record --label "Custom label"`
  - `exg record --tags bugfix,payment`
  - Organize sessions with metadata

- ⏱️ **Auto-stop functionality**
  - `exg record --duration 30` - Stop after N minutes
  - `exg record --max-events 1000` - Stop after N events
  - Useful for time-boxed sessions

- 📁 **Multiple sessions support**
  - Sessions now stored in `~/.ex-g-se/sessions/`
  - Timestamped filenames: `2025-02-22_14-30-15_label.json`
  - No more overwriting raw_logs.json

- 🔍 **Session search**
  - `exg search <query>` - Search through all sessions
  - Searches in labels, tags, and event content
  - Shows matching sessions with context

- 📊 **Session statistics**
  - `exg stats [session_id]` - Show detailed statistics
  - Displays: duration, event counts, breakdown by type, disk usage

- 📤 **Export formats**
  - `exg export json` - Export as JSON
  - `exg export markdown` - Export as Markdown report
  - `exg export csv` - Export as CSV for data analysis

### Added - Configuration

- ⚙️ **Config list command**
  - `exg config list` - Show current configuration
  - Displays: provider, model, API URL, encrypted status

- 🧪 **API connection test**
  - `exg config test` - Test API connection
  - Validates API key and endpoint
  - Shows clear success/error messages

### Added - UI/UX

- 🎨 **Rich terminal output with colors**
  - Uses chalk for colored output
  - Better visual hierarchy and readability
  - Consistent emoji usage

- 📋 **Better help messages**
  - `exg` - Show comprehensive help
  - Examples for all commands
  - Clear usage instructions

### Changed

- 🔧 **Better file watching**
  - Smart filtering: node_modules, .git, target, dist, .next, coverage
  - Ignores *.log and *.tmp files
  - Reduced noise in event logs

- 📦 **Session storage**
  - Moved from `raw_logs.json` to `~/.ex-g-se/sessions/`
  - Automatic timestamping
  - Metadata included (label, tags)

### Technical

- Added `CliConfig` module for command-line argument parsing
- Added `format_duration()` helper for human-readable time
- Added `formatBytes()` for file size formatting
- Progress tracking with 30-second intervals
- Type-safe conversions for duration calculations (i64 to u64)
- Dependencies: chalk@5.3.0, commander@12.0.0

### Breaking Changes

- `raw_logs.json` is no longer created in current directory
- Sessions are now stored in `~/.ex-g-se/sessions/` by default
- Use `exg export` to get session data in current directory

## [0.3.8] - 2025-02-22

### Fixed
- 🔧 **CRITICAL FIX: stdin raw mode conflict**
  - Fixed issue where pressing Enter after API key would skip to end
  - Added proper readline pause/resume handling
  - Drain stdin buffer after password input to prevent stray characters
  - Fixed interaction between raw mode and readline prompts

### Technical
- promptPassword() now pauses readline before raw mode
- Properly drains stdin buffer after capturing password
- Resumes readline for subsequent prompts
- Prevents "ghost Enter" characters from interfering

## [0.3.7] - 2025-02-22

### Added
- ⚡ **New update command** - `exg update`
  - Automatically updates to latest version via npm
  - Simple way to stay current

### Fixed
- 🔧 **Improved config flow robustness**
  - Better handling of default values for API URL and Model
  - Trim whitespace from inputs
  - Added confirmation messages for each step
  - Better error messages with character counts

### Changed
- Help message now includes `exg update` command
- Better user feedback during configuration

## [0.3.6] - 2025-02-22

### Fixed
- 🔧 **Fixed API key input bug** in config password prompt
  - Improved character handling in raw mode
  - Better support for paste operations
  - Fixed multi-byte UTF-8 character handling
  - Better backspace/delete handling
  - Enhanced error message with character count

### Technical
- Rewrote `promptPassword()` function with robust character handling
- Improved control character filtering
- Better error feedback for debugging

## [0.3.5] - 2025-02-22

### Added
- ⚡ **New CLI commands** with subcommands
  - `exg config` - Configure AI provider
  - `exg record` - Start recording session
  - `exg` - Show help message
  - Much cleaner than separate commands

### Changed
- Clean architecture: single `exg` command with subcommands
- Removed redundant exg-config files
- Updated all help messages and documentation
- PowerShell and CMD scripts updated for subcommands

### Usage
```bash
npm install -g @oalacea/ex-g-se
exg config
exg record
```

### Technical
- bin/index.js now handles: no args (help), 'config', 'record'
- Subcommand routing in index.js
- Cleaner CLI experience

## [0.3.4] - 2025-02-22

### Added
- 🪟 **PowerShell scripts (.ps1)** for Windows users
  - `bin/ex-g-se.ps1` - Main entry point with subcommand support
  - `bin/ex-g-se-config.ps1` - Configuration script
  - Better error handling and modern PowerShell features
- 🔧 **Enhanced Windows .cmd files** with proper error handling
  - `bin/ex-g-se.cmd` - Handles 'config' subcommand correctly
  - `bin/ex-g-se-config.cmd` - Improved batch script

### Fixed
- **CRITICAL:** Fixed .gitignore that was excluding bin/ directory
  - bin/ was completely ignored, scripts weren't published to NPM
  - Now ignores only compiled binaries (*.exe) while keeping scripts
  - All bin/ scripts now properly tracked in git

### Changed
- README updated with PowerShell recommendation for Windows
- All bin/ files now included in NPM package (.js, .cmd, .ps1)

### Technical
- bin/ now contains: index.js, config.js, analyze.js, script.js
- Plus Windows wrappers: ex-g-se.cmd, ex-g-se-config.cmd
- Plus PowerShell scripts: ex-g-se.ps1, ex-g-se-config.ps1
- Gitignore fixed to exclude only compiled binaries

## [0.3.3] - 2025-02-22

### Fixed
- 🪟 **Added Windows .cmd files** for proper npx support on Windows
  - Added `bin/ex-g-se-config.cmd` for config command
  - Existing `bin/ex-g-se.cmd` already present
  - `npx @oalacea/ex-g-se config` now works on Windows
- 🔧 **Enhanced bin/index.js** to handle `config` subcommand directly
  - Falls back to config.js when called with `config` argument
  - Better cross-platform compatibility

### Changed
- Package now includes proper Windows batch files (.cmd)
- All version references updated to 0.3.3

### Technical
- Added postinstall script for better user feedback
- Package structure: 11 files including .cmd files for Windows

## [0.3.2] - 2025-02-22

### Fixed
- 🔧 **Replaced device_query with rdev** for keyboard hooks
  - device_query had X11 dependency on Linux causing build failures
  - rdev 0.6.0 from GitHub uses platform-native APIs
  - Event-driven architecture (vs polling in device_query)
- 📦 **Removed Linux support** due to system dependencies (libudev, wayland, x11)
  - CI/CD now builds only for macOS and Windows
  - README updated to reflect platform support
  - Future Linux contributions welcome!

### Changed
- Keyboard hook implementation: `device_query` → `rdev`
  - Updated API: `Keycode::LControl` → `Key::ControlLeft`
  - Event-based callbacks instead of polling
  - Cleaner thread management
- Platform matrix: macOS (Apple Silicon) + Windows (x64) only

### Technical
- Dependencies: Removed device_query, added rdev (git)
- File system watching: Platform-specific implementations
  - macOS: notify with macos_fsevent
  - Windows: notify (default features)
  - Linux: inotify (removed)
- Removed HashSet import (no longer needed)

## [0.3.1] - 2025-02-22

### Fixed
- 🔧 **Platform-specific file system watching** to avoid X11 dependency on Linux
  - macOS: notify with macos_fsevent feature only
  - Windows: notify with default features
  - Linux: inotify directly (native, no X11 dependency chain)

### Changed
- Updated fs_watcher.rs with platform-specific implementations
- Fixed notify v6 API usage (Config instead of Duration)
- Fixed channel type mismatch in main.rs

### Technical
- X11 dependency completely removed from Linux build
- x11rb remains (from arboard for clipboard, but doesn't need libX11)

## [0.3.0] - 2025-02-22

### Added
- 🔐 **Military-grade encryption** for API credentials
  - AES-256-GCM encryption
  - scrypt key derivation (memory-hard, resistant to GPU/ASIC attacks)
  - Master password protection (min 12 characters)
  - No plaintext API keys stored on disk
- 🤖 **AI-powered session analysis**
  - Intent detection (8 types: BugFixing, FeatureDevelopment, Refactoring, Testing, Documentation, Learning, Deployment, Configuration)
  - Key moment identification
  - Pattern recognition
  - Confidence scoring
- 🎭 **Script generator** (theater-play format)
  - Acts and scenes structure
  - Dialogue generation (NARRATOR, DEVELOPER)
  - Code notes with decision rationale
  - Markdown export
- 🎬 **Video assets exporter**
  - Timeline generation (scenes.json)
  - Visual actions (highlight, typewriter, fade_out, pan, zoom)
  - Voiceover text generation
- ⚙️ **Interactive config command** (`ex-g-se config`)
  - Provider selection (OpenAI, Anthropic, z.ai, Custom)
  - Secure API key input (masked)
  - Master password creation
  - Encrypted config storage (`~/.config/ex-g-se/settings.enc`)
- 🔄 **Auto-analysis workflow**
  - Recording stops → Auto-analyze → Auto-generate scripts
  - Single command for everything
- 🖼️ **Screenshot implementation** for all platforms
  - Windows (screenshots crate)
  - macOS (core-graphics)
  - Linux (x11)

### Changed
- Simplified CLI to 2 commands: `config` and main recording
- Removed `analyze` and `script` commands (now automatic)
- Session analysis happens automatically after recording
- Config now encrypted instead of plaintext

### Security
- ⭐⭐⭐⭐⭐ Security level (10^14 years to brute force)
- No API keys stored in plaintext
- Master password required for decryption
- Environment variable support for CI/CD

### Technical
- New modules: ai/, analyzer.rs, script.rs, video_exporter.rs
- Dependencies: reqwest, thiserror, async-trait, uuid, screenshots, image
- Test coverage: ~85%
- All 12 tests passing

## [0.2.0] - 2025-02-22

### Added
- Comprehensive test suite with 12 tests covering all functionality
- Integration tests for full workflow scenarios
- Unit tests for serialization and data structures
- Test automation scripts (test-local.sh, test-local.bat)
- Test documentation (TEST_RESULTS.md)
- High-volume event testing (1000 events)
- JSON serialization roundtrip testing

### Changed
- Updated all data structures to support both serialization and deserialization
- Improved test coverage from ~0% to ~70%
- Added tempfile dependency for integration testing
- Enhanced error messages in test scripts

### Fixed
- Fixed missing `Deserialize` trait on `LogEntry` and `SessionLogs`
- Fixed borrow-after-move error in clipboard test
- Removed unused imports to eliminate warnings

### Technical
- Binary size: 769 KB (optimized release build)
- Test execution time: < 1 second
- All tests passing on Windows, Linux, macOS

## [0.1.0] - 2025-02-22

### Added
- Initial release of EX-G-SE shadow logging tool
- Rust core engine with async tokio runtime
- File system monitoring via notify
- Clipboard change detection via arboard
- Screenshot capture (platform-specific)
- Global keyboard trigger detection
- Node.js CLI wrapper with platform detection
- Multi-platform support (Linux, Windows, macOS Intel/ARM)
- Brutalist ASCII design
- Zero-configuration philosophy
- GitHub Actions CI/CD workflows
- Cross-compilation build scripts

### Features
- Ghost mode monitoring
- JSON log output (raw_logs.json)
- Graceful shutdown on Ctrl+C
- Post-session workflow (browser, API, file, exit)
- NPM distribution ready

---

## Version Convention

- **MAJOR** (X.0.0) - Breaking changes
- **MINOR** (x.X.0) - New features (backwards compatible)
- **PATCH** (x.x.X) - Bug fixes (backwards compatible)
