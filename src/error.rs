//! Error types for the video URL validator

use std::fmt;

/// Errors that can occur during URL validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The URL format is invalid
    InvalidFormat,
    /// The platform is not supported
    UnsupportedPlatform,
    /// The URL is empty or contains invalid characters
    MalformedUrl,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidFormat => write!(f, "Invalid URL format"),
            ValidationError::UnsupportedPlatform => write!(f, "Unsupported video platform"),
            ValidationError::MalformedUrl => write!(f, "Malformed URL"),
        }
    }
}

impl std::error::Error for ValidationError {}
