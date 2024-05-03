use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("APIError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("GenericError: {0}")]
    GenericError(String),
}
