// CLAUDE CODE CONTEXT READER
//
// Reads Claude Code conversation history from:
// - ~/.claude/history.jsonl (global history)
// - ~/.claude/projects/{encoded-project}/{sessionId}.jsonl (per-project sessions)
//
// This gives us access to:
// - User prompts
// - Assistant responses
// - Tool calls (Read, Edit, Bash, etc.)
// - Hook events (PreToolUse, PostToolUse)

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Entry from ~/.claude/history.jsonl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeHistoryEntry {
    #[serde(rename = "display")]
    pub display_text: String,

    #[serde(default)]
    pub pasted_contents: HashMap<String, PastContent>,

    #[serde(rename = "timestamp")]
    pub timestamp_ms: i64,

    #[serde(default)]
    pub session_id: Option<String>,

    #[serde(default)]
    pub project: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastContent {
    pub id: u64,
    #[serde(rename = "type")]
    pub content_type: String,
    pub content: String,
}

/// Entry from ~/.claude/projects/{project}/{sessionId}.jsonl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSessionEntry {
    #[serde(rename = "type")]
    pub entry_type: String,

    pub timestamp: String,

    #[serde(default)]
    pub cwd: String,

    pub session_id: String,

    #[serde(default)]
    pub git_branch: Option<String>,

    #[serde(default)]
    pub slug: Option<String>,

    pub data: SessionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    #[serde(rename = "type")]
    pub data_type: String,

    #[serde(default)]
    pub hook_event: Option<String>,

    #[serde(default)]
    pub hook_name: Option<String>,

    #[serde(default)]
    pub command: Option<String>,

    #[serde(rename = "parentToolUseID")]
    pub parent_tool_use_id: Option<String>,

    #[serde(rename = "toolUseID")]
    pub tool_use_id: Option<String>,
}

/// Tool call extracted from session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub timestamp: String,
    pub tool_name: String,
    pub description: String,
    #[serde(default)]
    pub details: HashMap<String, String>,
}

/// Reader for Claude Code context
pub struct ClaudeCodeReader {
    claude_dir: PathBuf,
}

impl ClaudeCodeReader {
    /// Create new reader, defaults to ~/.claude
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().context("Cannot find home directory")?;
        let claude_dir = home.join(".claude");

        if !claude_dir.exists() {
            return Ok(Self { claude_dir });
        }

