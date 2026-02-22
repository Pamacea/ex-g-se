// AI Module - Anthropic Provider Implementation
//
// This module implements the AI provider trait for Anthropic's Claude API.

use super::error::{AIError, AIResult};
use super::types::{AnalysisInput, Script, ScriptInput, SessionAnalysis};
use crate::ai::AIConfig;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ============================================================================
// Anthropic API Types
// ============================================================================

/// Content block for Anthropic messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ContentBlock {
    Text { text: String },
    Image { source: ImageSource },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageSource {
    #[serde(rename = "type")]
    media_type: String,
    data: String, // base64 encoded
}

/// Message in the Anthropic format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: Vec<ContentBlock>,
}

impl Message {
    fn user(text: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: text.into(),
            }],
        }
    }

    fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: vec![ContentBlock::Text {
                text: text.into(),
            }],
        }
    }
}

/// Request payload for Anthropic messages API
#[derive(Debug, Serialize)]
struct MessagesRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: Option<f32>,
    system: Option<String>,
}

/// Response from Anthropic messages API
#[derive(Debug, Deserialize)]
struct MessagesResponse {
    id: String,
    #[serde(rename = "type")]
    response_type: String,
    role: String,
    content: Vec<ContentBlock>,
    model: String,
    stop_reason: Option<String>,
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

/// API error response
#[derive(Debug, Deserialize)]
struct AnthropicErrorResponse {
    #[serde(rename = "type")]
    error_type: String,
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    #[serde(rename = "type")]
    detail_type: String,
    message: String,
}

// ============================================================================
// Anthropic Provider
// ============================================================================

/// Anthropic provider implementation
#[derive(Debug, Clone)]
pub struct AnthropicProvider {
    config: AIConfig,
    client: reqwest::Client,
}

impl AnthropicProvider {
    /// Anthropic API version
    const API_VERSION: &'static str = "2023-06-01";

