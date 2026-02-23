//! AI-Powered Script Generator
//!
//! This module analyzes development sessions and generates
//! theatrical scripts using AI to capture:
//! - Developer thoughts and intentions
//! - Why certain choices were made
//! - Technical context and decisions
//! - Narrative flow of the session

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Script generation input with enhanced context
#[derive(Debug, Clone, Serialize)]
pub struct ScriptGenerationInput {
    pub session_start: DateTime<Utc>,
    pub session_end: DateTime<Utc>,
    pub events: Vec<ScriptEvent>,
    pub project_context: ProjectContext,
    pub code_context: Vec<CodeFile>,
    pub screenshots: Vec<ScriptScreenshotInfo>,
}

/// Event for script generation
#[derive(Debug, Clone, Serialize)]
pub struct ScriptEvent {
    pub timestamp: String,
    pub event_type: String,
    pub description: String,
    pub details: serde_json::Value,
}

/// Code file with content
#[derive(Debug, Clone, Serialize)]
pub struct CodeFile {
    pub path: String,
    pub content: String,
    pub language: String,
}

/// Screenshot info for script
#[derive(Debug, Clone, Serialize)]
pub struct ScriptScreenshotInfo {
    pub path: String,
    pub timestamp: String,
}

/// Project context
#[derive(Debug, Clone, Serialize)]
pub struct ProjectContext {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tech_stack: Vec<String>,
    pub file_structure: Vec<String>,
    pub recent_changes: Vec<FileChange>,
    pub claude_md_context: Option<String>,
}

/// File change info
#[derive(Debug, Clone, Serialize)]
pub struct FileChange {
    pub path: String,
    pub action: String,
}

/// AI Provider config
#[derive(Debug, Clone)]
pub struct AIProviderConfig {
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

/// AI Script Generator
pub struct AIScriptGenerator {
    client: Client,
    config: AIProviderConfig,
}

impl AIScriptGenerator {
    /// Create new generator from encrypted config
    pub fn new() -> Result<Self> {
        let config = Self::load_config()?;

        Ok(Self {
            client: Client::new(),
            config,
        })
    }

    /// Load and decrypt AI config
    fn load_config() -> Result<AIProviderConfig> {
        // Try environment variables first (set by Node.js wrapper)
        if let Ok(provider) = std::env::var("EX_G_SE_PROVIDER") {
            let api_key = std::env::var("EX_G_SE_API_KEY").unwrap_or_default();
            let api_url = std::env::var("EX_G_SE_API_URL").unwrap_or_default();
            let model = std::env::var("EX_G_SE_MODEL").unwrap_or_default();

            if !api_key.is_empty() {
                return Ok(AIProviderConfig {
                    provider,
                    api_key,
                    api_url,
                    model,
                });
            }
        }

        // Fallback to encrypted file (for backward compatibility)
        let config_dir = dirs::home_dir()
            .map(|h| h.join(".config").join("ex-g-se"))
            .unwrap_or_else(|| PathBuf::from(".config/ex-g-se"));

        let config_path = config_dir.join("settings.enc");

        if !config_path.exists() {
            return Ok(AIProviderConfig {
                provider: "none".to_string(),
                api_key: String::new(),
                api_url: String::new(),
                model: String::new(),
            });
        }

        // Note: The encrypted file cannot be decrypted by Rust directly
        // It needs to be passed via environment variables from Node.js wrapper
        Ok(AIProviderConfig {
            provider: "none".to_string(),
            api_key: String::new(),
            api_url: String::new(),
            model: String::new(),
        })
    }

    /// Check if AI is configured
    pub fn is_configured(&self) -> bool {
        !self.config.api_key.is_empty()
    }

    /// Generate script from session data with full AI analysis
    pub async fn generate(&self, input: &ScriptGenerationInput) -> Result<String> {
        if !self.is_configured() {
            return Ok(Self::generate_fallback_script(input));
        }

        let prompt = self.build_detailed_prompt(input);

        let script_content = match self.config.provider.as_str() {
            "openai" => self.call_openai(&prompt).await?,
            "anthropic" => self.call_anthropic(&prompt).await?,
            "zai" => self.call_zai(&prompt).await?,
            _ => self.call_openai(&prompt).await?,
        };

        Ok(script_content)
    }

