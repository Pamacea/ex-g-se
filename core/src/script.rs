//! Script Generator - Creates theater-play format scripts from session analysis
//!
//! This module generates human-readable scripts that capture:
//! - What the developer was thinking
//! - Why they made certain choices
//! - Code notes and decisions
//! - Timeline with screenshots

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::analyzer::{KeyMoment, SessionAnalysis};

/// Generated script in theater play format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub title: String,
    pub duration_minutes: i64,
    pub acts: Vec<Act>,
    pub metadata: ScriptMetadata,
}

/// An act is a major phase of the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub number: usize,
    pub title: String,
    pub time_range: (String, String), // ISO 8601 timestamps
    pub intent: String,
    pub scenes: Vec<Scene>,
}

/// A scene is a specific moment within an act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub number: usize,
    pub timestamp: String,
    pub title: String,
    pub description: String,
    pub actions: Vec<Action>,
    pub dialogue: Vec<Dialogue>,
    pub screenshot: Option<String>,
    pub code_notes: Vec<CodeNote>,
}

/// An action taken by the developer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    FileChange {
        path: String,
        change: String,
        lines_added: usize,
        lines_removed: usize,
    },
    TerminalCommand {
        command: String,
        output: String,
    },
    ClipboardCopy {
        content: String,
    },
    Screenshot {
        path: String,
        width: u32,
        height: u32,
    },
    AICall {
        prompt: String,
        response: String,
    },
    Error {
        message: String,
        stack_trace: String,
    },
}

/// Dialogue represents thoughts or spoken words
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub speaker: String, // "DEVELOPER", "AI", "TERMINAL", "THOUGHT"
    pub line: String,
    pub timestamp: String,
}

/// Notes about code decisions and rationale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNote {
    pub location: String,    // File and line reference
    pub before: String,      // Code before change
    pub after: String,       // Code after change
    pub decision: String,    // What was decided
    pub rationale: String,   // Why this decision
    pub alternatives: Vec<String>, // Alternatives considered
}

/// Script metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMetadata {
    pub developer: Option<String>,
    pub date: String,
    pub intents: Vec<String>,
    pub key_moments_count: usize,
    pub total_changes: usize,
}

/// Timeline entry for video generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: String,
    pub duration_seconds: u32,
    pub title: String,
    pub description: String,
    pub screenshot: Option<String>,
    pub actions: Vec<TimelineAction>,
    pub voiceover: String,
}

/// Action for video timeline (with visual effects)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TimelineAction {
    Highlight {
        target: String,  // "line 42" or "function foo"
        duration: u32,   // seconds
    },
    Typewriter {
        text: String,
        duration: u32,
    },
    FadeOut {
        duration: u32,
    },
    Pan {
        direction: String, // "left", "right", "up", "down"
        duration: u32,
    },
    Zoom {
        level: f32, // 1.0 = no zoom, 2.0 = 2x zoom
        duration: u32,
    },
}

/// Script generator
pub struct ScriptGenerator;

impl ScriptGenerator {
    /// Generate a theater-play script from session analysis
    pub fn generate(analysis: &SessionAnalysis) -> Result<Script> {
        let duration_minutes = (analysis.end_time - analysis.start_time).num_minutes();
        let acts = Self::create_acts(analysis)?;
        let metadata = Self::create_metadata(analysis);

        let title = Self::generate_title(analysis);

        Ok(Script {
            title,
            duration_minutes,
            acts,
            metadata,
        })
    }

    /// Generate a timeline for video generation
    pub fn generate_timeline(analysis: &SessionAnalysis) -> Result<Vec<TimelineEntry>> {
        let mut timeline = Vec::new();

        for (i, moment) in analysis.key_moments.iter().enumerate() {
            let duration = if i < analysis.key_moments.len() - 1 {
                let next_moment = &analysis.key_moments[i + 1];
                (next_moment.timestamp - moment.timestamp).num_seconds() as u32
            } else {
                30 // Default 30 seconds for last moment
            };

            let actions = Self::create_timeline_actions(moment);

            timeline.push(TimelineEntry {
                timestamp: moment.timestamp.to_rfc3339(),
                duration_seconds: duration,
                title: moment.title.clone(),
                description: moment.description.clone(),
                screenshot: moment.screenshot_path.clone(),
                actions,
                voiceover: Self::generate_voiceover(moment),
            });
        }

        Ok(timeline)
    }

