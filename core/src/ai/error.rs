// AI Module - Error Types
//
// This module defines error types for AI provider operations.

use thiserror::Error;

// ============================================================================
// AI Error Types
// ============================================================================

/// Main error type for AI provider operations
#[derive(Error, Debug)]
pub enum AIError {
    /// Error during HTTP request
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Error parsing API response
    #[error("Failed to parse API response: {0}")]
    ParseError(String),

    /// API returned an error
    #[error("API error (status {status}): {message}")]
    ApiError {
        status: u16,
        message: String,
    },

    /// Authentication error
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// Missing required parameter
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Timeout error
    #[error("Request timed out after {0}ms")]
    Timeout(u64),

    /// Unsupported operation for this provider
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// Error encoding/decoding JSON
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Provider-specific error
    #[error("Provider error: {0}")]
    ProviderError(String),
}

/// Result type alias for AI operations
pub type AIResult<T> = Result<T, AIError>;

impl AIError {
    /// Create an API error with status code and message
    pub fn api_error(status: u16, message: impl Into<String>) -> Self {
        AIError::ApiError {
            status,
            message: message.into(),
        }
    }

    /// Create a parse error
    pub fn parse_error(msg: impl Into<String>) -> Self {
        AIError::ParseError(msg.into())
    }

    /// Create an authentication error
    pub fn auth_error(msg: impl Into<String>) -> Self {
        AIError::AuthError(msg.into())
    }

    /// Create a rate limit error
    pub fn rate_limit_error(msg: impl Into<String>) -> Self {
        AIError::RateLimitError(msg.into())
    }

    /// Create a configuration error
    pub fn config_error(msg: impl Into<String>) -> Self {
        AIError::ConfigError(msg.into())
    }

    /// Create a missing parameter error
    pub fn missing_parameter(param: impl Into<String>) -> Self {
        AIError::MissingParameter(param.into())
    }

    /// Create a timeout error
    pub fn timeout(ms: u64) -> Self {
        AIError::Timeout(ms)
    }

    /// Create an unsupported operation error
    pub fn unsupported(op: impl Into<String>) -> Self {
        AIError::UnsupportedOperation(op.into())
    }

    /// Create a provider-specific error
    pub fn provider_error(msg: impl Into<String>) -> Self {
        AIError::ProviderError(msg.into())
    }

    /// Check if this is a recoverable error (e.g., rate limit, timeout)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AIError::RateLimitError(_)
                | AIError::Timeout(_)
                | AIError::HttpError(_)
        )
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, AIError::AuthError(_))
    }

    /// Check if this is a configuration error
    pub fn is_config_error(&self) -> bool {
        matches!(self, AIError::ConfigError(_) | AIError::MissingParameter(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AIError::api_error(500, "Internal server error");
        assert!(err.to_string().contains("500"));
        assert!(err.to_string().contains("Internal server error"));
    }

    #[test]
    fn test_error_recoverable() {
        assert!(AIError::rate_limit_error("Too many requests").is_recoverable());
        assert!(AIError::timeout(30000).is_recoverable());
        assert!(!AIError::auth_error("Invalid key").is_recoverable());
        assert!(!AIError::config_error("Missing URL").is_recoverable());
    }

    #[test]
    fn test_error_classification() {
        assert!(AIError::auth_error("Bad token").is_auth_error());
        assert!(AIError::config_error("No model").is_config_error());
        assert!(AIError::missing_parameter("api_key").is_config_error());
    }
}
