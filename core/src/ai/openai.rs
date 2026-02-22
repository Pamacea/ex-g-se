// AI Module - OpenAI Provider Implementation
//
// This module implements the AI provider trait for OpenAI's API.

use super::error::{AIError, AIResult};
use super::types::{AnalysisInput, Script, ScriptInput, SessionAnalysis};
use crate::ai::AIConfig;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ============================================================================
// OpenAI API Types
// ============================================================================

/// Message in the chat completion format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

impl ChatMessage {
    fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// Request payload for chat completions
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    stream: Option<bool>,
}

/// Response from chat completions API
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// API error response
#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
}

// ============================================================================
// OpenAI Provider
// ============================================================================

/// OpenAI provider implementation
#[derive(Debug, Clone)]
pub struct OpenAIProvider {
    config: AIConfig,
    client: reqwest::Client,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider
    pub fn new(config: AIConfig) -> AIResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Self { config, client })
    }

    /// Get the base URL for OpenAI API
    fn base_url(&self) -> &str {
        self.config.api_url.as_deref().unwrap_or("https://api.openai.com/v1")
    }

    /// Build the system prompt for session analysis
    fn build_analysis_prompt(&self, input: &AnalysisInput) -> String {
        format!(
            "You are an expert code analyst. Analyze the following development session and provide insights.

Session Context:
- Start: {}
- End: {}
- Duration: {} minutes
{}

Your task is to:
1. Identify the primary intents (feature development, bug fixing, refactoring, etc.)
2. Extract key moments (significant events, breakthroughs, errors)
3. Detect patterns in the workflow
4. Provide a concise summary
5. Suggest a title for the session
6. Identify technologies used
7. List files that were modified

Respond in JSON format with the following structure:
{{
  \"intents\": [\"FeatureDevelopment\", \"BugFixing\"],
  \"key_moments\": [
    {{
      \"timestamp\": \"2024-01-01T12:00:00Z\",
      \"title\": \"Breakthrough moment\",
      \"description\": \"Detailed description\",
      \"related_entries\": [0, 1, 2]
    }}
  ],
  \"patterns\": [\"test-driven development\", \"iterative refinement\"],
  \"summary\": \"2-3 sentence summary\",
  \"suggested_title\": \"Descriptive session title\",
  \"technologies\": [\"Rust\", \"Tokio\"],
  \"files_modified\": [\"src/main.rs\", \"src/lib.rs\"],
  \"confidence\": 0.85
}}

Session Logs:
{}",
            input.start_time.format("%Y-%m-%d %H:%M:%S UTC"),
            input.end_time.format("%Y-%m-%d %H:%M:%S UTC"),
            (input.end_time - input.start_time).num_minutes().max(0),
            input.project_context.as_ref().map_or(String::new(), |ctx| format!("Project Context: {}\n", ctx)),
            serde_json::to_string_pretty(&input.logs).unwrap_or_else(|_| "{}".to_string())
        )
    }

    /// Build the system prompt for script generation
    fn build_script_prompt(&self, input: &ScriptInput) -> String {
        let style = input.style.as_deref().unwrap_or("educational and engaging");
        let audience = input.audience.as_deref().unwrap_or("developers");

        format!(
            "You are an expert scriptwriter. Create a compelling script based on the following session analysis.

Session Analysis:
- Title: {}
- Summary: {}
- Intents: {:?}
- Key Moments: {}
- Patterns: {:?}

Your task is to create a script with:
1. A catchy title and logline
2. 2-3 acts, each with multiple scenes
3. Natural dialogue between characters
4. Clear stage directions and actions
5. A defined genre and style

The script should be {} and target {}.

Respond in JSON format with the following structure:
{{
  \"title\": \"Script Title\",
  \"tagline\": \"Optional tagline\",
  \"logline\": \"One-sentence summary\",
  \"acts\": [
    {{
      \"number\": 1,
      \"title\": \"Act I\",
      \"description\": \"Act description\",
      \"theme\": \"Act theme\",
      \"scenes\": [
        {{
          \"number\": 1,
          \"title\": \"Scene title\",
          \"description\": \"Scene description\",
          \"timestamp\": \"2024-01-01T12:00:00Z\",
          \"dialogue\": [
            {{
              \"speaker\": \"Developer\",
              \"text\": \"Dialogue text\",
              \"action\": \"Optional action\"
            }}
          ],
          \"related_moments\": [0, 1]
        }}
      ]
    }}
  ],
  \"characters\": [\"Developer\", \"AI Assistant\"],
  \"genre\": \"Tech Comedy\",
  \"estimated_runtime_minutes\": 15
}}",
            input.analysis.suggested_title,
            input.analysis.summary,
            input.analysis.intents,
            input.analysis.key_moments.len(),
            input.analysis.patterns,
            style,
            audience
        )
    }

    /// Send a chat completion request
    async fn chat_completion(&self, messages: Vec<ChatMessage>) -> AIResult<String> {
        let url = format!("{}/chat/completions", self.base_url());

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: Some(false),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            // Try to parse as API error
            if let Ok(err_resp) = serde_json::from_str::<ApiErrorResponse>(&body) {
                return Err(AIError::api_error(status.as_u16(), err_resp.error.message));
            }
            return Err(AIError::api_error(status.as_u16(), body));
        }

        let completion: ChatCompletionResponse =
            serde_json::from_str(&body).map_err(|e| AIError::parse_error(format!(
                "Failed to parse response: {} (body: {})",
                e, body
            )))?;

        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(AIError::parse_error("No choices in response"))
        }
    }
}

