//! Video Assets Exporter - Generates scenes.json for video generation
//!
//! This module exports session analysis data in a format optimized for
//! video generation tools, including:
//! - Timeline with timestamps
//! - Screenshot references
//! - Visual actions (highlight, typewriter, fade_out, pan, zoom)
//! - Voiceover text for narration
//! - Scene transitions and effects

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::analyzer::{KeyMoment, SessionAnalysis};

// ============================================================================
// Data Structures for Video Generation
// ============================================================================

/// Complete video assets export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoAssets {
    pub metadata: VideoMetadata,
    pub scenes: Vec<VideoScene>,
    pub settings: VideoSettings,
}

/// Metadata for the video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub title: String,
    pub session_id: String,
    pub duration_seconds: u32,
    pub created_at: String,
    pub developer: Option<String>,
    pub intents: Vec<String>,
}

/// A video scene with all necessary assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoScene {
    pub id: String,
    pub timestamp: String,
    pub duration_seconds: u32,
    pub title: String,
    pub description: String,
    pub screenshot: Option<ScreenshotAsset>,
    pub actions: Vec<VideoAction>,
    pub voiceover: VoiceoverData,
    pub transitions: SceneTransitions,
}

/// Screenshot asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotAsset {
    pub path: String,
    pub timestamp: String,
    pub width: u32,
    pub height: u32,
    pub regions_of_interest: Vec<RegionOfInterest>,
}

/// Region of interest for highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionOfInterest {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

/// Video action with visual effects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VideoAction {
    /// Highlight a specific region or element
    Highlight {
        target: HighlightTarget,
        duration_ms: u32,
        color: Option<String>, // Hex color or named color
        style: HighlightStyle,
    },
    /// Typewriter effect for text
    Typewriter {
        text: String,
        duration_ms: u32,
        font_size: Option<u32>,
        position: TextPosition,
    },
    /// Fade out to black or transparent
    FadeOut {
        duration_ms: u32,
        to_color: Option<String>, // Default: black
    },
    /// Fade in from black or transparent
    FadeIn {
        duration_ms: u32,
        from_color: Option<String>, // Default: black
    },
    /// Pan the camera in a direction
    Pan {
        direction: PanDirection,
        duration_ms: u32,
        intensity: f32, // 0.0 to 1.0
    },
    /// Zoom in or out
    Zoom {
        level: f32, // 0.5 (zoom out) to 2.0 (zoom in), 1.0 = no zoom
        duration_ms: u32,
        focus_point: Option<(u32, u32)>, // (x, y) coordinates
    },
    /// Draw an arrow or pointer
    Pointer {
        start: (u32, u32),
        end: (u32, u32),
        duration_ms: u32,
        color: Option<String>,
    },
    /// Show code diff
    CodeDiff {
        before: String,
        after: String,
        duration_ms: u32,
        highlight_changes: bool,
    },
}

/// Target for highlight action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HighlightTarget {
    Region {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Element {
        selector: String, // CSS selector or element ID
    },
    Line {
        file: String,
        line_number: u32,
    },
}

/// Highlight style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HighlightStyle {
    Glow,
    Box,
    Circle,
    Underline,
    Background,
}

/// Text position for typewriter effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextPosition {
    TopCenter,
    TopLeft,
    TopRight,
    Center,
    BottomCenter,
    BottomLeft,
    BottomRight,
}

/// Pan direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanDirection {
    Left,
    Right,
    Up,
    Down,
    Diagonal { x: i32, y: i32 }, // Vector for custom direction
}

/// Voiceover data for narration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceoverData {
    pub text: String,
    pub duration_estimate_ms: u32, // Estimated speaking time
    pub tone: VoiceoverTone,
    pub pauses: Vec<VoiceoverPause>,
}

/// Voiceover tone
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoiceoverTone {
    Neutral,
    Excited,
    Thoughtful,
    Technical,
    Casual,
}

/// Pause in voiceover
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceoverPause {
    pub position_ms: u32, // Position in the voiceover
    pub duration_ms: u32,
}

/// Scene transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTransitions {
    pub entry: Option<Transition>,
    pub exit: Option<Transition>,
}

/// Transition effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub type_: TransitionType,
    pub duration_ms: u32,
}

/// Transition type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    FadeIn,
    FadeOut,
    Slide { direction: PanDirection },
    Wipe { direction: PanDirection },
    Dissolve,
    ZoomIn,
    ZoomOut,
}

