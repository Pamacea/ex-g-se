// AI Module - Provider Interface and Implementations
//
// This module provides AI provider support for analyzing development sessions
// and generating scripts. It supports multiple AI providers including OpenAI,
// Anthropic, ZAI, and custom HTTP endpoints.

mod anthropic;
mod error;
mod openai;
mod types;

pub use error::{AIError, AIResult};
pub use types::{
    Act, AnalysisInput, Dialogue, Intent, KeyMoment, Scene, Script, ScriptInput, SessionAnalysis,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ============================================================================
// AI Provider Types
// ============================================================================

/// Available AI provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AIProviderType {
    /// OpenAI (GPT-4, GPT-4o, etc.)
    OpenAI,
    /// Anthropic (Claude Opus, Sonnet, etc.)
    Anthropic,
    /// ZAI provider
    ZAI,
    /// Custom HTTP endpoint
    Custom,
}

impl std::fmt::Display for AIProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIProviderType::OpenAI => write!(f, "openai"),
            AIProviderType::Anthropic => write!(f, "anthropic"),
            AIProviderType::ZAI => write!(f, "zai"),
            AIProviderType::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for AIProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AIProviderType::OpenAI),
            "anthropic" => Ok(AIProviderType::Anthropic),
            "zai" => Ok(AIProviderType::ZAI),
            "custom" => Ok(AIProviderType::Custom),
            _ => Err(format!("Unknown AI provider: {}", s)),
        }
    }
}

/// Default model for each provider type
impl AIProviderType {
    /// Get the default model for this provider
    pub fn default_model(&self) -> &str {
        match self {
            AIProviderType::OpenAI => "gpt-4o",
            AIProviderType::Anthropic => "claude-3-5-sonnet-20241022",
            AIProviderType::ZAI => "zai-v1",
            AIProviderType::Custom => "custom-model",
        }
    }

    /// Get the default API URL for this provider
    pub fn default_url(&self) -> &str {
        match self {
            AIProviderType::OpenAI => "https://api.openai.com/v1",
            AIProviderType::Anthropic => "https://api.anthropic.com/v1",
            AIProviderType::ZAI => "https://api.zai.ai/v1",
            AIProviderType::Custom => "https://custom.example.com/v1",
        }
    }
}

// ============================================================================
// AI Configuration
// ============================================================================

/// Configuration for AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Provider type
    pub provider: AIProviderType,

    /// API key for authentication
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api_key: String,

    /// Optional custom API URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,

    /// Model identifier
    pub model: String,

    /// Maximum tokens for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Temperature for generation (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl AIConfig {
    /// Create a new AI configuration
    pub fn new(provider: AIProviderType, api_key: impl Into<String>) -> Self {
        let provider_type = provider;
        Self {
            provider: provider_type,
            api_key: api_key.into(),
            api_url: None,
            model: provider_type.default_model().to_string(),
            max_tokens: Some(4096),
            temperature: Some(0.7),
        }
    }

    /// Set a custom API URL
    pub fn with_api_url(mut self, url: impl Into<String>) -> Self {
        self.api_url = Some(url.into());
        self
    }

    /// Set the model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> AIResult<()> {
        if self.api_key.is_empty() {
            return Err(AIError::missing_parameter("api_key"));
        }

        if self.model.is_empty() {
            return Err(AIError::missing_parameter("model"));
        }

        if let Some(temp) = self.temperature {
            if !(0.0..=1.0).contains(&temp) {
                return Err(AIError::config_error(format!(
                    "temperature must be between 0.0 and 1.0, got {}",
                    temp
                )));
            }
        }

        Ok(())
    }

    /// Create configuration from environment variables
    ///
    /// Environment variables:
    /// - EX_AI_PROVIDER: Provider type (openai, anthropic, zai, custom)
    /// - EX_AI_API_KEY: API key
    /// - EX_AI_API_URL: Optional custom API URL
    /// - EX_AI_MODEL: Optional model name
    /// - EX_AI_MAX_TOKENS: Optional max tokens
    /// - EX_AI_TEMPERATURE: Optional temperature
    pub fn from_env() -> AIResult<Self> {
        let provider_str = std::env::var("EX_AI_PROVIDER")
            .unwrap_or_else(|_| "openai".to_string());
        let provider = provider_str.parse().map_err(|e| AIError::config_error(e))?;

        let api_key = std::env::var("EX_AI_API_KEY")
            .map_err(|_| AIError::missing_parameter("EX_AI_API_KEY environment variable"))?;

        let mut config = Self::new(provider, api_key);

        if let Ok(url) = std::env::var("EX_AI_API_URL") {
            config = config.with_api_url(url);
        }

        if let Ok(model) = std::env::var("EX_AI_MODEL") {
            config = config.with_model(model);
        }

        if let Ok(tokens) = std::env::var("EX_AI_MAX_TOKENS") {
            let tokens_val = tokens.parse().map_err(|_| {
                AIError::config_error("EX_AI_MAX_TOKENS must be a valid u32")
            })?;
            config = config.with_max_tokens(tokens_val);
        }

        if let Ok(temp) = std::env::var("EX_AI_TEMPERATURE") {
            let temp_val: f32 = temp.parse().map_err(|_| {
                AIError::config_error("EX_AI_TEMPERATURE must be a valid f32")
            })?;
            config = config.with_temperature(temp_val);
        }

        config.validate()?;
        Ok(config)
    }
}

// ============================================================================
// AI Provider Trait
// ============================================================================

