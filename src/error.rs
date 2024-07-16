use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("APIError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Unknown: {0}")]
    Unknown(String),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("AsyncError: {0}")]
    AsyncError(#[from] async_std::io::Error),
    #[error("HeaderError: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
