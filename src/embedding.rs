//! This module defines the structures and methods for handling text embeddings.
//! It includes:
//! - `EmbeddingData`: Struct representing the data of an embedding.
//! - `EmbeddingRequest`: Struct for creating a request to generate embeddings.
//! - `EmbeddingResponse`: Struct for the response from an embedding request.
//! - `Usage`: Struct for tracking token usage in embedding operations.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use crate::{impl_builder_methods, models::Model};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, option::Option};

/// Represents the data of an embedding.
#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddingData {
    /// Object type, typically "embedding".
    pub object: String,
    /// Vector representing the embedding.
    pub embedding: Vec<f32>,
    /// Index of the embedding.
    pub index: i32,
}

/// Represents a request to generate embeddings.
#[derive(Debug, Serialize, Clone)]
pub struct EmbeddingRequest {
    /// Model to be used for generating embeddings.
    pub model: Model,
    /// Input text for which embeddings are to be generated.
    pub input: String,
    /// Optional dimensions of the embedding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EmbeddingRequest {
    /// Creates a new `EmbeddingRequest` with the specified model and input text.
    pub fn new(model: Model, input: String) -> Self {
        Self {
            model,
            input,
            dimensions: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    EmbeddingRequest,
    user: String
);

/// Represents the response from an embedding request.
#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    /// Object type, typically "list".
    pub object: String,
    /// List of embedding data.
    pub data: Vec<EmbeddingData>,
    /// Model used for generating embeddings.
    pub model: Model,
    /// Usage information for the embedding request.
    pub usage: Usage,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents token usage in embedding operations.
#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    /// Number of tokens used in the prompt.
    pub prompt_tokens: i32,
    /// Total number of tokens used.
    pub total_tokens: i32,
}
