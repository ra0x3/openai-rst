//! This module defines the structures and methods for creating and modifying threads, as well as the message format used within threads.
//! It includes:
//! - `CreateThreadRequest`: Struct for creating a new thread with optional messages and metadata.
//! - `ThreadObject`: Struct representing a thread object with various attributes.
//! - `Message`: Struct for messages within a thread, including role, content, and optional metadata.
//! - `ModifyThreadRequest`: Struct for modifying an existing thread's metadata.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use crate::common::MessageRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

/// Represents a request to create a new thread.
#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadRequest {
    /// Optional list of messages in the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    /// Optional metadata for the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateThreadRequest {
    /// Creates a new `CreateThreadRequest`.
    pub fn new() -> Self {
        Self {
            messages: None,
            metadata: None,
        }
    }
}

impl Default for CreateThreadRequest {
    /// Provides a default implementation for `CreateThreadRequest`.
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
  CreateThreadRequest,
  messages: Vec<Message>,
  metadata: HashMap<String, String>
);

/// Represents a thread object with various attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadObject {
    /// Unique identifier for the thread.
    pub id: String,
    /// Object type, typically "thread".
    pub object: String,
    /// Timestamp of when the thread was created.
    pub created_at: i64,
    /// Metadata associated with the thread.
    pub metadata: HashMap<String, String>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a message within a thread.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    /// Role of the message sender.
    pub role: MessageRole,
    /// Content of the message.
    pub content: String,
    /// Optional file IDs associated with the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<String>,
    /// Optional metadata for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Represents a request to modify an existing thread's metadata.
#[derive(Default, Debug, Serialize, Clone)]
pub struct ModifyThreadRequest {
    /// Optional metadata to update in the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyThreadRequest {
    /// Creates a new `ModifyThreadRequest`.
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl_builder_methods!(
    ModifyThreadRequest,
    metadata: HashMap<String, String>
);