        Ok(Self { claude_dir })
    }

    /// Read global history in time range
    pub fn read_history_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<ClaudeHistoryEntry>> {
        let history_path = self.claude_dir.join("history.jsonl");

        if !history_path.exists() {
            eprintln!("[CLAUDE] No history.jsonl found");
            return Ok(Vec::new());
        }

        let file = File::open(&history_path)
            .with_context(|| format!("Cannot open history file: {:?}", history_path))?;

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        let start_ms = start.timestamp_millis();
        let end_ms = end.timestamp_millis();

        for line in reader.lines() {
            let line = line.with_context(|| "Failed to read history line")?;

            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<ClaudeHistoryEntry>(&line) {
                Ok(entry) => {
                    // Filter by timestamp
                    if entry.timestamp_ms >= start_ms && entry.timestamp_ms <= end_ms {
                        entries.push(entry);
                    }
                }
                Err(e) => {
                    // Skip invalid lines but don't fail
                    eprintln!("[CLAUDE] Failed to parse history entry: {}", e);
                }
            }
        }

        eprintln!("[CLAUDE] Loaded {} history entries in range", entries.len());
        Ok(entries)
    }

    /// Read project-specific session
    pub fn read_project_session(
        &self,
        project_path: &str,
        session_id: &str,
    ) -> Result<Vec<ClaudeSessionEntry>> {
        // Encode project path like Claude does: replace \ with -, remove :,
        // and encode special chars
        let encoded_path = project_path
            .replace('\\', "/")
            .replace(':', "")
            .replace('/', "-");

        let session_path = self
            .claude_dir
            .join("projects")
            .join(&encoded_path)
            .join(format!("{}.jsonl", session_id));

        if !session_path.exists() {
            eprintln!("[CLAUDE] Session file not found: {:?}", session_path);
            return Ok(Vec::new());
        }

        let file = File::open(&session_path)
            .with_context(|| format!("Cannot open session file: {:?}", session_path))?;

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line.with_context(|| "Failed to read session line")?;

            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<ClaudeSessionEntry>(&line) {
                Ok(entry) => {
                    entries.push(entry);
                }
                Err(e) => {
                    eprintln!("[CLAUDE] Failed to parse session entry: {}", e);
                }
            }
        }

        eprintln!(
            "[CLAUDE] Loaded {} session entries for {}",
            entries.len(),
            session_id
        );
        Ok(entries)
    }

    /// Extract user prompts from history entries
    pub fn extract_user_prompts(&self, entries: &[ClaudeHistoryEntry]) -> Vec<PromptEntry> {
        let mut prompts = Vec::new();

        for entry in entries {
            // Filter out hook errors and system messages
            if entry.display_text.contains("hook error")
                || entry.display_text.contains("PostToolUse")
                || entry.display_text.contains("PreToolUse")
            {
                continue;
            }

            // Skip if it's just a statusline or other noise
            if entry.display_text.len() < 5 {
                continue;
            }

            let timestamp =
                DateTime::<Utc>::from_timestamp_millis(entry.timestamp_ms).unwrap_or_default();

            prompts.push(PromptEntry {
                timestamp: timestamp.to_rfc3339(),
                text: entry.display_text.clone(),
                session_id: entry.session_id.clone(),
                project: entry.project.clone(),
            });
        }

        eprintln!("[CLAUDE] Extracted {} user prompts", prompts.len());
        prompts
    }

    /// Extract assistant responses from history
    pub fn extract_assistant_responses(
        &self,
        entries: &[ClaudeHistoryEntry],
    ) -> Vec<ResponseEntry> {
        let mut responses = Vec::new();

        // Look for pasted contents which often contain assistant responses
        for entry in entries {
            for (id, content) in &entry.pasted_contents {
                if content.content_type == "text" && content.content.len() > 50 {
                    let timestamp =
                        DateTime::<Utc>::from_timestamp_millis(entry.timestamp_ms)
                            .unwrap_or_default();

                    responses.push(ResponseEntry {
                        timestamp: timestamp.to_rfc3339(),
                        content: content.content.clone(),
                        id: id.clone(),
                    });
                }
            }
        }

        eprintln!("[CLAUDE] Extracted {} assistant responses", responses.len());
        responses
    }

    /// Extract tool calls from session entries
    pub fn extract_tool_calls(&self, entries: &[ClaudeSessionEntry]) -> Vec<ToolCall> {
        let mut calls = Vec::new();

        for entry in entries {
            // Skip hook errors per user request
            if entry
                .data
                .hook_name
                .as_ref()
                .map(|h| h.contains("error"))
                .unwrap_or(false)
            {
                continue;
            }

            // Extract tool name from hook_name or command
            let tool_name = if let Some(hook_name) = &entry.data.hook_name {
                // Extract from "PreToolUse:Read", "PostToolUse:Bash", etc.
                hook_name
                    .split(':')
                    .last()
                    .unwrap_or("unknown")
                    .to_string()
            } else if let Some(command) = &entry.data.command {
                // Extract from command string
                if command.contains("hooks/") {
                    command
                        .split('/')
                        .last()
                        .and_then(|s| s.split('.').next())
                        .unwrap_or("unknown")
                        .to_string()
                } else {
                    "command".to_string()
                }
            } else {
                entry.data.data_type.clone()
            };

            let description = if let Some(hook_name) = &entry.data.hook_name {
                hook_name.clone()
            } else if let Some(command) = &entry.data.command {
                command.clone()
            } else {
                format!("{:?}", entry.data)
            };

            calls.push(ToolCall {
                timestamp: entry.timestamp.clone(),
                tool_name,
                description,
                details: HashMap::new(),
            });
        }

        eprintln!("[CLAUDE] Extracted {} tool calls (excluding hook errors)", calls.len());
        calls
    }

    /// Get Claude Code settings
    pub fn get_settings(&self) -> Result<Option<ClaudeSettings>> {
        let settings_path = self.claude_dir.join("settings.json");

        if !settings_path.exists() {
            return Ok(None);
        }

        let file = File::open(&settings_path)
            .with_context(|| format!("Cannot open settings file: {:?}", settings_path))?;

        let settings: ClaudeSettings = serde_json::from_reader(file)
            .with_context(|| "Failed to parse settings.json")?;

        Ok(Some(settings))
    }
}

impl Default for ClaudeCodeReader {
    fn default() -> Self {
        Self::new().expect("Cannot create ClaudeCodeReader")
    }
}

/// User prompt entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptEntry {
    pub timestamp: String,
    pub text: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub project: Option<String>,
}

/// Assistant response entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEntry {
    pub timestamp: String,
    pub content: String,
    pub id: String,
}

/// Claude Code settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSettings {
    #[serde(rename = "fontSize")]
    pub font_size: u32,

    #[serde(default)]
    pub theme: String,

    #[serde(default)]
    pub provider: Option<String>,
}
