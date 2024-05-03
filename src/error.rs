use async_std::io::Error as AsyncError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("APIError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("GenericError: {0}")]
    GenericError(String),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] SerdeError),
    #[error("AsyncError: {0}")]
    AsyncError(#[from] AsyncError),
}
