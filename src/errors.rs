use std::collections::HashMap;
use thiserror::Error;

/// The primary error type for the WazobiaMail SDK.
#[derive(Debug, Error)]
pub enum WazobiaError {
    /// Authentication failure: missing, invalid, or expired API key or Bearer token (HTTP 401).
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Permission denied or plan limitation reached (HTTP 403).
    #[error("Permission denied: {message}")]
    Forbidden {
        message: String,
        upgrade_required: bool,
    },

    /// The requested resource does not exist (HTTP 404).
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error on input fields (HTTP 422).
    #[error("Validation failed: {message}")]
    Validation {
        message: String,
        errors: HashMap<String, String>,
    },

    /// Rate limit reached (HTTP 429).
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        retry_after: Option<u64>,
    },

    /// Resource state conflict (HTTP 409).
    #[error("Resource conflict: {0}")]
    Conflict(String),

    /// Remote server error (HTTP 5xx).
    #[error("Server error ({status}): {message}")]
    Server { status: u16, message: String },

    /// General API error with non-2xx status code.
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<String>,
    },

    /// Configuration error (e.g. invalid base URL, missing credentials).
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// HTTP network or transport failure.
    #[error("Network error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization or deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// I/O error (e.g. reading attachment file).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience result type for WazobiaMail operations.
pub type Result<T> = std::result::Result<T, WazobiaError>;