    /// Create a new Anthropic provider
    pub fn new(config: AIConfig) -> AIResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Self { config, client })
    }

    /// Get the base URL for Anthropic API
    fn base_url(&self) -> &str {
        self.config.api_url.as_deref().unwrap_or("https://api.anthropic.com/v1")
    }

    /// Build the system prompt for session analysis
    fn build_analysis_system_prompt(&self) -> String {
        "You are an expert code analyst with deep knowledge of software development practices, \
        patterns, and workflows. Your task is to analyze development sessions and extract \
        meaningful insights.\n\n\
        Always respond with valid JSON only. Do not use markdown formatting or code blocks.\n\n\
        Focus on:\n\
        - Identifying the developer's intent and goals\n\
        - Extracting key moments and breakthroughs\n\
        - Recognizing patterns in the workflow\n\
        - Providing actionable insights\n\n\
        Be specific and precise in your analysis.".to_string()
    }

    /// Build the user prompt for session analysis
    fn build_analysis_user_prompt(&self, input: &AnalysisInput) -> String {
        format!(
            "Analyze the following development session and provide structured insights.\n\n\
            Session Details:\n\
            - Start: {}\n\
            - End: {}\n\
            - Duration: {} minutes\n\
            {}\n\n\
            Session Logs:\n\
            {}\n\n\
            Please respond with a JSON object containing:\n\
            - intents: Array of primary intents (FeatureDevelopment, BugFixing, Refactoring, etc.)\n\
            - key_moments: Array of significant moments with timestamps, titles, and descriptions\n\
            - patterns: Array of detected workflow patterns\n\
            - summary: 2-3 sentence summary of the session\n\
            - suggested_title: A descriptive title for the session\n\
            - technologies: Array of technologies and tools detected\n\
            - files_modified: Array of files that were modified\n\
            - confidence: Float from 0.0 to 1.0 indicating analysis confidence",
            input.start_time.format("%Y-%m-%d %H:%M:%S UTC"),
            input.end_time.format("%Y-%m-%d %H:%M:%S UTC"),
            (input.end_time - input.start_time).num_minutes().max(0),
            input.project_context.as_ref().map_or(String::new(), |ctx| format!("Project Context: {}\n", ctx)),
            serde_json::to_string_pretty(&input.logs).unwrap_or_else(|_| "{}".to_string())
        )
    }

    /// Build the system prompt for script generation
    fn build_script_system_prompt(&self) -> String {
        "You are an expert scriptwriter specializing in creating engaging content about \
        technology and software development. You have a talent for making technical concepts \
        accessible and entertaining.\n\n\
        Always respond with valid JSON only. Do not use markdown formatting or code blocks.\n\n\
        Create scripts with:\n\
        - Compelling narratives and dialogue\n\
        - Clear three-act structure\n\
        - Well-defined characters with distinct voices\n\
        - Appropriate pacing and tension\n\
        - Technical accuracy wrapped in entertainment".to_string()
    }

    /// Build the user prompt for script generation
    fn build_script_user_prompt(&self, input: &ScriptInput) -> String {
        let style_desc = input.style.as_deref().unwrap_or("educational and engaging");
        let audience_desc = input.audience.as_deref().unwrap_or("developers");

        format!(
            "Create a compelling script based on the following development session analysis.\n\n\
            Session Analysis:\n\
            - Title: {}\n\
            - Summary: {}\n\
            - Intents: {:?}\n\
            - Key Moments: {}\n\
            - Patterns: {:?}\n\
            - Technologies: {:?}\n\
            - Confidence: {:.2}\n\n\
            Script Requirements:\n\
            - Style: {}\n\
            - Target Audience: {}\n\
            - Format: 2-3 acts with multiple scenes each\n\
            - Include natural dialogue and stage directions\n\n\
            Please respond with a JSON object containing:\n\
            - title: Script title\n\
            - tagline: Optional tagline\n\
            - logline: One-sentence summary\n\
            - acts: Array of acts with scenes\n\
            - characters: Array of character names\n\
            - genre: Script genre\n\
            - estimated_runtime_minutes: Estimated runtime",
            input.analysis.suggested_title,
            input.analysis.summary,
            input.analysis.intents,
            input.analysis.key_moments.len(),
            input.analysis.patterns,
            input.analysis.technologies,
            input.analysis.confidence,
            style_desc,
            audience_desc
        )
    }

    /// Send a messages API request
    async fn send_message(&self, system: Option<String>, messages: Vec<Message>) -> AIResult<String> {
        let url = format!("{}/messages", self.base_url());

        // Anthropic requires max_tokens to be set
        let max_tokens = self.config.max_tokens.unwrap_or(4096);

        let request = MessagesRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens,
            temperature: self.config.temperature,
            system,
        };

        let mut request_builder = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", Self::API_VERSION)
            .header("Content-Type", "application/json");

        request_builder = request_builder.json(&request);

        let response = request_builder.send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            // Try to parse as API error
            if let Ok(err_resp) = serde_json::from_str::<AnthropicErrorResponse>(&body) {
                return Err(AIError::api_error(
                    status.as_u16(),
                    err_resp.error.message,
                ));
            }
            return Err(AIError::api_error(status.as_u16(), body));
        }

        let completion: MessagesResponse = serde_json::from_str(&body).map_err(|e| {
            AIError::parse_error(format!("Failed to parse response: {} (body: {})", e, body))
        })?;

        // Extract text from content blocks
        for block in completion.content {
            if let ContentBlock::Text { text } = block {
                return Ok(text);
            }
        }

        Err(AIError::parse_error("No text content in response"))
    }
}