#[async_trait]
impl super::AIProvider for OpenAIProvider {
    /// Analyze a development session
    async fn analyze_session(&self, input: &AnalysisInput) -> AIResult<SessionAnalysis> {
        let system_prompt = "You are an expert code analyst. Always respond with valid JSON only, no markdown formatting.";
        let user_prompt = self.build_analysis_prompt(input);

        let messages = vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ];

        let response = self.chat_completion(messages).await?;

        // Parse the JSON response
        let analysis: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| AIError::parse_error(format!("Invalid JSON in response: {}", e)))?;

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
        let system_prompt = "You are an expert scriptwriter. Always respond with valid JSON only, no markdown formatting.";
        let user_prompt = self.build_script_prompt(input);

        let messages = vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ];

        let response = self.chat_completion(messages).await?;

        // Parse the JSON response
        let script: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| AIError::parse_error(format!("Invalid JSON in response: {}", e)))?;

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
    fn test_chat_message_creation() {
        let sys = ChatMessage::system("System prompt");
        assert_eq!(sys.role, "system");
        assert_eq!(sys.content, "System prompt");

        let user = ChatMessage::user("User prompt");
        assert_eq!(user.role, "user");

        let asst = ChatMessage::assistant("Assistant response");
        assert_eq!(asst.role, "assistant");
    }

    #[test]
    fn test_openai_provider_creation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::OpenAI,
            api_key: "test-key".to_string(),
            api_url: Some("https://api.openai.com/v1".to_string()),
            model: "gpt-4".to_string(),
            max_tokens: Some(4096),
            temperature: Some(0.7),
        };

        let provider = OpenAIProvider::new(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_analysis_prompt_generation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::OpenAI,
            api_key: "test-key".to_string(),
            api_url: None,
            model: "gpt-4".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let provider = OpenAIProvider::new(config).unwrap();

        let input = AnalysisInput {
            logs: serde_json::json!({"test": "data"}),
            start_time: Utc::now(),
            end_time: Utc::now(),
            project_context: Some("Test project".to_string()),
            screenshots: None,
        };

        let prompt = provider.build_analysis_prompt(&input);
        assert!(prompt.contains("expert code analyst"));
        assert!(prompt.contains("Test project"));
        assert!(prompt.contains("test"));
    }

    #[test]
    fn test_script_prompt_generation() {
        let config = AIConfig {
            provider: crate::ai::AIProviderType::OpenAI,
            api_key: "test-key".to_string(),
            api_url: None,
            model: "gpt-4".to_string(),
            max_tokens: None,
            temperature: None,
        };

        let provider = OpenAIProvider::new(config).unwrap();

        let analysis = SessionAnalysis::new(
            Utc::now(),
            Utc::now(),
            "Test summary".to_string(),
            "Test Session".to_string(),
        );

        let input = ScriptInput {
            analysis,
            style: Some("comedy".to_string()),
            audience: Some("developers".to_string()),
        };

        let prompt = provider.build_script_prompt(&input);
        assert!(prompt.contains("expert scriptwriter"));
        assert!(prompt.contains("comedy"));
        assert!(prompt.contains("developers"));
    }
}