/// Video generation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub resolution: (u32, u32), // (width, height)
    pub frame_rate: u32,        // fps
    pub quality: VideoQuality,
    pub format: VideoFormat,
    pub default_font: String,
    pub default_transition: TransitionType,
}

/// Video quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Video output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoFormat {
    MP4,
    WebM,
    MOV,
    GIF,
}

// ============================================================================
// Video Assets Exporter
// ============================================================================

/// Video Assets Exporter
pub struct VideoAssetsExporter {
    default_settings: VideoSettings,
}

impl VideoAssetsExporter {
    /// Create a new exporter with default settings
    pub fn new() -> Self {
        Self {
            default_settings: VideoSettings {
                resolution: (1920, 1080),
                frame_rate: 30,
                quality: VideoQuality::High,
                format: VideoFormat::MP4,
                default_font: "Arial, 16px".to_string(),
                default_transition: TransitionType::Fade,
            },
        }
    }

    /// Create a new exporter with custom settings
    pub fn with_settings(settings: VideoSettings) -> Self {
        Self {
            default_settings: settings,
        }
    }

    /// Export session analysis as video assets
    pub fn export(&self, analysis: &SessionAnalysis) -> Result<VideoAssets> {
        let metadata = self.create_metadata(analysis)?;
        let scenes = self.create_scenes(analysis)?;

        let _duration_seconds = scenes
            .iter()
            .map(|s| s.duration_seconds)
            .sum::<u32>();

        Ok(VideoAssets {
            metadata,
            scenes,
            settings: self.default_settings.clone(),
        })
    }