    /// Build detailed prompt for AI with code and screenshot analysis
    fn build_detailed_prompt(&self, input: &ScriptGenerationInput) -> String {
        let duration = (input.session_end - input.session_start).num_seconds();

        let mut prompt = String::new();

        prompt.push_str("# Development Session Analysis Request\n\n");
        prompt.push_str(&format!("Duration: {} seconds\n", duration));
        prompt.push_str(&format!("Events: {}\n\n", input.events.len()));

        // Project context with CLAUDE.md
        prompt.push_str("## Project Context\n\n");
        if let Some(name) = &input.project_context.name {
            prompt.push_str(&format!("Project: {}\n", name));
        }
        if let Some(desc) = &input.project_context.description {
            prompt.push_str(&format!("Description: {}\n", desc));
        }
        prompt.push_str(&format!("Tech Stack: {}\n", input.project_context.tech_stack.join(", ")));

        if let Some(claude_ctx) = &input.project_context.claude_md_context {
            prompt.push_str("\n### Project Configuration (CLAUDE.md)\n\n");
            prompt.push_str(claude_ctx);
            prompt.push_str("\n");
        }

        // Code changes with content
        if !input.code_context.is_empty() {
            prompt.push_str("\n## Code Changes Analysis\n\n");
            prompt.push_str("### Modified Files (with content):\n\n");
            for code_file in &input.code_context {
                prompt.push_str(&format!("#### {}\n\n", code_file.path));
                prompt.push_str(&format!("**Language:** {}\n\n", code_file.language));

                // Include first 500 chars of code (using char boundaries)
                let preview = if code_file.content.chars().count() > 500 {
                    let truncated: String = code_file.content.chars().take(500).collect();
                    format!("{}...\n\n[{} chars total]\n", truncated, code_file.content.chars().count())
                } else {
                    format!("{}\n\n", code_file.content)
                };
                prompt.push_str(&format!("```{}\n{}\n```\n\n",
                    code_file.language, preview));
            }
        }

        // Screenshots
        if !input.screenshots.is_empty() {
            prompt.push_str("## Screenshots\n\n");
            for (i, shot) in input.screenshots.iter().enumerate() {
                prompt.push_str(&format!("### Screenshot {} - {}\n\n", i + 1, shot.timestamp));
                prompt.push_str(&format!("File: {}\n\n", shot.path));
                // Note: In v0.4.6 we can add OCR here
                prompt.push_str("(Visual screenshot captured)\n\n");
            }
        }

        // Timeline
        prompt.push_str("## Session Timeline\n\n");
        for (i, event) in input.events.iter().enumerate() {
            prompt.push_str(&format!("### [{}] {}\n\n", event.timestamp, event.event_type));
            prompt.push_str(&format!("{}\n\n", event.description));

            if event.event_type == "clipboard" {
                if let Some(content) = event.details.get("content") {
                    if let Some(text) = content.as_str() {
                        if text.len() < 800 {
                            prompt.push_str(&format!("**Clipboard:** `{}`\n\n", text));
                        } else {
                            prompt.push_str(&format!("**Clipboard:** [{} chars, truncated]\n\n", text.len()));
                        }
                    }
                }
            }
        }

        // Request
        prompt.push_str("\n## Task\n\n");
        prompt.push_str("Generate a **theatrical script** in Markdown that tells the story of this development session.\n\n");
        prompt.push_str("### Requirements:\n\n");
        prompt.push_str("1. **Structure:** Use Acts and Scenes (theatrical format)\n");
        prompt.push_str("2. **Narrative:** Tell the STORY of what happened\n");
        prompt.push_str("3. **Thoughts:** Capture the developer's INTENTIONS and REASONING\n");
        prompt.push_str("4. **Technical:** Explain WHY choices were made\n");
        prompt.push_str("5. **Visual:** Describe what's visible in screenshots\n");
        prompt.push_str("6. **Context:** Use the project info and code to understand the domain\n\n");
        prompt.push_str("### Format:\n\n");
        prompt.push_str("# [Engaging Title]\n\n");
        prompt.push_str("## Act 1 - [The Purpose/Goal]\n\n");
        prompt.push_str("*Setting the scene: What is the developer trying to accomplish?*\n\n");
        prompt.push_str("### Scene 1 - [HH:MM:SS] - [Brief Moment Title]\n\n");
        prompt.push_str("**[Stage Direction]**\n\n");
        prompt.push_str("The developer [action]. [Technical detail].\n\n");
        prompt.push_str("**Developer's Thoughts:**\n\n");
        prompt.push_str("> [Internal monologue - what they're thinking and why]\n\n");
        prompt.push_str("**Technical Context:**\n\n");
        prompt.push_str("> [Relevant technical details from the code/project]\n\n");
        prompt.push_str("---\n\n");

        prompt.push_str("**IMPORTANT:**\n");
        prompt.push_str("- Be SPECIFIC about technical decisions\n");
        prompt.push_str("- Show DEVELOPER'S THOUGHT PROCESS\n");
        prompt.push_str("- Reference actual code and project context\n");
        prompt.push_str("- Make it ENGAGING like a story\n");
        prompt.push_str("- Use timestamps for scene headers\n");

        prompt
    }

