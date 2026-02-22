// AI Module - Shared Types
//
// This module defines the shared data structures used across the AI provider implementations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// Intent Types
// ============================================================================

/// Represents the primary intent detected during a development session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    /// Developing a new feature or functionality
    FeatureDevelopment,
    /// Fixing bugs or resolving errors
    BugFixing,
    /// Restructuring code without changing behavior
    Refactoring,
    /// Updating documentation
    Documentation,
    /// Writing or running tests
    Testing,
    /// Code review and analysis
    CodeReview,
    /// Learning or researching
    Learning,
    /// Deployment and operations
    Deployment,
    /// Other intent not covered by above categories
    Other(String),
}

impl std::fmt::Display for Intent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Intent::FeatureDevelopment => write!(f, "feature_development"),
            Intent::BugFixing => write!(f, "bug_fixing"),
            Intent::Refactoring => write!(f, "refactoring"),
            Intent::Documentation => write!(f, "documentation"),
            Intent::Testing => write!(f, "testing"),
            Intent::CodeReview => write!(f, "code_review"),
            Intent::Learning => write!(f, "learning"),
            Intent::Deployment => write!(f, "deployment"),
            Intent::Other(s) => write!(f, "other: {}", s),
        }
    }
}

// ============================================================================
// Key Moment Types
// ============================================================================

/// Represents a significant moment during a development session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMoment {
    /// Timestamp of the key moment
    pub timestamp: DateTime<Utc>,
    /// Title describing the moment
    pub title: String,
    /// Detailed description of what happened
    pub description: String,
    /// Optional path to a screenshot taken at this moment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_path: Option<PathBuf>,
    /// Associated log entry indices
    #[serde(default)]
    pub related_entries: Vec<usize>,
}

impl KeyMoment {
    /// Create a new key moment
    pub fn new(
        timestamp: DateTime<Utc>,
        title: String,
        description: String,
    ) -> Self {
        Self {
            timestamp,
            title,
            description,
            screenshot_path: None,
            related_entries: Vec::new(),
        }
    }

    /// Add a screenshot path to this key moment
    pub fn with_screenshot(mut self, path: PathBuf) -> Self {
        self.screenshot_path = Some(path);
        self
    }

    /// Add related log entry indices
    pub fn with_related_entries(mut self, entries: Vec<usize>) -> Self {
        self.related_entries = entries;
        self
    }
}

// ============================================================================
// Session Analysis Types
// ============================================================================

/// Complete analysis of a development session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalysis {
    /// Session start time
    pub session_start: DateTime<Utc>,
    /// Session end time
    pub session_end: DateTime<Utc>,
    /// Detected primary intents (ordered by confidence)
    pub intents: Vec<Intent>,
    /// Significant moments identified during the session
    pub key_moments: Vec<KeyMoment>,
    /// Patterns detected in the development workflow
    pub patterns: Vec<String>,
    /// Overall summary of the session
    pub summary: String,
    /// Suggested title for the session
    pub suggested_title: String,
    /// Technologies and tools detected
    #[serde(default)]
    pub technologies: Vec<String>,
    /// Files that were modified
    #[serde(default)]
    pub files_modified: Vec<String>,
    /// Confidence score of the analysis (0.0 to 1.0)
    pub confidence: f32,
}

impl SessionAnalysis {
    /// Create a new session analysis
    pub fn new(
        session_start: DateTime<Utc>,
        session_end: DateTime<Utc>,
        summary: String,
        suggested_title: String,
    ) -> Self {
        Self {
            session_start,
            session_end,
            intents: Vec::new(),
            key_moments: Vec::new(),
            patterns: Vec::new(),
            summary,
            suggested_title,
            technologies: Vec::new(),
            files_modified: Vec::new(),
            confidence: 0.0,
        }
    }

    /// Add an intent to the analysis
    pub fn with_intent(mut self, intent: Intent) -> Self {
        self.intents.push(intent);
        self
    }

    /// Add a key moment
    pub fn with_key_moment(mut self, moment: KeyMoment) -> Self {
        self.key_moments.push(moment);
        self
    }

    /// Set the confidence score
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }
}

// ============================================================================
// Script Generation Types
// ============================================================================

/// A dialogue line in a script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    /// The character/speaker
    pub speaker: String,
    /// The spoken text
    pub text: String,
    /// Optional action or direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl Dialogue {
    /// Create a new dialogue line
    pub fn new(speaker: String, text: String) -> Self {
        Self {
            speaker,
            text,
            action: None,
        }
    }

    /// Add an action to the dialogue
    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }
}

/// A scene within an act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Scene number or identifier
    pub number: u32,
    /// Scene title
    pub title: String,
    /// Scene description
    pub description: String,
    /// Timestamp reference from the session
    pub timestamp: DateTime<Utc>,
    /// Dialogue and action in this scene
    pub dialogue: Vec<Dialogue>,
    /// Related key moments
    #[serde(default)]
    pub related_moments: Vec<usize>,
}