    /// Export and save to scenes.json file
    pub fn export_to_file(&self, analysis: &SessionAnalysis, path: &str) -> Result<()> {
        let assets = self.export(analysis)?;
        let json = serde_json::to_string_pretty(&assets)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn create_metadata(&self, analysis: &SessionAnalysis) -> Result<VideoMetadata> {
        let duration_seconds = (analysis.end_time - analysis.start_time).num_seconds() as u32;

        Ok(VideoMetadata {
            title: self.generate_title(analysis),
            session_id: analysis.session_id.clone(),
            duration_seconds,
            created_at: Utc::now().to_rfc3339(),
            developer: analysis.developer.clone(),
            intents: analysis
                .intents
                .iter()
                .map(|i| format!("{:?}", i.intent))
                .collect(),
        })
    }

    fn create_scenes(&self, analysis: &SessionAnalysis) -> Result<Vec<VideoScene>> {
        let mut scenes = Vec::new();

        for (i, moment) in analysis.key_moments.iter().enumerate() {
            let duration = self.calculate_scene_duration(moment, i, analysis);
            let screenshot = self.create_screenshot_asset(moment)?;
            let actions = self.create_actions(moment);
            let voiceover = self.create_voiceover(moment, duration);
            let transitions = self.create_transitions(i, analysis.key_moments.len());

            scenes.push(VideoScene {
                id: format!("scene-{:03}", i + 1),
                timestamp: moment.timestamp.to_rfc3339(),
                duration_seconds: duration,
                title: moment.title.clone(),
                description: moment.description.clone(),
                screenshot,
                actions,
                voiceover,
                transitions,
            });
        }

        // If no key moments, create a default scene
        if scenes.is_empty() {
            scenes.push(self.create_default_scene(analysis)?);
        }

        Ok(scenes)
    }

    fn calculate_scene_duration(
        &self,
        moment: &KeyMoment,
        index: usize,
        analysis: &SessionAnalysis,
    ) -> u32 {
        // Calculate duration based on next moment or default
        if index < analysis.key_moments.len() - 1 {
            let next_moment = &analysis.key_moments[index + 1];
            let diff = (next_moment.timestamp - moment.timestamp).num_seconds();
            if diff > 0 && diff < 300 {
                // Max 5 minutes per scene
                return diff as u32;
            }
        }

        // Default duration based on content
        let base_duration = 30; // 30 seconds default

        // Add time for file changes
        let file_change_time = moment.file_changes.len() as u32 * 5; // 5 seconds per file

        // Add time for screenshot viewing
        let screenshot_time = if moment.screenshot_path.is_some() { 10 } else { 0 };

        base_duration + file_change_time + screenshot_time
    }

    fn create_screenshot_asset(&self, moment: &KeyMoment) -> Result<Option<ScreenshotAsset>> {
        if let Some(ref path) = moment.screenshot_path {
            Ok(Some(ScreenshotAsset {
                path: path.clone(),
                timestamp: moment.timestamp.to_rfc3339(),
                width: 1920,  // Default, should be read from actual file
                height: 1080,
                regions_of_interest: self.detect_regions_of_interest(moment),
            }))
        } else {
            Ok(None)
        }
    }

    fn detect_regions_of_interest(&self, moment: &KeyMoment) -> Vec<RegionOfInterest> {
        let mut regions = Vec::new();

        // Add regions for file changes
        for (i, change) in moment.file_changes.iter().enumerate() {
            regions.push(RegionOfInterest {
                x: 100 + (i as u32 * 50), // Example positioning
                y: 200,
                width: 800,
                height: 100,
                label: change.path.clone(),
            });
        }

        regions
    }

    fn create_actions(&self, moment: &KeyMoment) -> Vec<VideoAction> {
        let mut actions = Vec::new();

        // Add highlight for first file change
        if let Some(first_file) = moment.file_changes.first() {
            actions.push(VideoAction::Highlight {
                target: HighlightTarget::Line {
                    file: first_file.path.clone(),
                    line_number: 1, // Would be calculated from actual diff
                },
                duration_ms: 3000,
                color: Some("#FFD700".to_string()), // Gold
                style: HighlightStyle::Glow,
            });
        }

        // Add typewriter for title
        actions.push(VideoAction::Typewriter {
            text: moment.title.clone(),
            duration_ms: 2000,
            font_size: Some(32),
            position: TextPosition::TopCenter,
        });

        // Add code diff if there are file changes
        if !moment.file_changes.is_empty() {
            actions.push(VideoAction::CodeDiff {
                before: "[Previous code]".to_string(),
                after: "[New code]".to_string(),
                duration_ms: 5000,
                highlight_changes: true,
            });
        }

        // Add fade out at end
        actions.push(VideoAction::FadeOut {
            duration_ms: 1000,
            to_color: Some("#000000".to_string()),
        });

        actions
    }

    fn create_voiceover(&self, moment: &KeyMoment, duration_seconds: u32) -> VoiceoverData {
        let text = self.generate_voiceover_text(moment);
        let duration_estimate_ms = duration_seconds * 1000;
        let tone = self.determine_tone(moment);
        let pauses = self.generate_pauses(&text);

        VoiceoverData {
            text,
            duration_estimate_ms,
            tone,
            pauses,
        }
    }

    fn generate_voiceover_text(&self, moment: &KeyMoment) -> String {
        match moment.intent {
            crate::analyzer::Intent::BugFixing => {
                format!(
                    "Let's debug this issue. {} The developer is investigating {}.",
                    moment.title,
                    moment.file_changes.iter().map(|f| f.path.as_str()).collect::<Vec<_>>().join(" and ")
                )
            }
            crate::analyzer::Intent::FeatureDevelopment => {
                format!(
                    "Now implementing a new feature. {} {}",
                    moment.title, moment.description
                )
            }
            crate::analyzer::Intent::Refactoring => {
                format!(
                    "Improving the code structure. {} This refactoring focuses on {}.",
                    moment.title,
                    moment.file_changes.iter().map(|f| f.path.as_str()).collect::<Vec<_>>().join(" and ")
                )
            }
            crate::analyzer::Intent::Testing => {
                format!(
                    "Verifying the implementation. {} Running tests to ensure everything works correctly.",
                    moment.title
                )
            }
            _ => {
                format!(
                    "{}. {}",
                    moment.title, moment.description
                )
            }
        }
    }

    fn determine_tone(&self, moment: &KeyMoment) -> VoiceoverTone {
        match moment.intent {
            crate::analyzer::Intent::BugFixing => VoiceoverTone::Thoughtful,
            crate::analyzer::Intent::FeatureDevelopment => VoiceoverTone::Excited,
            crate::analyzer::Intent::Refactoring => VoiceoverTone::Technical,
            crate::analyzer::Intent::Testing => VoiceoverTone::Neutral,
            _ => VoiceoverTone::Casual,
        }
    }

    fn generate_pauses(&self, text: &str) -> Vec<VoiceoverPause> {
        // Generate pauses at sentence boundaries
        let mut pauses = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if c == '.' || c == '!' || c == '?' {
                pauses.push(VoiceoverPause {
                    position_ms: ((i as f32) / (chars.len() as f32) * 10000.0) as u32, // Normalize to 10 seconds
                    duration_ms: 500, // 500ms pause
                });
            }
        }

        pauses
    }