    /// Build prompt for AI (deprecated - use build_detailed_prompt)
    fn build_prompt(input: &ScriptGenerationInput) -> String {
        let duration = (input.session_end - input.session_start).num_seconds();

        let mut prompt = String::new();

        prompt.push_str("# Development Session Analysis\n\n");
        prompt.push_str(&format!("Duration: {} seconds\n", duration));
        prompt.push_str(&format!("Events: {}\n\n", input.events.len()));

        // Project context
        prompt.push_str("## Project Context\n\n");
        if let Some(name) = &input.project_context.name {
            prompt.push_str(&format!("Project: {}\n", name));
        }
        if let Some(desc) = &input.project_context.description {
            prompt.push_str(&format!("Description: {}\n", desc));
        }
        prompt.push_str(&format!("Tech Stack: {}\n", input.project_context.tech_stack.join(", ")));
        prompt.push_str("\n");

        // Timeline
        prompt.push_str("## Session Timeline\n\n");
        for (i, event) in input.events.iter().enumerate() {
            prompt.push_str(&format!("{}. [{}] {}\n", i + 1, event.timestamp, event.description));
            if event.event_type == "clipboard" {
                if let Some(content) = event.details.get("content") {
                    if let Some(text) = content.as_str() {
                        if text.len() < 500 {
                            prompt.push_str(&format!("   Content: `{}`\n", text));
                        }
                    }
                }
            }
        }
        prompt.push_str("\n");

        // Request
        prompt.push_str("## Task\n\n");
        prompt.push_str("Generate a theatrical script in Markdown format that captures:\n");
        prompt.push_str("- What the developer was thinking and trying to accomplish\n");
        prompt.push_str("- Why they made certain technical choices\n");
        prompt.push_str("- The flow of their work and decision-making process\n");
        prompt.push_str("- Key moments and insights from the session\n\n");
        prompt.push_str("Format:\n");
        prompt.push_str("# Title\n\n");
        prompt.push_str("## Act 1 - [Purpose]\n\n");
        prompt.push_str("### Scene 1 - [HH:MM:SS]\n\n");
        prompt.push_str("[Narrative description of what happened and why]\n\n");
        prompt.push_str("**Developer's Thoughts:** [What they were thinking]\n\n");
        prompt.push_str("**Technical Context:** [Relevant technical details]\n\n");

        prompt
    }

    /// Call OpenAI API
    async fn call_openai(&self, prompt: &str) -> Result<String> {
        let url = if self.config.api_url.is_empty() {
            "https://api.openai.com/v1/chat/completions".to_string()
        } else {
            format!("{}/chat/completions", self.config.api_url)
        };

        let model = if self.config.model.is_empty() {
            "gpt-4o"
        } else {
            &self.config.model
        };

        let request = json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert technical writer and analyst. Generate engaging, insightful scripts from development sessions. Capture the developer's thought process, technical decisions, and the 'why' behind their actions. Write in a theatrical narrative style with acts and scenes."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 4000
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        Ok(result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Error: No response from AI")
            .to_string())
    }

    /// Call Anthropic API
    async fn call_anthropic(&self, prompt: &str) -> Result<String> {
        let url = if self.config.api_url.is_empty() {
            "https://api.anthropic.com/v1/messages".to_string()
        } else {
            format!("{}/messages", self.config.api_url)
        };

        let model = if self.config.model.is_empty() {
            "claude-3-5-sonnet-20241022"
        } else {
            &self.config.model
        };

        let request = json!({
            "model": model,
            "max_tokens": 4000,
            "system": "You are an expert technical writer and analyst. Generate engaging, insightful scripts from development sessions. Capture the developer's thought process, technical decisions, and the 'why' behind their actions. Write in a theatrical narrative style with acts and scenes.",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = self.client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        Ok(result["content"][0]["text"]
            .as_str()
            .unwrap_or("Error: No response from AI")
            .to_string())
    }

    /// Call ZAI API
    async fn call_zai(&self, prompt: &str) -> Result<String> {
        // ZAI uses OpenAI-compatible format
        // Config URL should already include /v1, just append /chat/completions
        let base_url = if self.config.api_url.is_empty() {
            "https://api.z.ai/v1".to_string()
        } else {
            // Auto-fix common mistake: .v1 -> /v1
            let url = self.config.api_url.trim_end_matches('/');
            url.replace(".v1", "/v1")
        };

        let full_url = format!("{}/chat/completions", base_url);
        self.call_openai_with_url(prompt, &full_url).await
    }

    /// Call OpenAI-compatible API with custom URL
    async fn call_openai_with_url(&self, prompt: &str, url: &str) -> Result<String> {
        let model = if self.config.model.is_empty() {
            "gpt-4o"
        } else {
            &self.config.model
        };

        let request = json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert technical writer and analyst. Generate engaging, insightful scripts from development sessions."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 4000
        });

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        Ok(result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Error: No response from AI")
            .to_string())
    }