impl Scene {
    /// Create a new scene
    pub fn new(
        number: u32,
        title: String,
        description: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            number,
            title,
            description,
            timestamp,
            dialogue: Vec::new(),
            related_moments: Vec::new(),
        }
    }

    /// Add dialogue to the scene
    pub fn with_dialogue(mut self, dialogue: Dialogue) -> Self {
        self.dialogue.push(dialogue);
        self
    }
}

/// An act containing multiple scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    /// Act number (I, II, III, etc.)
    pub number: u32,
    /// Act title
    pub title: String,
    /// Act description
    pub description: String,
    /// Scenes in this act
    pub scenes: Vec<Scene>,
    /// Overall theme of the act
    pub theme: String,
}

impl Act {
    /// Create a new act
    pub fn new(number: u32, title: String, description: String, theme: String) -> Self {
        Self {
            number,
            title,
            description,
            scenes: Vec::new(),
            theme,
        }
    }

    /// Add a scene to the act
    pub fn with_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(scene);
        self
    }
}

/// Complete generated script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// Script title
    pub title: String,
    /// Script tagline or subtitle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    /// Logline (one-sentence summary)
    pub logline: String,
    /// Acts in the script
    pub acts: Vec<Act>,
    /// Characters in the script
    #[serde(default)]
    pub characters: Vec<String>,
    /// Genre or style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Estimated runtime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_runtime_minutes: Option<u32>,
}

impl Script {
    /// Create a new script
    pub fn new(title: String, logline: String) -> Self {
        Self {
            title,
            tagline: None,
            logline,
            acts: Vec::new(),
            characters: Vec::new(),
            genre: None,
            estimated_runtime_minutes: None,
        }
    }

    /// Add an act to the script
    pub fn with_act(mut self, act: Act) -> Self {
        self.acts.push(act);
        self
    }

    /// Set the genre
    pub fn with_genre(mut self, genre: String) -> Self {
        self.genre = Some(genre);
        self
    }

    /// Add a character
    pub fn with_character(mut self, character: String) -> Self {
        self.characters.push(character);
        self
    }
}

// ============================================================================
// AI Request/Response Types
// ============================================================================

/// Input data for session analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisInput {
    /// Session logs
    pub logs: serde_json::Value,
    /// Session start time
    pub start_time: DateTime<Utc>,
    /// Session end time
    pub end_time: DateTime<Utc>,
    /// Optional context about the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_context: Option<String>,
    /// Optional list of screenshot paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<PathBuf>>,
}

/// Input data for script generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptInput {
    /// Session analysis results
    pub analysis: SessionAnalysis,
    /// Desired style or tone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Target audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_display() {
        assert_eq!(Intent::FeatureDevelopment.to_string(), "feature_development");
        assert_eq!(Intent::BugFixing.to_string(), "bug_fixing");
        assert_eq!(Intent::Other("custom".to_string()).to_string(), "other: custom");
    }

    #[test]
    fn test_key_moment_builder() {
        let ts = Utc::now();
        let moment = KeyMoment::new(
            ts,
            "Test Moment".to_string(),
            "Test Description".to_string(),
        )
        .with_screenshot(PathBuf::from("/test/path.png"))
        .with_related_entries(vec![0, 1, 2]);

        assert_eq!(moment.title, "Test Moment");
        assert_eq!(moment.screenshot_path, Some(PathBuf::from("/test/path.png")));
        assert_eq!(moment.related_entries, vec![0, 1, 2]);
    }

    #[test]
    fn test_session_analysis_serialization() {
        let analysis = SessionAnalysis::new(
            Utc::now(),
            Utc::now(),
            "Test summary".to_string(),
            "Test Title".to_string(),
        )
        .with_intent(Intent::FeatureDevelopment)
        .with_confidence(0.85);

        let json = serde_json::to_string(&analysis).unwrap();
        assert!(json.contains("feature_development"));
        assert!(json.contains("0.85"));
    }

    #[test]
    fn test_script_builder() {
        let script = Script::new(
            "Test Script".to_string(),
            "A test logline".to_string(),
        )
        .with_genre("Comedy".to_string())
        .with_character("Developer".to_string());

        assert_eq!(script.title, "Test Script");
        assert_eq!(script.genre, Some("Comedy".to_string()));
        assert!(script.characters.contains(&"Developer".to_string()));
    }

    #[test]
    fn test_dialogue_builder() {
        let dialogue = Dialogue::new("Dev".to_string(), "Hello, world!".to_string())
            .with_action("types furiously".to_string());

        assert_eq!(dialogue.speaker, "Dev");
        assert_eq!(dialogue.action, Some("types furiously".to_string()));
    }

    #[test]
    fn test_act_and_scene() {
        let ts = Utc::now();
        let scene = Scene::new(1, "Opening".to_string(), "Start".to_string(), ts)
            .with_dialogue(Dialogue::new("Dev".to_string(), "Code".to_string()));

        let act = Act::new(1, "Act I".to_string(), "Beginning".to_string(), "Setup".to_string())
            .with_scene(scene);

        assert_eq!(act.scenes.len(), 1);
        assert_eq!(act.scenes[0].dialogue.len(), 1);
    }
}
