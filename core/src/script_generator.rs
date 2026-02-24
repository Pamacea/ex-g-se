//! AI-Powered Script Generator
//!
//! This module analyzes development sessions and generates
//! theatrical scripts using AI to capture:
//! - Developer thoughts and intentions
//! - Why certain choices were made
//! - Technical context and decisions
//! - Narrative flow of the session
//! - Claude Code conversation (prompts and responses)

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use crate::claude_context::{PromptEntry, ResponseEntry, ToolCall};

/// Script generation input with enhanced context
#[derive(Debug, Clone, Serialize)]
pub struct ScriptGenerationInput {
    pub session_start: DateTime<Utc>,
    pub session_end: DateTime<Utc>,
    pub events: Vec<ScriptEvent>,
    pub project_context: ProjectContext,
    pub code_context: Vec<CodeFile>,
    pub screenshots: Vec<ScriptScreenshotInfo>,

    // Claude Code conversation context
    #[serde(default)]
    pub user_prompts: Vec<PromptEntry>,
    #[serde(default)]
    pub assistant_responses: Vec<ResponseEntry>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
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

    /// Load style guide from ~/.config/ex-g-se/EXG.md if exists
    fn load_style_guide(&self) -> Option<String> {
        let config_dir = dirs::home_dir()?
            .join(".config")
            .join("ex-g-se");

        let style_path = config_dir.join("EXG.md");

        if !style_path.exists() {
            return None;
        }

        match fs::read_to_string(&style_path) {
            Ok(content) => {
                eprintln!("[Style] ✓ Loaded style guide from EXG.md ({} chars)", content.len());
                Some(content)
            }
            Err(e) => {
                eprintln!("[Style] ⚠ Failed to load EXG.md: {}", e);
                None
            }
        }
    }

    /// Generate script from session data with full AI analysis
    pub async fn generate(&self, input: &ScriptGenerationInput) -> Result<String> {
        if !self.is_configured() {
            eprintln!("[AI] No API configuration found, using fallback script");
            return Ok(Self::generate_fallback_script(input));
        }

        let prompt = self.build_detailed_prompt(input);

        eprintln!("[AI] Generating script with provider: {}", self.config.provider);

        // Simplified: only Anthropic is special, everyone else uses OpenAI-compatible
        let script_content = if self.config.provider == "anthropic" {
            self.call_anthropic(&prompt).await?
        } else {
            // Works for: openai, zai, z.ai, together, groq, and any OpenAI-compatible provider
            self.call_openai_compatible(&prompt).await?
        };

        if script_content.contains("Error:") || script_content.contains("**Error:**") {
            eprintln!("[AI] API returned an error response");
        } else {
            eprintln!("[AI] Script generated successfully ({} chars)", script_content.len());
        }

        Ok(script_content)
    }

