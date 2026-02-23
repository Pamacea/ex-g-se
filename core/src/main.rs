// EX-G-SE: Shadow Logging Core Engine
//
// A cross-platform Rust core that runs in "Ghost Mode" monitoring:
// - File system changes
// - Clipboard content
// - Screenshots (interval-based)
// - Global keyboard input (trigger detection)
//
// PLATFORMS: macOS (Apple Silicon), Windows (x64)
//
// Usage:
//   exg                  Start recording (Ctrl+Shift+X to stop)
//   exg-config           Configure AI provider
//
// Or via npx:
//   npx @oalacea/ex-g-se
//   npx @oalacea/ex-g-se config

use anyhow::Result;
use chrono::{DateTime, Utc};
use rdev::{Event, EventType, Key};
use serde::Serialize;
use serde_json::json;
use ex_g_se::{capture_screenshot, fs_watcher, CliConfig};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc as std_mpsc};
use std::time::Duration;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc;
use tokio::time::interval;

// Global shutdown hook for Windows
static SHUTDOWN_HOOK: std::sync::Once = std::sync::Once::new();

// ============================================================================
// Data Structures
// ============================================================================

/// Log entry structure
#[derive(Debug, Clone, Serialize)]
struct LogEntry {
    #[serde(rename = "ts")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    event_type: String,
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// Complete session logs
#[derive(Debug, Serialize)]
struct SessionLogs {
    #[serde(rename = "start")]
    start_time: DateTime<Utc>,
    #[serde(rename = "end")]
    end_time: Option<DateTime<Utc>>,
    #[serde(rename = "events")]
    events: Vec<LogEntry>,
}

// ============================================================================
// Application State
// ============================================================================

/// EX-G-SE Core Engine
struct ExGSeEngine {
    events: Vec<LogEntry>,
    start_time: DateTime<Utc>,
    running: Arc<AtomicBool>,
    trigger_keys: Vec<Key>,
    screenshot_interval_secs: u64,
    cli_config: CliConfig,
    // Better file filtering
    ignore_patterns: Vec<String>,
}

impl ExGSeEngine {
    fn new() -> Self {
        let cli_config = CliConfig::from_args();

        // Show CLI config if set
        if let Some(ref label) = cli_config.session_label {
            eprintln!("[CLI] Session label: {}", label);
        }
        if !cli_config.tags.is_empty() {
            eprintln!("[CLI] Tags: {}", cli_config.tags.join(", "));
        }
        if let Some(duration) = cli_config.max_duration_secs {
            eprintln!("[CLI] Auto-stop after: {}s", duration);
        }
        if let Some(max_events) = cli_config.max_events {
            eprintln!("[CLI] Max events: {}", max_events);
        }

        Self {
            events: Vec::new(),
            start_time: Utc::now(),
            running: Arc::new(AtomicBool::new(true)),
            // Trigger: Ctrl+Shift+X
            trigger_keys: vec![Key::ControlLeft, Key::ShiftLeft, Key::KeyX],
            screenshot_interval_secs: 30,
            cli_config,
            // Better file filtering
            ignore_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "dist".to_string(),
                ".ex-g-se".to_string(),
                ".next".to_string(),
                "coverage".to_string(),
                "*.log".to_string(),
                "*.tmp".to_string(),
            ],
        }
    }

    /// Check if path should be ignored
    fn should_ignore_path(&self, path: &str) -> bool {
        for pattern in &self.ignore_patterns {
            if pattern.starts_with('*') {
                // Extension pattern
                let ext = &pattern[1..];
                if path.ends_with(ext) {
                    return true;
                }
            } else if path.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Record an event
    #[allow(dead_code)]
    pub fn log_event(&mut self, event_type: &str, data: serde_json::Value) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            data,
        };
        self.events.push(entry);
    }