    fn create_acts(analysis: &SessionAnalysis) -> Result<Vec<Act>> {
        let mut acts = Vec::new();
        let mut scene_number = 1;

        // Group key moments by intent to create acts
        let mut intent_groups: std::collections::HashMap<String, Vec<&KeyMoment>> =
            std::collections::HashMap::new();

        for moment in &analysis.key_moments {
            let intent_label = format!("{:?}", moment.intent);
            intent_groups
                .entry(intent_label)
                .or_insert_with(Vec::new)
                .push(moment);
        }

        // Create acts from intent groups
        for (act_number, (intent, moments)) in intent_groups.into_iter().enumerate() {
            let start_time = moments
                .first()
                .map(|m| m.timestamp.to_rfc3339())
                .unwrap_or_else(|| analysis.start_time.to_rfc3339());
            let end_time = moments
                .last()
                .map(|m| m.timestamp.to_rfc3339())
                .unwrap_or_else(|| analysis.end_time.to_rfc3339());

            let mut scenes = Vec::new();
            for moment in moments {
                scenes.push(Self::create_scene(moment, scene_number)?);
                scene_number += 1;
            }

            acts.push(Act {
                number: act_number + 1,
                title: Self::generate_act_title(&intent),
                time_range: (start_time, end_time),
                intent: intent.clone(),
                scenes,
            });
        }

        // If no key moments, create a default act
        if acts.is_empty() {
            acts.push(Act {
                number: 1,
                title: "Development Session".to_string(),
                time_range: (
                    analysis.start_time.to_rfc3339(),
                    analysis.end_time.to_rfc3339(),
                ),
                intent: "Feature Development".to_string(),
                scenes: vec![],
            });
        }

        Ok(acts)
    }

    fn create_scene(moment: &KeyMoment, number: usize) -> Result<Scene> {
        let actions = Self::create_actions(moment);
        let dialogue = Self::create_dialogue(moment);
        let code_notes = Self::create_code_notes(moment);

        Ok(Scene {
            number,
            timestamp: moment.timestamp.to_rfc3339(),
            title: moment.title.clone(),
            description: moment.description.clone(),
            actions,
            dialogue,
            screenshot: moment.screenshot_path.clone(),
            code_notes,
        })
    }

    fn create_actions(moment: &KeyMoment) -> Vec<Action> {
        let mut actions = Vec::new();

        // Add file changes
        for change in &moment.file_changes {
            actions.push(Action::FileChange {
                path: change.path.clone(),
                change: format!("{:?}", change.action),
                lines_added: 0, // Would be extracted from actual diff
                lines_removed: 0,
            });
        }

        // Add screenshot if available
        if let Some(ref screenshot) = moment.screenshot_path {
            actions.push(Action::Screenshot {
                path: screenshot.clone(),
                width: 1920,  // Defaults
                height: 1080,
            });
        }

        // Add clipboard content if available
        if let Some(ref content) = moment.clipboard_content {
            actions.push(Action::ClipboardCopy {
                content: content.clone(),
            });
        }

        actions
    }

    fn create_dialogue(moment: &KeyMoment) -> Vec<Dialogue> {
        vec![
            Dialogue {
                speaker: "NARRATOR".to_string(),
                line: moment.description.clone(),
                timestamp: moment.timestamp.to_rfc3339(),
            },
            Dialogue {
                speaker: "DEVELOPER".to_string(),
                line: Self::generate_developer_thought(moment),
                timestamp: moment.timestamp.to_rfc3339(),
            },
        ]
    }