/// Trait defining the interface for AI providers
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Analyze a development session
    ///
    /// This method takes session logs and metadata, then uses AI to:
    /// - Identify the primary intents
    /// - Extract key moments
    /// - Detect patterns
    /// - Generate a summary
    async fn analyze_session(&self, input: &AnalysisInput) -> AIResult<SessionAnalysis>;

    /// Generate a script from session analysis
    ///
    /// This method takes the results of session analysis and generates
    /// a scripted narrative with acts, scenes, and dialogue.
    async fn generate_script(&self, input: &ScriptInput) -> AIResult<Script>;
}

// ============================================================================
// Provider Factory
// ============================================================================

/// Factory for creating AI provider instances
pub struct AIProviderFactory;

impl AIProviderFactory {
    /// Create an AI provider instance from configuration
    ///
    /// This returns a boxed trait object, allowing for dynamic provider selection
    pub fn create_provider(config: AIConfig) -> AIResult<Box<dyn AIProvider>> {
        config.validate()?;

        match config.provider {
            AIProviderType::OpenAI => Ok(Box::new(openai::OpenAIProvider::new(config)?)),
            AIProviderType::Anthropic => Ok(Box::new(anthropic::AnthropicProvider::new(config)?)),
            AIProviderType::ZAI => Err(AIError::unsupported(
                "ZAI provider not yet implemented",
            )),
            AIProviderType::Custom => Err(AIError::unsupported(
                "Custom provider not yet implemented",
            )),
        }
    }

    /// Create a provider from environment variables
    pub fn create_from_env() -> AIResult<Box<dyn AIProvider>> {
        let config = AIConfig::from_env()?;
        Self::create_provider(config)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a default OpenAI provider
pub fn create_openai_provider(api_key: impl Into<String>) -> AIResult<Box<dyn AIProvider>> {
    let config = AIConfig::new(AIProviderType::OpenAI, api_key);
    AIProviderFactory::create_provider(config)
}

/// Create a default Anthropic provider
pub fn create_anthropic_provider(api_key: impl Into<String>) -> AIResult<Box<dyn AIProvider>> {
    let config = AIConfig::new(AIProviderType::Anthropic, api_key);
    AIProviderFactory::create_provider(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_display() {
        assert_eq!(AIProviderType::OpenAI.to_string(), "openai");
        assert_eq!(AIProviderType::Anthropic.to_string(), "anthropic");
        assert_eq!(AIProviderType::ZAI.to_string(), "zai");
        assert_eq!(AIProviderType::Custom.to_string(), "custom");
    }

    #[test]
    fn test_provider_type_from_str() {
        assert_eq!("openai".parse::<AIProviderType>().unwrap(), AIProviderType::OpenAI);
        assert_eq!("anthropic".parse::<AIProviderType>().unwrap(), AIProviderType::Anthropic);
        assert_eq!("OPENAI".parse::<AIProviderType>().unwrap(), AIProviderType::OpenAI);
        assert!("unknown".parse::<AIProviderType>().is_err());
    }

    #[test]
    fn test_provider_defaults() {
        assert_eq!(AIProviderType::OpenAI.default_model(), "gpt-4o");
        assert_eq!(
            AIProviderType::Anthropic.default_model(),
            "claude-3-5-sonnet-20241022"
        );
        assert_eq!(
            AIProviderType::OpenAI.default_url(),
            "https://api.openai.com/v1"
        );
    }

    #[test]
    fn test_config_builder() {
        let config = AIConfig::new(AIProviderType::OpenAI, "test-key")
            .with_model("gpt-4")
            .with_api_url("https://custom.example.com/v1")
            .with_max_tokens(2048)
            .with_temperature(0.5);

        assert_eq!(config.provider, AIProviderType::OpenAI);
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.api_url, Some("https://custom.example.com/v1".to_string()));
        assert_eq!(config.max_tokens, Some(2048));
        assert_eq!(config.temperature, Some(0.5));
    }

    #[test]
    fn test_config_validation() {
        // Valid config
        let config = AIConfig::new(AIProviderType::OpenAI, "test-key");
        assert!(config.validate().is_ok());

        // Missing API key
        let config = AIConfig {
            provider: AIProviderType::OpenAI,
            api_key: "".to_string(),
            api_url: None,
            model: "gpt-4".to_string(),
            max_tokens: None,
            temperature: None,
        };
        assert!(config.validate().is_err());

        // Invalid temperature
        let config = AIConfig {
            provider: AIProviderType::OpenAI,
            api_key: "test-key".to_string(),
            api_url: None,
            model: "gpt-4".to_string(),
            max_tokens: None,
            temperature: Some(1.5),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_provider_factory_openai() {
        let config = AIConfig::new(AIProviderType::OpenAI, "test-key");
        let provider = AIProviderFactory::create_provider(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_provider_factory_anthropic() {
        let config = AIConfig::new(AIProviderType::Anthropic, "test-key");
        let provider = AIProviderFactory::create_provider(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_provider_factory_unimplemented() {
        let config = AIConfig::new(AIProviderType::ZAI, "test-key");
        let result = AIProviderFactory::create_provider(config);
        assert!(result.is_err());
        assert!(matches!(result, Err(AIError::UnsupportedOperation(_))));

        let config = AIConfig::new(AIProviderType::Custom, "test-key");
        let result = AIProviderFactory::create_provider(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_helper_functions() {
        let provider = create_openai_provider("test-key");
        assert!(provider.is_ok());

        let provider = create_anthropic_provider("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = AIConfig::new(AIProviderType::OpenAI, "sk-test123");
        let json = serde_json::to_string(&config).unwrap();
        let parsed: AIConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.provider, AIProviderType::OpenAI);
        assert_eq!(parsed.api_key, "sk-test123");
        assert_eq!(parsed.model, "gpt-4o");
    }
}
