//! Session Analyzer - Detects intents and key moments from raw logs
//!
//! This module analyzes raw session logs to:
//! - Detect developer intent (bug fixing, feature development, etc.)
//! - Identify key moments in the session
//! - Recognize patterns in code changes

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::watchers::LogEntry;

/// Developer intent detected from session activity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Intent {
    /// Creating new functionality
    FeatureDevelopment,
    /// Debugging and fixing errors
    BugFixing,
    /// Improving existing code without changing behavior
    Refactoring,
    /// Writing documentation or comments
    Documentation,
    /// Writing or running tests
    Testing,
    /// Exploring new code or libraries
    Learning,
    /// Deploying or publishing code
    Deployment,
    /// Setup and configuration work
    Configuration,
}

/// A key moment in the development session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMoment {
    pub timestamp: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub intent: Intent,
    pub screenshot_path: Option<String>,
    pub file_changes: Vec<FileChange>,
    pub clipboard_content: Option<String>,
}

/// A file change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub action: FileAction,
    pub timestamp: DateTime<Utc>,
}

/// Type of file action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileAction {
    Created,
    Modified,
    Deleted,
}

/// A recognized pattern in the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub occurrences: usize,
}

/// Complete session analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalysis {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub intents: Vec<DetectedIntent>,
    pub key_moments: Vec<KeyMoment>,
    pub patterns: Vec<Pattern>,
    pub summary: String,
    pub developer: Option<String>,
}

/// A detected intent with timeframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIntent {
    pub intent: Intent,
    pub confidence: f32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

/// Session analyzer
pub struct SessionAnalyzer {
    min_confidence: f32,
}

impl SessionAnalyzer {
    pub fn new() -> Self {
        Self {
            min_confidence: 0.6,
        }
    }

    /// Analyze raw session logs
    pub fn analyze(&self, logs: &[LogEntry]) -> Result<SessionAnalysis> {
        if logs.is_empty() {
            return Ok(SessionAnalysis {
                session_id: uuid::Uuid::new_v4().to_string(),
                start_time: Utc::now(),
                end_time: Utc::now(),
                intents: vec![],
                key_moments: vec![],
                patterns: vec![],
                summary: "Empty session".to_string(),
                developer: None,
            });
        }

        let start_time = logs.first().map(|e| e.timestamp).unwrap_or_else(Utc::now);
        let end_time = logs.last().map(|e| e.timestamp).unwrap_or_else(Utc::now);

        // Detect intents from activity patterns
        let intents = self.detect_intents(logs)?;

        // Identify key moments
        let key_moments = self.identify_key_moments(logs, &intents)?;

        // Recognize patterns
        let patterns = self.recognize_patterns(logs)?;

        // Generate summary
        let summary = self.generate_summary(&key_moments, &intents);

        Ok(SessionAnalysis {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time,
            end_time,
            intents,
            key_moments,
            patterns,
            summary,
            developer: None, // Could be detected from git config
        })
    }

    /// Detect intents from activity patterns
    fn detect_intents(&self, logs: &[LogEntry]) -> Result<Vec<DetectedIntent>> {
        let mut intents = Vec::new();
        let mut current_intent: Option<(Intent, DateTime<Utc>)> = None;
        let mut intent_events = Vec::new();

        for log in logs {
            let detected_intent = self.detect_intent_from_event(log);

            match &mut current_intent {
                Some((intent, start)) if intent == &detected_intent => {
                    // Continue current intent
                    intent_events.push(log.timestamp);
                }
                Some((intent, start)) => {
                    // End current intent, start new one
                    let confidence = self.calculate_confidence(&intent_events);
                    if confidence >= self.min_confidence {
                        intents.push(DetectedIntent {
                            intent: intent.clone(),
                            confidence,
                            start_time: *start,
                            end_time: log.timestamp,
                        });
                    }
                    current_intent = Some((detected_intent, log.timestamp));
                    intent_events.clear();
                }
                None => {
                    current_intent = Some((detected_intent, log.timestamp));
                    intent_events.push(log.timestamp);
                }
            }
        }

        // Don't forget the last intent
        if let Some((intent, start)) = current_intent {
            let confidence = self.calculate_confidence(&intent_events);
            if confidence >= self.min_confidence {
                let end = logs.last().map(|l| l.timestamp).unwrap_or_else(Utc::now);
                intents.push(DetectedIntent {
                    intent,
                    confidence,
                    start_time: start,
                    end_time: end,
                });
            }
        }

        Ok(intents)
    }