    /// Build detailed prompt for AI with two-part output (theatrical + social media)
    fn build_detailed_prompt(&self, input: &ScriptGenerationInput) -> String {
        // 🆕 Generate conversation first and use it as context
        let conversation_md = generate_conversation_markdown(input);

        let mut prompt = String::new();

        // Use conversation as the primary context
        prompt.push_str("# Development Session Content Generation\n\n");
        prompt.push_str("Below is the complete conversation and context from a development session.\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str(&conversation_md);
        prompt.push_str("\n---\n\n");

        // 🆕 Load and inject style guide if exists
        if let Some(style_guide) = self.load_style_guide() {
            prompt.push_str("\n## ========================================\n");
            prompt.push_str("## STYLE GUIDE (User Preferences)\n");
            prompt.push_str("## ========================================\n\n");
            prompt.push_str(&style_guide);
            prompt.push_str("\n\n---\n");
            prompt.push_str("**IMPORTANT:** Apply this style guide to ALL generated content below!\n");
            prompt.push_str("---\n\n");
        }

        // ========================================
        // PART 1: Theatrical Script for Video
        // ========================================
        prompt.push_str("\n## ========================================\n");
        prompt.push_str("## PART 1: THEATRICAL SCRIPT (FOR VIDEO)\n");
        prompt.push_str("## ========================================\n\n");

        prompt.push_str("Generate an engaging **theatrical script** that tells the story of this development session.\n\n");

        prompt.push_str("### Requirements:\n\n");
        prompt.push_str("1. **Structure:** Use Acts and Scenes (theatrical format)\n");
        prompt.push_str("2. **Narrative:** Tell the STORY of what happened during the session\n");
        prompt.push_str("3. **Real Prompts:** Quote the ACTUAL user prompts from the conversation\n");
        prompt.push_str("4. **Thoughts:** Capture the developer's INTENTIONS and REASONING\n");
        prompt.push_str("5. **Technical:** Explain WHY choices were made based on the files modified\n");
        prompt.push_str("6. **Visual:** Describe what's visible in screenshots\n");
        prompt.push_str("7. **Context:** Use the project info and code changes to understand the domain\n\n");

        prompt.push_str("### Format:\n\n");
        prompt.push_str("# [Engaging Title that captures the essence]\n\n");
        prompt.push_str("## Act 1 - [The Purpose/Goal]\n\n");
        prompt.push_str("*Setting the scene: What is the developer trying to accomplish?*\n\n");
        prompt.push_str("### Scene 1 - [HH:MM:SS] - [Brief Moment Title]\n\n");
        prompt.push_str("**[Stage Direction]**\n\n");
        prompt.push_str("The developer [action]. [Technical detail].\n\n");
        prompt.push_str("**User's Prompt to Claude:**\n\n");
        prompt.push_str("> [Actual prompt text from conversation]\n\n");
        prompt.push_str("**Developer's Thoughts:**\n\n");
        prompt.push_str("> [Internal monologue - what they're thinking and why]\n\n");
        prompt.push_str("**Technical Context:**\n\n");
        prompt.push_str("> [Relevant technical details from the code/project]\n\n");
        prompt.push_str("---\n\n");

        // ========================================
        // PART 2: Social Media Posts
        // ========================================
        prompt.push_str("\n## ========================================\n");
        prompt.push_str("## PART 2: SOCIAL MEDIA POSTS\n");
        prompt.push_str("## ========================================\n\n");

        prompt.push_str("Now generate social media posts based on this development session.\n\n");
        prompt.push_str("### Generate posts for:\n\n");
        prompt.push_str("1. **LinkedIn Post** - Professional, detailed, showcases technical achievement\n");
        prompt.push_str("   - Focus on business value, technical challenge, solution\n");
        prompt.push_str("   - 3-5 paragraphs, professional tone\n");
        prompt.push_str("   - Include hashtags like #Development #Coding #Tech\n\n");

        prompt.push_str("2. **Twitter/X Thread** - Concise, thread-style, technical highlights\n");
        prompt.push_str("   - Main tweet + 2-3 reply tweets\n");
        prompt.push_str("   - Focus on key insights, quick wins, interesting discoveries\n");
        prompt.push_str("   - Use tech hashtags, keep it punchy\n\n");

        prompt.push_str("3. **Bluesky Post** - Developer community focused, authentic\n");
        prompt.push_str("   - Share learnings, what worked/didn't work\n");
        prompt.push_str("   - Casual but technical tone\n\n");

        prompt.push_str("4. **Dev.to/Hashnode Article** - Tutorial style, educational\n");
        prompt.push_str("   - Title: How I [solved X] using [Y]\n");
        prompt.push_str("   - Structure: Problem → Solution → Code Examples → Takeaway\n");
        prompt.push_str("   - Include code snippets from the session\n\n");

        prompt.push_str("5. **Mastodon Post** - Open source community, transparent\n");
        prompt.push_str("   - Share the journey, lessons learned\n");
        prompt.push_str("   - Include relevant hashtags\n\n");

        prompt.push_str("6. **Personal Blog Summary** - Reflective, detailed\n");
        prompt.push_str("   - What I built, why, how, challenges faced\n");
        prompt.push_str("   - Technical deep-dive\n\n");

        prompt.push_str("### Content Guidelines:\n\n");
        prompt.push_str("- **Be Authentic:** Share real challenges and learnings\n");
        prompt.push_str("- **Be Specific:** Reference actual files, technologies, decisions\n");
        prompt.push_str("- **Be Engaging:** Hook readers with interesting insights\n");
        prompt.push_str("- **Add Value:** Teach something from your experience\n");
        prompt.push_str("- **Use Emojis:** Sparingly but effectively for visual appeal\n\n");
        prompt.push_str("- **Include Call-to-Actions:** Ask questions, encourage discussion\n\n");

        prompt.push_str("### Format for Social Posts:\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 🚀 LinkedIn\n\n");
        prompt.push_str("[Post content here]\n\n");
        prompt.push_str("#hashtags #Tech\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 🐦 Twitter/X\n\n");
        prompt.push_str("**Tweet 1/3:** [Main insight]\n\n");
        prompt.push_str("**Reply 1:** [Elaboration]\n\n");
        prompt.push_str("**Reply 2:** [Call to action]\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 🦋 Bluesky\n\n");
        prompt.push_str("[Post content here]\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 📝 Dev.to/Hashnode\n\n");
        prompt.push_str("**Title:** [Catchy title]\n\n");
        prompt.push_str("[Article content with code blocks]\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 🐘 Mastodon\n\n");
        prompt.push_str("[Post content here]\n\n");
        prompt.push_str("---\n\n");
        prompt.push_str("### 📝 Personal Blog Summary\n\n");
        prompt.push_str("[Summary content]\n\n");
        prompt.push_str("---\n\n");

        prompt.push_str("**Make each platform's content unique and tailored to that audience!**\n\n");

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

    /// Call OpenAI-compatible API (works with OpenAI, Z.AI, Together, Groq, etc.)
    async fn call_openai_compatible(&self, prompt: &str) -> Result<String> {
        // Build URL with fallback
        let url = if self.config.api_url.is_empty() {
            "https://api.openai.com/v1/chat/completions".to_string()
        } else {
            // Handle both full URLs and base URLs
            let base = self.config.api_url.trim_end_matches('/');
            if base.contains("/chat/completions") {
                base.to_string()
            } else {
                format!("{}/chat/completions", base)
            }
        };

        // Default models per provider
        let model = if self.config.model.is_empty() {
            match self.config.provider.as_str() {
                "zai" | "z.ai" => "glm-5",
                "together" => "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo",
                "groq" => "llama-3.3-70b-versatile",
                _ => "gpt-4o"
            }
        } else {
            &self.config.model
        };

        // Build request (standard OpenAI format)
        let mut request = json!({
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

        // Add provider-specific extras
        if self.config.provider == "zai" || self.config.provider == "z.ai" {
            if let Some(obj) = request.as_object_mut() {
                obj.insert("thinking".to_string(), json!({"type": "enabled"}));
                obj.insert("temperature".to_string(), json!(1.0));
                obj.insert("max_tokens".to_string(), json!(4096));
            }
        }

        eprintln!("[AI] Calling {} API: {}", self.config.provider.to_uppercase(), url);
        eprintln!("[AI] Model: {}", model);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        eprintln!("[AI] Status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Ok(format!("**Error:** API returned status {} - {}\n\n**Response:**\n```\n{}\n```\n\n**Tip:** Check your API key and model at your provider's dashboard",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown"),
                error_text
            ));
        }

        let result: serde_json::Value = response.json().await?;

        // Extract content from standard OpenAI format
        if let Some(content) = result["choices"][0]["message"]["content"].as_str() {
            eprintln!("[AI] ✓ Script generated ({} chars)", content.len());
            return Ok(content.to_string());
        }

        // Fallback: try alternative formats
        if let Some(content) = result["output"]["choices"][0]["message"]["content"].as_str() {
            eprintln!("[AI] ✓ Script generated from alternative format ({} chars)", content.len());
            return Ok(content.to_string());
        }

        // Error: unexpected format
        Ok(format!("**Error: Unexpected response format**\n\n**Response Structure:**\n```json\n{}\n```\n\n**Expected:** `result[\"choices\"][0][\"message\"][\"content\"]`\n\n**Provider:** {} ({})\n\n**Tip:** This provider may have changed their API format. Check their documentation.",
            serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Cannot serialize".to_string()),
            self.config.provider,
            url
        ))
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

/// Generate conversation.md with full Claude Code conversation
pub fn generate_conversation_markdown(input: &ScriptGenerationInput) -> String {
    let mut md = String::new();

    let duration = (input.session_end - input.session_start).num_seconds();
    let duration_str = if duration >= 60 {
        format!("{}m {}s", duration / 60, duration % 60)
    } else {
        format!("{}s", duration)
    };

    // Header
    md.push_str("# 💬 Claude Code Conversation\n\n");
    md.push_str("---\n\n");
    md.push_str(&format!("**Session Started:** {}\n", input.session_start.format("%Y-%m-%d %H:%M:%S UTC")));
    md.push_str(&format!("**Session Ended:** {}\n", input.session_end.format("%Y-%m-%d %H:%M:%S UTC")));
    md.push_str(&format!("**Duration:** {}\n", duration_str));
    md.push_str(&format!("**Total Events:** {}\n\n", input.events.len()));

    // Project context
    md.push_str("## 📁 Project Context\n\n");
    if let Some(name) = &input.project_context.name {
        md.push_str(&format!("**Project:** {}\n", name));
    }
    if !input.project_context.tech_stack.is_empty() {
        md.push_str(&format!("**Tech Stack:** {}\n", input.project_context.tech_stack.join(", ")));
    }
    md.push_str("\n---\n\n");

    // User prompts
    if !input.user_prompts.is_empty() {
        md.push_str("## 🎯 User Prompts\n\n");
        md.push_str(&format!("**Total Prompts:** {}\n\n", input.user_prompts.len()));

        for (i, prompt) in input.user_prompts.iter().enumerate() {
            md.push_str(&format!("### Prompt {} - [{}]\n\n", i + 1, prompt.timestamp));
            md.push_str(&format!("{}\n\n", prompt.text));

            if let Some(project) = &prompt.project {
                md.push_str(&format!("*Project: {}*\n\n", project));
            }
        }

        md.push_str("---\n\n");
    }

    // Assistant responses
    if !input.assistant_responses.is_empty() {
        md.push_str("## 🤖 Assistant Responses\n\n");
        md.push_str(&format!("**Total Responses:** {}\n\n", input.assistant_responses.len()));

        for (i, response) in input.assistant_responses.iter().enumerate() {
            md.push_str(&format!("### Response {} - [{}]\n\n", i + 1, response.timestamp));

            // Truncate very long responses
            let content = if response.content.len() > 5000 {
                format!("{}...\n\n*[{} chars total - truncated]*",
                    response.content.chars().take(5000).collect::<String>(),
                    response.content.len()
                )
            } else {
                response.content.clone()
            };

            md.push_str(&format!("{}\n\n", content));
        }

        md.push_str("---\n\n");
    }

    // Tool calls timeline
    if !input.tool_calls.is_empty() {
        md.push_str("## 🔧 Tool Calls Timeline\n\n");
        md.push_str(&format!("**Total Tool Calls:** {}\n\n", input.tool_calls.len()));

        // Group by tool type
        let mut tool_groups: std::collections::HashMap<String, Vec<&ToolCall>> = std::collections::HashMap::new();
        for call in &input.tool_calls {
            tool_groups.entry(call.tool_name.clone()).or_default().push(call);
        }

        for (tool_name, calls) in tool_groups.iter() {
            md.push_str(&format!("### {} ({} calls)\n\n", tool_name, calls.len()));

            for call in calls {
                md.push_str(&format!("- **[{}]** {}\n", call.timestamp, call.description));
            }
            md.push_str("\n");
        }

        md.push_str("---\n\n");
    }

    // File changes
    if !input.code_context.is_empty() {
        md.push_str("## 📝 Files Modified\n\n");
        md.push_str(&format!("**Total Files:** {}\n\n", input.code_context.len()));

        // Show full content for small files, preview for large files
        for file in &input.code_context {
            md.push_str(&format!("### `{}`\n\n", file.path));
            md.push_str(&format!("**Language:** {}\n", file.language));
            md.push_str(&format!("**Size:** {} chars\n\n", file.content.len()));

            // Show more content - up to 3000 chars or full file if small
            let preview = if file.content.chars().count() > 3000 {
                let truncated: String = file.content.chars().take(3000).collect();
                format!("{}\n\n...[{} more chars, truncated]\n", truncated, file.content.chars().count() - 3000)
            } else {
                file.content.clone()
            };

            md.push_str(&format!("**Content:**\n```\n{}\n```\n\n", preview));
        }

        md.push_str("---\n\n");
    }

    // Session timeline (summary)
    md.push_str("## 📊 Session Timeline Summary\n\n");

    let mut event_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for event in &input.events {
        *event_counts.entry(event.event_type.clone()).or_insert(0) += 1;
    }

    for (event_type, count) in event_counts.iter() {
        md.push_str(&format!("- **{}**: {}\n", event_type, count));
    }

    md.push_str("\n---\n\n");
    md.push_str("*Generated by EX-G-SE v0.6.0*\n");

    md
}