#[async_trait]
impl super::AIProvider for AnthropicProvider {
    /// Analyze a development session
    async fn analyze_session(&self, input: &AnalysisInput) -> AIResult<SessionAnalysis> {
        let system = self.build_analysis_system_prompt();
        let user_prompt = self.build_analysis_user_prompt(input);

        let messages = vec![Message::user(user_prompt)];

        let response = self.send_message(Some(system), messages).await?;

        // Parse the JSON response
        let analysis: serde_json::Value = serde_json::from_str(&response).map_err(|e| {
            AIError::parse_error(format!("Invalid JSON in response: {} (response: {})", e, response))
        })?;

        Ok(SessionAnalysis {
            session_start: input.start_time,
            session_end: input.end_time,
            intents: serde_json::from_value(analysis["intents"].clone()).unwrap_or_default(),
            key_moments: serde_json::from_value(analysis["key_moments"].clone())
                .unwrap_or_default(),
            patterns: serde_json::from_value(analysis["patterns"].clone()).unwrap_or_default(),
            summary: analysis["summary"]
                .as_str()
                .unwrap_or("No summary available")
                .to_string(),
            suggested_title: analysis["suggested_title"]
                .as_str()
                .unwrap_or("Untitled Session")
                .to_string(),
            technologies: serde_json::from_value(analysis["technologies"].clone())
                .unwrap_or_default(),
            files_modified: serde_json::from_value(analysis["files_modified"].clone())
                .unwrap_or_default(),
            confidence: analysis["confidence"].as_f64().unwrap_or(0.5) as f32,
        })
    }

    /// Generate a script from session analysis
    async fn generate_script(&self, input: &ScriptInput) -> AIResult<Script> {
        let system = self.build_script_system_prompt();
        let user_prompt = self.build_script_user_prompt(input);

        let messages = vec![Message::user(user_prompt)];

        let response = self.send_message(Some(system), messages).await?;

        // Parse the JSON response
        let script: serde_json::Value = serde_json::from_str(&response).map_err(|e| {
            AIError::parse_error(format!("Invalid JSON in response: {} (response: {})", e, response))
        })?;

        Ok(Script {
            title: script["title"]
                .as_str()
                .unwrap_or("Untitled Script")
                .to_string(),
            tagline: script["tagline"].as_str().map(|s| s.to_string()),
            logline: script["logline"]
                .as_str()
                .unwrap_or("No logline")
                .to_string(),
            acts: serde_json::from_value(script["acts"].clone()).unwrap_or_default(),
            characters: serde_json::from_value(script["characters"].clone())
                .unwrap_or_default(),
            genre: script["genre"].as_str().map(|s| s.to_string()),
            estimated_runtime_minutes: script["estimated_runtime_minutes"].as_u64().map(|v| v as u32),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_message_creation() {
        let user_msg = Message::user("User prompt");
        assert_eq!(user_msg.role, "user");
        assert!(matches!(user_msg.content[0], ContentBlock::Text { .. }));

        let asst_msg = Message::assistant("Assistant response");
        assert_eq!(asst_msg.role, "assistant");
    }

    #[test]
    fn test_anthropic_provider_creation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::Anthropic,
            api_key: "test-key".to_string(),
            api_url: Some("https://api.anthropic.com/v1".to_string()),
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: Some(4096),
            temperature: Some(0.7),
        };

        let provider = AnthropicProvider::new(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_system_prompt_generation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::Anthropic,
            api_key: "test-key".to_string(),
            api_url: None,
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let provider = AnthropicProvider::new(config).unwrap();
        let system_prompt = provider.build_analysis_system_prompt();

        assert!(system_prompt.contains("expert code analyst"));
        assert!(system_prompt.contains("valid JSON only"));
    }

    #[test]
    fn test_user_prompt_generation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::Anthropic,
            api_key: "test-key".to_string(),
            api_url: None,
            model: "claude-3-opus-20240229".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let provider = AnthropicProvider::new(config).unwrap();

        let input = AnalysisInput {
            logs: serde_json::json!({"test": "data"}),
            start_time: Utc::now(),
            end_time: Utc::now(),
            project_context: Some("Test project".to_string()),
            screenshots: None,
        };

        let user_prompt = provider.build_analysis_user_prompt(&input);
        assert!(user_prompt.contains("Analyze the following development session"));
        assert!(user_prompt.contains("Test project"));
    }
}
