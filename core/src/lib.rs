// EX-G-SE: Shadow Logging Core Engine - Library Interface
//
// This library provides the core functionality for the EX-G-SE shadow logging system.

pub mod ai;
pub mod analyzer;
pub mod cli;
pub mod core;
pub mod fs_watcher;
pub mod script;
pub mod script_generator;
pub mod screenshot;
pub mod video_exporter;
pub mod watchers;

pub use ai::{
    Act, AIConfig, AIError, AIProvider, AIProviderFactory, AIProviderType, AIResult,
    AnalysisInput, Dialogue, Intent, KeyMoment, Scene, Script, ScriptInput, SessionAnalysis,
    create_anthropic_provider, create_openai_provider,
};
pub use analyzer::{Intent as AnalyzerIntent, KeyMoment as AnalyzerKeyMoment, SessionAnalysis as AnalyzerSessionAnalysis, SessionAnalyzer};
pub use cli::CliConfig;
pub use core::ExGSeEngine;
pub use fs_watcher::watch_directory;
pub use screenshot::{capture_active_window, capture_ide_window, capture_screenshot, ScreenshotInfo};
pub use script::{Script as ScriptScript, ScriptGenerator, TimelineEntry};
pub use video_exporter::{VideoAssets, VideoAssetsExporter, VideoScene, VideoSettings};
pub use watchers::{LogEntry, SessionLogs};
pub use script_generator::{AIScriptGenerator, ScriptGenerationInput, ScriptEvent, extract_project_context, extract_code_files, CodeFile, ScriptScreenshotInfo};

// Include test modules
#[cfg(test)]
mod tests {
    // Tests are in separate files to keep the main code clean
    // The test modules will be compiled when running `cargo test`
}
