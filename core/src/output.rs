use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutputEvent {
    #[serde(rename = "fs")]
    FileSystem {
        timestamp: DateTime<Utc>,
        event: String,
        path: String,
        size: Option<u64>,
    },
    #[serde(rename = "clipboard")]
    Clipboard {
        timestamp: DateTime<Utc>,
        content: String,
        length: usize,
        hash: String,
    },
    #[serde(rename = "screenshot")]
    Screenshot {
        timestamp: DateTime<Utc>,
        path: String,
        width: u32,
        height: u32,
        size: u64,
    },
    #[serde(rename = "keyboard")]
    Keyboard {
        timestamp: DateTime<Utc>,
        action: String,
        key: String,
        modifiers: Vec<String>,
    },
}