    /// Write logs to file in sessions directory
    fn save_logs(&self) -> Result<()> {
        // Create sessions directory
        let sessions_dir = dirs::home_dir()
            .map(|home| home.join(".ex-g-se").join("sessions"))
            .unwrap_or_else(|| PathBuf::from(".ex-g-se/sessions"));

        eprintln!("[DEBUG] Sessions directory: {:?}", sessions_dir);
        fs::create_dir_all(&sessions_dir)?;
        eprintln!("[DEBUG] Directory created/verified");

        // Generate timestamped filename
        let timestamp = self.start_time.format("%Y-%m-%d_%H-%M-%S");
        let label_part = self.cli_config.session_label.as_ref()
            .map(|l| l.replace(' ', "_"))
            .unwrap_or_else(|| "session".to_string());
        let filename = format!("{}_{}.json", timestamp, label_part);
        let filepath = sessions_dir.join(&filename);

        eprintln!("[DEBUG] Saving to: {:?}", filepath);
        eprintln!("[DEBUG] Events count: {}", self.events.len());

        let logs = SessionLogs {
            start_time: self.start_time,
            end_time: Some(Utc::now()),
            events: self.events.clone(),
        };

        let json = serde_json::to_string_pretty(&logs)?;
        eprintln!("[DEBUG] JSON size: {} bytes", json.len());

        let mut file = File::create(&filepath)?;
        file.write_all(json.as_bytes())?;
        file.flush()?; // Ensure data is written to disk
        file.sync_all()?; // Force OS to flush to disk

        eprintln!("[DEBUG] File written and synced");

        // Verify file exists
        if !filepath.exists() {
            return Err(anyhow::anyhow!("File was not created: {:?}", filepath));
        }

        // Show clear success message
        eprintln!("\n{}", "=".repeat(60));
        eprintln!("✅ SESSION SAVED SUCCESSFULLY!");
        eprintln!("{}", "=".repeat(60));
        eprintln!("\n📁 Session file: {}", filepath.display());
        eprintln!("📊 Events captured: {}", self.events.len());

        Ok(())
    }