    fn create_code_notes(moment: &KeyMoment) -> Vec<CodeNote> {
        let mut notes = Vec::new();

        for change in &moment.file_changes {
            if change.path.ends_with(".rs")
                || change.path.ends_with(".js")
                || change.path.ends_with(".ts")
            {
                notes.push(CodeNote {
                    location: change.path.clone(),
                    before: "[Previous code]".to_string(),
                    after: "[New code]".to_string(),
                    decision: format!("{:?}", change.action),
                    rationale: "Implement feature requirements".to_string(),
                    alternatives: vec!["Alternative approach 1".to_string()],
                });
            }
        }

        notes
    }

    fn create_timeline_actions(moment: &KeyMoment) -> Vec<TimelineAction> {
        let mut actions = vec![];

        // Add highlight action for file changes
        if !moment.file_changes.is_empty() {
            if let Some(first_file) = moment.file_changes.first() {
                actions.push(TimelineAction::Highlight {
                    target: format!("{}:{}", first_file.path, "line 42"),
                    duration: 3,
                });
            }
        }

        // Add typewriter for title
        actions.push(TimelineAction::Typewriter {
            text: moment.title.clone(),
            duration: 2,
        });

        // Add fade out at end
        actions.push(TimelineAction::FadeOut { duration: 1 });

        actions
    }

    fn generate_voiceover(moment: &KeyMoment) -> String {
        format!(
            "At this moment, the developer is working on: {}. {}",
            moment.title, moment.description
        )
    }

    fn generate_developer_thought(moment: &KeyMoment) -> String {
        match moment.intent {
            crate::analyzer::Intent::BugFixing => {
                "Hmm, this isn't working. Let me debug this issue...".to_string()
            }
            crate::analyzer::Intent::FeatureDevelopment => {
                "Now I'll implement this new feature...".to_string()
            }
            crate::analyzer::Intent::Refactoring => {
                "This code could be cleaner. Let me refactor it...".to_string()
            }
            crate::analyzer::Intent::Testing => {
                "Let me verify this works with a test...".to_string()
            }
            _ => "Working on the code...".to_string(),
        }
    }

    fn generate_act_title(intent: &str) -> String {
        match intent {
            "BugFixing" => "The Investigation".to_string(),
            "FeatureDevelopment" => "The Creation".to_string(),
            "Refactoring" => "The Improvement".to_string(),
            "Testing" => "The Verification".to_string(),
            "Deployment" => "The Release".to_string(),
            _ => format!("The {:?}", intent),
        }
    }

    fn generate_title(analysis: &SessionAnalysis) -> String {
        let date = analysis.start_time.format("%Y-%m-%d").to_string();
        let primary_intent = analysis
            .intents
            .first()
            .map(|i| format!("{:?}", i.intent))
            .unwrap_or_else(|| "Development".to_string());

        format!("Development Session - {} ({})", date, primary_intent)
    }

    fn create_metadata(analysis: &SessionAnalysis) -> ScriptMetadata {
        ScriptMetadata {
            developer: analysis.developer.clone(),
            date: analysis.start_time.format("%Y-%m-%d").to_string(),
            intents: analysis
                .intents
                .iter()
                .map(|i| format!("{:?}", i.intent))
                .collect(),
            key_moments_count: analysis.key_moments.len(),
            total_changes: analysis
                .key_moments
                .iter()
                .map(|m| m.file_changes.len())
                .sum(),
        }
    }

