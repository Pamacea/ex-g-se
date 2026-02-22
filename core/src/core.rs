// Core engine implementation

use crate::watchers::LogEntry;
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// EX-G-SE Core Engine
pub struct ExGSeEngine {
    pub events: Vec<LogEntry>,
    pub start_time: DateTime<Utc>,
    pub running: Arc<AtomicBool>,
}

impl ExGSeEngine {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            start_time: Utc::now(),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Record an event
    pub fn log_event(&mut self, event_type: &str, data: serde_json::Value) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            data,
        };
        self.events.push(entry);
    }

    /// Shutdown the engine
    pub fn shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Check if engine is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

impl Default for ExGSeEngine {
    fn default() -> Self {
        Self::new()
    }
}