    /// Detect intent from a single event
    fn detect_intent_from_event(&self, log: &LogEntry) -> Intent {
        match log.event_type.as_str() {
            "fs_change" => {
                // Analyze file path and change type
                if let Some(path) = log.data.get("path").and_then(|p| p.as_str()) {
                    if path.contains("test") || path.contains("spec") {
                        return Intent::Testing;
                    }
                    if path.contains("doc") || path.ends_with(".md") {
                        return Intent::Documentation;
                    }
                    if path.contains("config") || path.ends_with(".json") || path.ends_with(".yaml") {
                        return Intent::Configuration;
                    }
                }

                // Check action kind for more context
                if let Some(kind) = log.data.get("kind") {
                    let kind_str = kind.to_string();
                    if kind_str.contains("remove") || kind_str.contains("delete") {
                        // Could be refactoring or bug fixing
                        return Intent::Refactoring;
                    }
                }

                Intent::FeatureDevelopment
            }
            "clipboard" => {
                // Analyze clipboard content
                if let Some(content) = log.data.get("content").and_then(|c| c.as_str()) {
                    if content.contains("error")
                        || content.contains("Error")
                        || content.contains("panic")
                        || content.contains("exception")
                    {
                        return Intent::BugFixing;
                    }
                    if content.contains("test") || content.contains("assert") {
                        return Intent::Testing;
                    }
                    if content.contains("deploy") || content.contains("publish") {
                        return Intent::Deployment;
                    }
                }
                Intent::FeatureDevelopment
            }
            "screenshot" => Intent::FeatureDevelopment,
            _ => Intent::FeatureDevelopment,
        }
    }

    /// Calculate confidence score for an intent
    fn calculate_confidence(&self, events: &[DateTime<Utc>]) -> f32 {
        if events.is_empty() {
            return 0.0;
        }

        // More events = higher confidence
        let base_confidence = (events.len() as f32 / 10.0).min(1.0);

        // Consider time span
        if events.len() > 1 {
            let duration = *events.last().unwrap() - *events.first().unwrap();
            let duration_minutes = duration.num_seconds() as f32 / 60.0;

            // Longer duration with consistent activity = higher confidence
            let duration_factor = (duration_minutes / 5.0).min(1.0);

            (base_confidence + duration_factor) / 2.0
        } else {
            base_confidence
        }
    }

    /// Identify key moments from the session
    fn identify_key_moments(
        &self,
        logs: &[LogEntry],
        intents: &[DetectedIntent],
    ) -> Result<Vec<KeyMoment>> {
        let mut key_moments = Vec::new();

        // Find intent transitions (key moments when intent changes)
        for (i, intent) in intents.iter().enumerate() {
            if i == 0 || intents[i - 1].intent != intent.intent {
                key_moments.push(KeyMoment {
                    timestamp: intent.start_time,
                    title: format!("Started: {:?}", intent.intent),
                    description: format!("Developer began working on: {:?}", intent.intent),
                    intent: intent.intent.clone(),
                    screenshot_path: None, // Would be correlated with screenshot events
                    file_changes: vec![],
                    clipboard_content: None,
                });
            }
        }

        // Find clusters of activity (bursts of file changes)
        let mut activity_cluster = Vec::new();
        let mut last_log_time: Option<DateTime<Utc>> = None;

        for log in logs {
            if log.event_type == "fs_change" {
                activity_cluster.push(log);

                // If 2 minutes passed without activity, consider it a key moment
                if let Some(last_time) = last_log_time {
                    let elapsed = log.timestamp.signed_duration_since(last_time).num_seconds().abs() / 60;
                    if elapsed > 2 && !activity_cluster.is_empty() {
                        // Key moment: burst of activity followed by pause
                        key_moments.push(KeyMoment {
                            timestamp: activity_cluster.first().unwrap().timestamp,
                            title: "Activity Burst".to_string(),
                            description: format!(
                                "{} files modified in quick succession",
                                activity_cluster.len()
                            ),
                            intent: Intent::FeatureDevelopment, // Default
                            screenshot_path: None,
                            file_changes: activity_cluster
                                .iter()
                                .filter_map(|l| {
                                    l.data.get("path").and_then(|p| p.as_str()).map(|path| FileChange {
                                        path: path.to_string(),
                                        action: FileAction::Modified, // Simplified
                                        timestamp: l.timestamp,
                                    })
                                })
                                .collect(),
                            clipboard_content: None,
                        });
                        activity_cluster.clear();
                    }
                }
                last_log_time = Some(log.timestamp);
            }
        }

        // Sort by timestamp
        key_moments.sort_by_key(|k| k.timestamp);

        Ok(key_moments)
    }