    /// File system watcher with better filtering
    async fn watch_fs(&self, tx: mpsc::UnboundedSender<LogEntry>) -> Result<()> {
        let running = self.running.clone();
        let ignore_patterns = self.ignore_patterns.clone();
        let (fs_tx, fs_rx) = std_mpsc::channel();

        // Spawn fs watcher in blocking thread
        std::thread::spawn(move || {
            if let Err(e) = fs_watcher::watch_directory(PathBuf::from("."), fs_tx) {
                eprintln!("FS watcher error: {}", e);
            }
        });

        // Forward fs events to main channel
        tokio::spawn(async move {
            while running.load(Ordering::Relaxed) {
                match fs_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(event) => {
                        // Better filtering
                        let path = event.data["path"].as_str().unwrap_or("unknown");

                        let mut should_ignore = false;
                        for pattern in &ignore_patterns {
                            if pattern.starts_with('*') {
                                // Extension pattern
                                let ext = &pattern[1..];
                                if path.ends_with(ext) {
                                    should_ignore = true;
                                    break;
                                }
                            } else if path.contains(pattern) {
                                should_ignore = true;
                                break;
                            }
                        }

                        if should_ignore {
                            continue;
                        }

                        let entry = LogEntry {
                            timestamp: Utc::now(),
                            event_type: "fs_change".to_string(),
                            data: event.data,
                        };
                        let _ = tx.send(entry);
                        eprint!(".");
                    }
                    Err(std_mpsc::RecvTimeoutError::Timeout) => {
                        // Continue checking
                    }
                    Err(std_mpsc::RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// Clipboard monitor
    async fn watch_clipboard(&self, tx: mpsc::UnboundedSender<LogEntry>) -> Result<()> {
        let running = self.running.clone();

        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(2));
            let mut last_content: Option<String> = None;

            loop {
                timer.tick().await;

                if !running.load(Ordering::Relaxed) {
                    break;
                }

                // Attempt to get clipboard content
                let mut clipboard = match arboard::Clipboard::new() {
                    Ok(c) => c,
                    Err(_) => continue, // Silently fail on unsupported platforms
                };

                let content = match clipboard.get_text() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                if last_content.as_ref() != Some(&content) {
                    // Truncate long clipboard content
                    let truncated = if content.len() > 500 {
                        format!(
                            "{}... [truncated, {} chars total]",
                            &content[..500],
                            content.len()
                        )
                    } else {
                        content.clone()
                    };

                    last_content = Some(content.clone());

                    let entry = LogEntry {
                        timestamp: Utc::now(),
                        event_type: "clipboard".to_string(),
                        data: json!({
                            "content": truncated,
                            "length": content.len(),
                        }),
                    };

                    eprintln!("\n[CLIP] Content changed ({} chars)", content.len());
                    let _ = tx.send(entry);
                }
            }
        });

        Ok(())
    }

    /// Screenshot capture
    async fn capture_screenshots(&self, tx: mpsc::UnboundedSender<LogEntry>) -> Result<()> {
        let running = self.running.clone();
        let interval_secs = self.screenshot_interval_secs;

        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(interval_secs));
            timer.tick().await; // Skip first immediate tick

            loop {
                timer.tick().await;

                if !running.load(Ordering::Relaxed) {
                    break;
                }

                match capture_screenshot() {
                    Ok(info) => {
                        let entry = LogEntry {
                            timestamp: Utc::now(),
                            event_type: "screenshot".to_string(),
                            data: json!({
                                "path": info.path,
                                "width": info.width,
                                "height": info.height,
                                "size": info.size,
                            }),
                        };
                        eprintln!("\n[SHOT] Screenshot saved: {} ({}x{})", info.path, info.width, info.height);
                        let _ = tx.send(entry);
                    }
                    Err(e) => {
                        // Screenshot is optional - log but don't crash
                        eprintln!("\n[WARN] Screenshot failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Global keyboard trigger listener
    async fn watch_keyboard(&self) -> Result<()> {
        let running = self.running.clone();
        let trigger_keys = self.trigger_keys.clone();

        tokio::task::spawn_blocking(move || {
            let mut pressed_keys: Vec<Key> = Vec::new();

            eprintln!("[HOOK] Keyboard hook active - Press Ctrl+Shift+X to trigger");

            let callback = move |event: Event| {
                // Check if we should stop
                if !running.load(Ordering::Relaxed) {
                    std::process::exit(0);
                }

                match event.event_type {
                    EventType::KeyPress(key) => {
                        // Add key if not already pressed
                        if !pressed_keys.contains(&key) {
                            pressed_keys.push(key);

                            // Check if all trigger keys are pressed
                            if trigger_keys.iter().all(|k| pressed_keys.contains(k)) {
                                eprintln!("\n[TRIG] Manual trigger activated!");
                                // Note: For proper event logging, we'd need a channel that works
                                // from blocking contexts. For now, just print the message.
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        // Remove the released key
                        pressed_keys.retain(|k| *k != key);
                    }
                    _ => {}
                }
            };

            if let Err(e) = rdev::listen(callback) {
                eprintln!("Keyboard hook error: {:?}", e);
            }
        });

        Ok(())
    }

    /// Main run loop with CLI limits
    async fn run(&mut self) -> Result<()> {
        eprintln!("EX-G-SE Core Engine v4.0.0 - Ghost Mode");
        eprintln!("========================================");
        eprintln!("[INFO] Starting shadow logging session...");

        let (tx, mut rx) = mpsc::unbounded_channel();

        // Start all watchers
        self.watch_fs(tx.clone()).await?;
        self.watch_clipboard(tx.clone()).await?;
        self.capture_screenshots(tx.clone()).await?;
        self.watch_keyboard().await?;

        eprintln!("[INFO] All monitors active - monitoring current directory");
        eprintln!("[INFO] Press Ctrl+C to stop and save logs");
        eprintln!();

        // Wait for shutdown signal with improved error handling
        let ctrl_c_running = self.running.clone();
        let ctrl_c_tx = tx.clone();

        tokio::spawn(async move {
            match ctrl_c().await {
                Ok(()) => {
                    eprintln!("\n[INFO] Shutdown signal received...");
                    ctrl_c_running.store(false, Ordering::Relaxed);
                    // Send a dummy event to unblock recv()
                    let _ = ctrl_c_tx.send(LogEntry {
                        timestamp: Utc::now(),
                        event_type: "_shutdown".to_string(),
                        data: json!({}),
                    });
                }
                Err(e) => {
                    eprintln!("\n[ERROR] Signal handler error: {:?}", e);
                    ctrl_c_running.store(false, Ordering::Relaxed);
                }
            }
        });

        // Collect events with CLI limits
        let start_time = Utc::now();
        let mut last_stats_time = start_time;

        while self.running.load(Ordering::Relaxed) {
            match rx.recv().await {
                Some(entry) => {
                    // Filter out shutdown events
                    if entry.event_type == "_shutdown" {
                        break;
                    }

                    self.events.push(entry);

                    // Check CLI limits
                    let elapsed_secs = (Utc::now() - start_time).num_seconds();
                    if self.cli_config.should_stop(elapsed_secs, self.events.len()) {
                        eprintln!("\n[CLI] Limit reached, stopping session...");
                        self.running.store(false, Ordering::Relaxed);
                        break;
                    }

                    // Show progress every 30 seconds
                    if elapsed_secs > 0 && elapsed_secs % 30 == 0 && elapsed_secs != (last_stats_time - start_time).num_seconds() {
                        last_stats_time = Utc::now();
                        let clip_count = self.events.iter().filter(|e| e.event_type == "clipboard").count();
                        let fs_count = self.events.iter().filter(|e| e.event_type == "fs_change").count();
                        let screenshot_count = self.events.iter().filter(|e| e.event_type == "screenshot").count();
                        let duration = format_duration(elapsed_secs.try_into().unwrap());

                        eprint!("\r[EX-G-SE] Recording... ({:<15} | Events: {:>4} | FS: {:>3} | Clips: {:>3} | Shots: {:>3})",
                            duration,
                            self.events.len(),
                            fs_count,
                            clip_count,
                            screenshot_count
                        );
                    }
                }
                None => {
                    // Channel closed, exit loop
                    eprintln!("\n[INFO] Event channel closed");
                    break;
                }
            }
        }

        // Save logs on shutdown with error handling
        eprintln!("\n[INFO] Saving session...");
        match self.save_logs() {
            Ok(()) => {
                // Already shows success message
            }
            Err(e) => {
                eprintln!("\n[ERROR] Failed to save session: {}", e);
                eprintln!("[ERROR] Session data may be lost!");
            }
        }

        // Show detailed session summary
        let end_time = Utc::now();
        let duration_secs = (end_time - start_time).num_seconds();
        let clip_count = self.events.iter().filter(|e| e.event_type == "clipboard").count();
        let fs_count = self.events.iter().filter(|e| e.event_type == "fs_change").count();
        let screenshot_count = self.events.iter().filter(|e| e.event_type == "screenshot").count();

        eprintln!("\n📊 Session Summary:");
        eprintln!("   ⏱️  Duration: {}", format_duration(duration_secs.try_into().unwrap()));
        eprintln!("   📁 Total Events: {}", self.events.len());
        eprintln!("   📋 Clipboard Changes: {}", clip_count);
        eprintln!("   📂 File Changes: {}", fs_count);
        eprintln!("   🖼️  Screenshots: {}", screenshot_count);
        eprintln!("\n{}", "=".repeat(60));
        eprintln!("\n⏸️  Press ENTER to exit...");

        // Wait for user to press Enter
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(())
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let mut engine = ExGSeEngine::new();
    engine.run().await?;

    eprintln!("\n[INFO] Session complete. Goodbye!");

    Ok(())
}

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: "test".to_string(),
            data: json!({"key": "value"}),
        };

        let json_result = serde_json::to_string(&entry);
        assert!(json_result.is_ok());
        let json = json_result.unwrap();
        assert!(json.contains("test"));
    }

    #[test]
    fn test_session_logs_serialization() {
        let session = SessionLogs {
            start_time: Utc::now(),
            end_time: None,
            events: vec![],
        };

        let json_result = serde_json::to_string(&session);
        assert!(json_result.is_ok());
        let json = json_result.unwrap();
        assert!(json.contains("start"));
        assert!(json.contains("events"));
    }
}
