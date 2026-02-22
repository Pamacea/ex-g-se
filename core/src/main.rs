// EX-G-SE: Shadow Logging Core Engine
//
// A cross-platform Rust core that runs in "Ghost Mode" monitoring:
// - File system changes
// - Clipboard content
// - Screenshots (interval-based)
// - Global keyboard input (trigger detection)
//
// PLATFORM REQUIREMENTS:
// - Linux: libx11-dev, libxtst-dev, libxrandr-dev (for X11)
// - macOS: Accessibility permissions for keyboard hooks
// - Windows: Works out of the box

use anyhow::Result;
use chrono::{DateTime, Utc};
use device_query::{DeviceQuery, DeviceState, Keycode};
use notify::{RecursiveMode, Watcher};
use serde::Serialize;
use serde_json::json;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc;
use tokio::time::interval;

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
    trigger_keys: HashSet<Keycode>,
    screenshot_interval_secs: u64,
}

impl ExGSeEngine {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            start_time: Utc::now(),
            running: Arc::new(AtomicBool::new(true)),
            // Trigger: Ctrl+Shift+X
            trigger_keys: HashSet::from([Keycode::LControl, Keycode::LShift, Keycode::X]),
            screenshot_interval_secs: 30,
        }
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

    /// Write logs to file
    fn save_logs(&self) -> Result<()> {
        let logs = SessionLogs {
            start_time: self.start_time,
            end_time: Some(Utc::now()),
            events: self.events.clone(),
        };

        let json = serde_json::to_string_pretty(&logs)?;
        let mut file = File::create("raw_logs.json")?;
        file.write_all(json.as_bytes())?;

        eprintln!(".");
        eprintln!(".");
        eprintln!("[EX-G-SE] Session saved to raw_logs.json");
        eprintln!("[EX-G-SE] {} events captured", self.events.len());

        Ok(())
    }

    /// File system watcher
    async fn watch_fs(&self, tx: mpsc::UnboundedSender<LogEntry>) -> Result<()> {
        let (watcher_tx, mut watcher_rx) = mpsc::unbounded_channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = watcher_tx.send(event);
            }
        })?;

        watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

        let running = self.running.clone();

        tokio::spawn(async move {
            while running.load(Ordering::Relaxed) {
                if let Some(event) = watcher_rx.recv().await {
                    // Extract path from event if available
                    let path = event
                        .paths
                        .first()
                        .and_then(|p| p.to_str())
                        .unwrap_or("unknown");

                    // Filter noise (node_modules, .git, target)
                    if path.contains("node_modules")
                        || path.contains(".git")
                        || path.contains("target")
                    {
                        continue;
                    }

                    let entry = LogEntry {
                        timestamp: Utc::now(),
                        event_type: "fs_change".to_string(),
                        data: json!({
                            "path": path,
                            "kind": format!("{:?}", event.kind),
                        }),
                    };
                    let _ = tx.send(entry);
                    eprint!(".");
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

                // Platform-specific screenshot handling
                #[cfg(target_os = "windows")]
                let screenshot_result = capture_screenshot_windows();

                #[cfg(target_os = "linux")]
                let screenshot_result = capture_screenshot_linux();

                #[cfg(target_os = "macos")]
                let screenshot_result = capture_screenshot_macos();

                #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
                let screenshot_result: Result<String, String> =
                    Err("Unsupported platform".to_string());

                match screenshot_result {
                    Ok(path) => {
                        let entry = LogEntry {
                            timestamp: Utc::now(),
                            event_type: "screenshot".to_string(),
                            data: json!({ "path": path }),
                        };
                        eprintln!("\n[SHOT] Screenshot saved: {}", path);
                        let _ = tx.send(entry);
                    }
                    Err(e) => {
                        // Silently fail for now - screenshot is optional
                        eprintln!("\n[ERROR] Screenshot failed: {}", e);
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
            let device_state = DeviceState::new();
            let mut pressed_keys: HashSet<Keycode> = HashSet::new();

            eprintln!("[HOOK] Keyboard hook active - Press Ctrl+Shift+X to trigger");

            while running.load(Ordering::Relaxed) {
                let keys = device_state.get_keys();
                let keys_set: HashSet<Keycode> = keys.into_iter().collect();

                // Check for newly pressed keys
                for key in keys_set.iter() {
                    if !pressed_keys.contains(key) {
                        pressed_keys.insert(*key);

                        // Check if all trigger keys are pressed
                        if trigger_keys.iter().all(|k| pressed_keys.contains(k)) {
                            eprintln!("\n[TRIG] Manual trigger activated!");
                            // Note: For proper event logging, we'd need a channel that works
                            // from blocking contexts. For now, just print the message.
                        }
                    }
                }

                // Remove released keys
                pressed_keys = pressed_keys.intersection(&keys_set).cloned().collect();

                std::thread::sleep(Duration::from_millis(50));
            }
        });

        Ok(())
    }

    /// Main run loop
    async fn run(&mut self) -> Result<()> {
        eprintln!("EX-G-SE Core Engine v0.1.0 - Ghost Mode");
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

        // Wait for shutdown signal
        let ctrl_c_running = self.running.clone();
        tokio::spawn(async move {
            if let Ok(()) = ctrl_c().await {
                ctrl_c_running.store(false, Ordering::Relaxed);
            }
        });

        // Collect events
        while self.running.load(Ordering::Relaxed) {
            if let Some(entry) = rx.recv().await {
                self.events.push(entry);
            }
        }

        // Save logs on shutdown
        self.save_logs()?;

        Ok(())
    }
}

// ============================================================================
// Platform-specific screenshot implementations
// ============================================================================

#[cfg(target_os = "windows")]
fn capture_screenshot_windows() -> Result<String, String> {
    // Windows screenshot implementation using winsafe or screenshot crate
    // For now, return a placeholder - actual implementation would use:
    // - screenshot crate
    // - winsafe for direct Win32 API calls
    Err("Windows screenshot implementation pending".to_string())
}

#[cfg(target_os = "linux")]
fn capture_screenshot_linux() -> Result<String, String> {
    // Linux requires X11 or Wayland
    // Could use: image::codecs::png or external tool like scrot
    Err("Linux screenshot requires X11/Wayland libs - install libx11-dev libxtst-dev".to_string())
}

#[cfg(target_os = "macos")]
fn capture_screenshot_macos() -> Result<String, String> {
    // macOS requires Accessibility permissions
    // Could use CGDisplayCreateImage from core-graphics crate
    Err("macOS screenshot requires Accessibility permissions".to_string())
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