    /// Generate fallback script without AI
    fn generate_fallback_script(input: &ScriptGenerationInput) -> String {
        let mut md = String::new();

        md.push_str("# 🎭 Development Session Script\n\n");
        md.push_str("*Note: AI analysis not available. Configure with `exg config` for enhanced scripts.\n\n");

        md.push_str(&format!("**Duration:** {} seconds\n", (input.session_end - input.session_start).num_seconds()));
        md.push_str(&format!("**Events:** {}\n\n", input.events.len()));

        md.push_str("## Act 1 - Development Session\n\n");
        md.push_str("### Scene 1 - Session Overview\n\n");
        md.push_str("The developer worked on the project with the following tech stack:\n\n");
        for tech in &input.project_context.tech_stack {
            md.push_str(&format!("- {}\n", tech));
        }
        md.push_str("\n");

        if !input.events.is_empty() {
            md.push_str("### Key Events\n\n");
            for (i, event) in input.events.iter().enumerate() {
                md.push_str(&format!("#### {} - {}\n\n", i + 1, event.timestamp));
                md.push_str(&format!("**Event:** {}\n", event.event_type));
                md.push_str(&format!("**Description:** {}\n\n", event.description));
            }
        }

        md.push_str("---\n*Generated by EX-G-SE*\n");
        md
    }
}

/// Extract project context from working directory with enhanced info
pub fn extract_project_context() -> ProjectContext {
    let mut context = ProjectContext {
        name: None,
        description: None,
        tech_stack: vec![],
        file_structure: vec![],
        recent_changes: vec![],
        claude_md_context: None,
    };

    // Try to read CLAUDE.md first
    if let Ok(claude_md) = fs::read_to_string("CLAUDE.md") {
        // Extract first 2000 chars (using char boundaries to avoid cutting UTF-8)
        let preview = if claude_md.chars().count() > 2000 {
            let truncated: String = claude_md.chars().take(2000).collect();
            format!("{}...\n\n[{} chars total]", truncated, claude_md.len())
        } else {
            claude_md.clone()
        };
        context.claude_md_context = Some(preview);
    }

    // Try to read package.json
    if let Ok(pkg) = fs::read_to_string("package.json") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&pkg) {
            context.name = json["name"].as_str().map(|s| s.to_string());
            context.description = json["description"].as_str().map(|s| s.to_string());

            if let Some(deps) = json["dependencies"].as_object() {
                for dep in deps.keys().take(10) {
                    context.tech_stack.push(dep.to_string());
                }
            }
        }
    }

    // Try to read Cargo.toml
    if let Ok(cargo) = fs::read_to_string("Cargo.toml") {
        if cargo.contains("dependencies") {
            context.tech_stack.push("rust".to_string());
        }
    }

    // List project files
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.take(20) {
            if let Ok(entry) = entry {
                if let Ok(name) = entry.file_name().into_string() {
                    if !name.starts_with('.') && name != "node_modules" && name != "target" && name != ".git" {
                        context.file_structure.push(name);
                    }
                }
            }
        }
    }

    context
}

/// Extract modified files with their content
pub fn extract_code_files(events: &[ScriptEvent]) -> Vec<CodeFile> {
    let mut code_files = Vec::new();
    let mut processed_paths = std::collections::HashSet::new();

    for event in events {
        if event.event_type == "fs_change" {
            if let Some(path) = event.details.get("path") {
                if let Some(path_str) = path.as_str() {
                    // Skip if already processed
                    if processed_paths.contains(path_str) {
                        continue;
                    }
                    processed_paths.insert(path_str.to_string());

                    // Only process text-based files
                    if let Ok(content) = fs::read_to_string(path_str) {
                        // Limit size to avoid huge files
                        if content.len() > 100_000 {
                            continue;
                        }

                        let language = detect_language(path_str);
                        code_files.push(CodeFile {
                            path: path_str.to_string(),
                            content,
                            language,
                        });
                    }
                }
            }
        }
    }

    code_files
}

/// Detect programming language from file extension
fn detect_language(path: &str) -> String {
    if path.ends_with(".rs") {
        "rust".to_string()
    } else if path.ends_with(".js") || path.ends_with(".jsx") {
        "javascript".to_string()
    } else if path.ends_with(".ts") || path.ends_with(".tsx") {
        "typescript".to_string()
    } else if path.ends_with(".py") {
        "python".to_string()
    } else if path.ends_with(".go") {
        "go".to_string()
    } else if path.ends_with(".java") {
        "java".to_string()
    } else if path.ends_with(".c") || path.ends_with(".h") {
        "c".to_string()
    } else if path.ends_with(".cpp") || path.ends_with(".hpp") {
        "cpp".to_string()
    } else if path.ends_with(".md") {
        "markdown".to_string()
    } else if path.ends_with(".json") {
        "json".to_string()
    } else {
        "text".to_string()
    }
}
