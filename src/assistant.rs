//! This module defines the structures and methods for handling assistant-related requests and responses.
//! It includes the `AssistantRequest`, `AssistantObject`, `DeletionStatus`, `ListAssistant`, `AssistantFileRequest`,
//! `AssistantFileObject`, and `ListAssistantFile` structs along with their associated methods.

use crate::{impl_builder_methods, models::Model};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a request to create or update an assistant.
#[derive(Debug, Serialize, Clone)]
pub struct AssistantRequest {
    /// Model to be used for the assistant.
    pub model: Model,
    /// Optional name of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional description of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional instructions for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Optional tools to be used by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    /// Optional file IDs associated with the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Optional metadata for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl AssistantRequest {
    /// Creates a new `AssistantRequest` with the specified model.
    pub fn new(model: Model) -> Self {
        Self {
            model,
            name: None,
            description: None,
            instructions: None,
            tools: None,
            file_ids: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    AssistantRequest,
    name: String,
    description: String,
    instructions: String,
    tools: Vec<HashMap<String, String>>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>
);

/// Represents an assistant object with its properties.
#[derive(Debug, Deserialize, Serialize)]
pub struct AssistantObject {
    /// Unique identifier for the assistant.
    pub id: String,
    /// Object type, typically "assistant".
    pub object: String,
    /// Timestamp of when the assistant was created.
    pub created_at: i64,
    /// Optional name of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional description of the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Model used by the assistant.
    pub model: Model,
    /// Optional instructions for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Tools associated with the assistant.
    pub tools: Vec<HashMap<String, String>>,
    /// File IDs associated with the assistant.
    pub file_ids: Vec<String>,
    /// Metadata for the assistant.
    pub metadata: HashMap<String, String>,
    /// Optional headers associated with the assistant.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents the status of an assistant deletion request.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletionStatus {
    /// Unique identifier for the assistant.
    pub id: String,
    /// Object type, typically "assistant".
    pub object: String,
    /// Indicates whether the assistant was deleted.
    pub deleted: bool,
    /// Optional headers associated with the deletion status.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a list of assistants.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistant {
    /// Object type, typically "list".
    pub object: String,
    /// List of assistant objects.
    pub data: Vec<AssistantObject>,
    /// Optional headers associated with the list of assistants.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to get an assistant file by its ID.
#[derive(Debug, Serialize, Clone)]
pub struct AssistantFileRequest {
    /// Unique identifier for the file.
    pub file_id: String,
}

/// Represents an assistant file object with its properties.
#[derive(Debug, Deserialize, Serialize)]
pub struct AssistantFileObject {
    /// Unique identifier for the file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Unique identifier for the assistant associated with the file.
    pub assistant_id: String,
    /// Optional headers associated with the file.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a list of assistant files.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistantFile {
    /// Object type, typically "list".
    pub object: String,
    /// List of assistant file objects.
    pub data: Vec<AssistantFileObject>,
    /// Optional headers associated with the list of assistant files.
    pub headers: Option<HashMap<String, String>>,
}