    fn create_transitions(&self, index: usize, total_scenes: usize) -> SceneTransitions {
        let entry = if index > 0 {
            Some(Transition {
                type_: TransitionType::Fade,
                duration_ms: 500,
            })
        } else {
            Some(Transition {
                type_: TransitionType::FadeIn,
                duration_ms: 1000,
            })
        };

        let exit = if index < total_scenes - 1 {
            Some(Transition {
                type_: TransitionType::Fade,
                duration_ms: 500,
            })
        } else {
            Some(Transition {
                type_: TransitionType::FadeOut,
                duration_ms: 1500,
            })
        };

        SceneTransitions { entry, exit }
    }

    fn create_default_scene(&self, analysis: &SessionAnalysis) -> Result<VideoScene> {
        let duration = (analysis.end_time - analysis.start_time).num_seconds() as u32;
        let text = format!(
            "Development session from {} to {}",
            analysis.start_time.format("%H:%M"),
            analysis.end_time.format("%H:%M")
        );

        Ok(VideoScene {
            id: "scene-001".to_string(),
            timestamp: analysis.start_time.to_rfc3339(),
            duration_seconds: duration.max(30),
            title: "Development Session".to_string(),
            description: analysis.summary.clone(),
            screenshot: None,
            actions: vec![],
            voiceover: VoiceoverData {
                text,
                duration_estimate_ms: duration * 1000,
                tone: VoiceoverTone::Neutral,
                pauses: vec![],
            },
            transitions: SceneTransitions {
                entry: Some(Transition {
                    type_: TransitionType::FadeIn,
                    duration_ms: 1000,
                }),
                exit: Some(Transition {
                    type_: TransitionType::FadeOut,
                    duration_ms: 1000,
                }),
            },
        })
    }

    fn generate_title(&self, analysis: &SessionAnalysis) -> String {
        let date = analysis.start_time.format("%Y-%m-%d").to_string();
        let primary_intent = analysis
            .intents
            .first()
            .map(|i| format!("{:?}", i.intent))
            .unwrap_or_else(|| "Development".to_string());

        format!("Development Session - {} ({})", date, primary_intent)
    }
}

impl Default for VideoAssetsExporter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{Intent, KeyMoment};
    use chrono::Utc;

    #[test]
    fn test_export_empty_session() {
        let exporter = VideoAssetsExporter::new();

        let analysis = SessionAnalysis {
            session_id: "test".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::minutes(30),
            intents: vec![],
            key_moments: vec![],
            patterns: vec![],
            summary: "Empty session".to_string(),
            developer: None,
        };

        let assets = exporter.export(&analysis).unwrap();
        assert_eq!(assets.scenes.len(), 1); // Default scene created
        assert_eq!(assets.scenes[0].title, "Development Session");
    }

    #[test]
    fn test_export_with_key_moments() {
        let exporter = VideoAssetsExporter::new();

        let moment = KeyMoment {
            timestamp: Utc::now(),
            title: "Test Moment".to_string(),
            description: "Testing video export".to_string(),
            intent: Intent::FeatureDevelopment,
            screenshot_path: Some("/path/to/screenshot.png".to_string()),
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
            developer: Some("Developer".to_string()),
        };

        let assets = exporter.export(&analysis).unwrap();
        assert_eq!(assets.scenes.len(), 1);
        assert_eq!(assets.scenes[0].title, "Test Moment");
        assert!(assets.scenes[0].screenshot.is_some());
        assert!(!assets.scenes[0].actions.is_empty());
        assert!(!assets.scenes[0].voiceover.text.is_empty());
    }

    #[test]
    fn test_voiceover_generation() {
        let exporter = VideoAssetsExporter::new();

        let moment = KeyMoment {
            timestamp: Utc::now(),
            title: "Fix Bug".to_string(),
            description: "Fixing critical bug".to_string(),
            intent: Intent::BugFixing,
            screenshot_path: None,
            file_changes: vec![],
            clipboard_content: None,
        };

        let voiceover = exporter.create_voiceover(&moment, 30);
        assert!(!voiceover.text.is_empty());
        assert_eq!(voiceover.tone, VoiceoverTone::Thoughtful);
        assert_eq!(voiceover.duration_estimate_ms, 30000);
    }

    #[test]
    fn test_scene_transitions() {
        let exporter = VideoAssetsExporter::new();

        let transitions_first = exporter.create_transitions(0, 3);
        assert!(transitions_first.entry.is_some());
        assert!(transitions_first.exit.is_some());

        let transitions_middle = exporter.create_transitions(1, 3);
        assert!(transitions_middle.entry.is_some());
        assert!(transitions_middle.exit.is_some());

        let transitions_last = exporter.create_transitions(2, 3);
        assert!(transitions_last.entry.is_some());
        assert!(transitions_last.exit.is_some());
    }
}