    /// Recognize patterns in the session
    fn recognize_patterns(&self, logs: &[LogEntry]) -> Result<Vec<Pattern>> {
        let mut patterns = Vec::new();
        let mut pattern_counts: HashMap<String, usize> = HashMap::new();

        // Count file extensions
        for log in logs {
            if log.event_type == "fs_change" {
                if let Some(path) = log.data.get("path").and_then(|p| p.as_str()) {
                    if let Some(ext) = path.split('.').last() {
                        *pattern_counts
                            .entry(format!("Working with .{} files", ext))
                            .or_insert(0) += 1;
                    }
                }
            }
        }

        // Convert to patterns
        for (name, occurrences) in pattern_counts {
            if occurrences >= 3 {
                // Only include if seen at least 3 times
                patterns.push(Pattern {
                    name: name.clone(),
                    description: format!("Frequently worked with {}", name),
                    occurrences,
                });
            }
        }

        Ok(patterns)
    }

    /// Generate a human-readable summary
    fn generate_summary(&self, key_moments: &[KeyMoment], intents: &[DetectedIntent]) -> String {
        if key_moments.is_empty() {
            return "No significant activity detected".to_string();
        }

        let mut summary = String::from("Session Summary:\n");

        // Add intents
        if !intents.is_empty() {
            summary.push_str("\nIntents:\n");
            for intent in intents {
                summary.push_str(&format!(
                    "- {:?} (confidence: {:.0}%)\n",
                    intent.intent,
                    intent.confidence * 100.0
                ));
            }
        }

        // Add key moments
        if !key_moments.is_empty() {
            summary.push_str(&format!("\nKey Moments ({} total):\n", key_moments.len()));
            for (i, moment) in key_moments.iter().take(5).enumerate() {
                summary.push_str(&format!(
                    "{}. {}\n",
                    i + 1,
                    moment.title
                ));
            }
        }

        summary
    }
}

impl Default for SessionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_detect_intent_bug_fixing() {
        let analyzer = SessionAnalyzer::new();
        let log = LogEntry {
            timestamp: Utc::now(),
            event_type: "clipboard".to_string(),
            data: json!({"content": "TypeError: Cannot read property 'id'"}),
        };

        let intent = analyzer.detect_intent_from_event(&log);
        assert_eq!(intent, Intent::BugFixing);
    }

    #[test]
    fn test_detect_intent_testing() {
        let analyzer = SessionAnalyzer::new();
        let log = LogEntry {
            timestamp: Utc::now(),
            event_type: "fs_change".to_string(),
            data: json!({"path": "./test/user.test.js"}),
        };

        let intent = analyzer.detect_intent_from_event(&log);
        assert_eq!(intent, Intent::Testing);
    }

    #[test]
    fn test_analyze_empty_session() {
        let analyzer = SessionAnalyzer::new();
        let analysis = analyzer.analyze(&[]).unwrap();

        assert_eq!(analysis.key_moments.len(), 0);
        assert_eq!(analysis.intents.len(), 0);
    }
}