    /// Save script to markdown file
    pub fn save_script(script: &Script, path: &str) -> Result<()> {
        let mut markdown = String::new();

        markdown.push_str(&format!("# {}\n\n", script.title));
        markdown.push_str(&format!("**Duration**: {} minutes\n\n", script.duration_minutes));
        markdown.push_str(&format!("**Developer**: {}\n\n", script.metadata.developer.as_ref().map(|s| s.as_str()).unwrap_or("Unknown")));
        markdown.push_str(&format!("**Date**: {}\n\n", script.metadata.date));
        markdown.push_str(&format!("**Intents**: {}\n\n", script.metadata.intents.join(", ")));

        for act in &script.acts {
            markdown.push_str(&format!("\n## ACT {} - {}\n\n", act.number, act.title));
            markdown.push_str(&format!("**Time**: {} - {}\n", act.time_range.0, act.time_range.1));
            markdown.push_str(&format!("**Intent**: {}\n\n", act.intent));

            for scene in &act.scenes {
                markdown.push_str(&format!("### Scene {}: {}\n\n", scene.number, scene.title));
                markdown.push_str(&format!("**Timestamp**: {}\n\n", scene.timestamp));
                markdown.push_str(&format!("{}\n\n", scene.description));

                if let Some(ref screenshot) = scene.screenshot {
                    markdown.push_str(&format!("![Screenshot]({})\n\n", screenshot));
                }

                if !scene.actions.is_empty() {
                    markdown.push_str("**Actions:**\n\n");
                    for action in &scene.actions {
                        match action {
                            Action::FileChange { path, change, .. } => {
                                markdown.push_str(&format!("- 📝 File: {} ({})\n", path, change));
                            }
                            Action::TerminalCommand { command, .. } => {
                                markdown.push_str(&format!("- 💻 Command: `{}`\n", command));
                            }
                            Action::ClipboardCopy { content } => {
                                markdown.push_str(&format!("- 📋 Clipboard: `{}`\n", content.chars().take(50).collect::<String>()));
                            }
                            Action::Screenshot { path, .. } => {
                                markdown.push_str(&format!("- 📸 Screenshot: {}\n", path));
                            }
                            Action::Error { message, .. } => {
                                markdown.push_str(&format!("- ❌ Error: {}\n", message));
                            }
                            _ => {}
                        }
                    }
                    markdown.push_str("\n");
                }

                if !scene.dialogue.is_empty() {
                    markdown.push_str("**Dialogue:**\n\n");
                    for dialogue in &scene.dialogue {
                        markdown.push_str(&format!("**{}**: {}\n", dialogue.speaker, dialogue.line));
                    }
                    markdown.push_str("\n");
                }

                if !scene.code_notes.is_empty() {
                    markdown.push_str("**Code Notes:**\n\n");
                    for note in &scene.code_notes {
                        markdown.push_str(&format!("#### {}\n\n", note.location));
                        markdown.push_str(&format!("**Decision**: {}\n\n", note.decision));
                        markdown.push_str(&format!("**Rationale**: {}\n\n", note.rationale));
                        if !note.alternatives.is_empty() {
                            markdown.push_str(&format!("**Alternatives**: {}\n\n", note.alternatives.join(", ")));
                        }
                    }
                }
            }
        }

        std::fs::write(path, markdown)?;
        Ok(())
    }

    /// Save timeline to JSON file for video generation
    pub fn save_timeline(timeline: &[TimelineEntry], path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(timeline)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{Intent, KeyMoment};
    use chrono::Utc;

    #[test]
    fn test_generate_script_empty() {
        let analysis = SessionAnalysis {
            session_id: "test".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            intents: vec![],
            key_moments: vec![],
            patterns: vec![],
            summary: "Empty".to_string(),
            developer: None,
        };

        let script = ScriptGenerator::generate(&analysis).unwrap();
        assert_eq!(script.acts.len(), 1); // Default act created
    }

    #[test]
    fn test_generate_timeline() {
        let moment = KeyMoment {
            timestamp: Utc::now(),
            title: "Test Moment".to_string(),
            description: "Testing timeline generation".to_string(),
            intent: Intent::FeatureDevelopment,
            screenshot_path: None,
            file_changes: vec![],
            clipboard_content: None,
        };

        let analysis = SessionAnalysis {
            session_id: "test".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::minutes(30),
            intents: vec![],
            key_moments: vec![moment],
            patterns: vec![],
            summary: "Test".to_string(),
            developer: None,
        };

        let timeline = ScriptGenerator::generate_timeline(&analysis).unwrap();
        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].title, "Test Moment");
    }
}
