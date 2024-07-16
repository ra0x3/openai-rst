//! This module defines custom error types for handling API-related errors.
//! The `APIError` enum provides variants for different kinds of errors that may occur
//! when interacting with APIs, including network errors, serialization errors, and more.

use thiserror::Error;

/// Enum representing different kinds of API-related errors.
#[derive(Debug, Error)]
pub enum APIError {
    /// Error variant for network-related errors, originating from the `reqwest` library.
    #[error("APIError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error variant for unknown errors with a descriptive message.
    #[error("Unknown: {0}")]
    Unknown(String),

    /// Error variant for serialization/deserialization errors, originating from the `serde_json` library.
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Error variant for asynchronous I/O errors, originating from the `async_std` library.
    #[error("AsyncError: {0}")]
    AsyncError(#[from] async_std::io::Error),

    /// Error variant for invalid header values, originating from the `reqwest` library.
    #[error("HeaderError: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
