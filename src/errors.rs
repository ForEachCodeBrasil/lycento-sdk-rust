//! Error types for the Lycento SDK.
//!
//! This module provides custom error types that map to the JavaScript SDK errors:
//! - `LycentoError` - Base error type
//! - `ActivationError` - License activation failures
//! - `ValidationError` - License validation failures
//! - `DeactivationError` - License deactivation failures
//! - `NetworkError` - Network connectivity issues

use thiserror::Error;

/// Base error type for all Lycento SDK errors.
#[derive(Debug, Error)]
pub enum LycentoError {
    #[error("LycentoError: {0}")]
    Custom(String),
}

impl LycentoError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Custom(msg) => msg,
        }
    }
}

impl From<reqwest::Error> for LycentoError {
    fn from(err: reqwest::Error) -> Self {
        LycentoError::Custom(err.to_string())
    }
}

impl From<serde_json::Error> for LycentoError {
    fn from(err: serde_json::Error) -> Self {
        LycentoError::Custom(err.to_string())
    }
}

/// Error type for license activation failures.
#[derive(Debug, Error)]
pub enum ActivationError {
    #[error("ActivationError: {0}")]
    Custom(String),
}

impl ActivationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Custom(msg) => msg,
        }
    }
}

impl From<LycentoError> for ActivationError {
    fn from(err: LycentoError) -> Self {
        ActivationError::Custom(err.message().to_string())
    }
}

/// Error type for license validation failures.
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("ValidationError: {0}")]
    Custom(String),
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Custom(msg) => msg,
        }
    }
}

impl From<LycentoError> for ValidationError {
    fn from(err: LycentoError) -> Self {
        ValidationError::Custom(err.message().to_string())
    }
}

/// Error type for license deactivation failures.
#[derive(Debug, Error)]
pub enum DeactivationError {
    #[error("DeactivationError: {0}")]
    Custom(String),
}

impl DeactivationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Custom(msg) => msg,
        }
    }
}

impl From<LycentoError> for DeactivationError {
    fn from(err: LycentoError) -> Self {
        DeactivationError::Custom(err.message().to_string())
    }
}

/// Error type for network-related failures.
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("NetworkError: {0}")]
    Custom(String),
}

impl NetworkError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Custom(msg) => msg,
        }
    }
}

impl From<reqwest::Error> for NetworkError {
    fn from(err: reqwest::Error) -> Self {
        NetworkError::Custom(err.to_string())
    }
}

/// Result type alias for SDK operations.
pub type Result<T, E = LycentoError> = std::result::Result<T, E>;

/// Specialized result type for activation operations.
pub type ActivationResult<T> = std::result::Result<T, ActivationError>;

/// Specialized result type for validation operations.
pub type ValidationResult<T> = std::result::Result<T, ValidationError>;

/// Specialized result type for deactivation operations.
pub type DeactivationResult<T> = std::result::Result<T, DeactivationError>;

/// Specialized result type for network operations.
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;
