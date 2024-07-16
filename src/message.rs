//! This module defines the structures and methods for creating, modifying, and managing messages and their related content.
//! It includes:
//! - `CreateMessageRequest`: Struct for creating a new message with optional file IDs and metadata.
//! - `ModifyMessageRequest`: Struct for modifying an existing message's metadata.
//! - `MessageObject`: Struct representing a message object with various attributes.
//! - `Content`: Struct for the content of a message.
//! - `ContentText`: Struct for text content within a message, including annotations.
//! - `ListMessage`: Struct for listing multiple messages.
//! - `MessageFileObject`: Struct representing a file object associated with a message.
//! - `ListMessageFile`: Struct for listing multiple message file objects.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use crate::common::MessageRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

/// Represents a request to create a new message.
#[derive(Debug, Serialize, Clone)]
pub struct CreateMessageRequest {
    /// Role of the message sender.
    pub role: MessageRole,
    /// Content of the message.
    pub content: String,
    /// Optional file IDs associated with the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Optional metadata for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateMessageRequest {
    /// Creates a new `CreateMessageRequest` with the specified role and content.
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role,
            content,
            file_ids: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    CreateMessageRequest,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>
);

/// Represents a request to modify an existing message's metadata.
#[derive(Debug, Serialize, Clone)]
pub struct ModifyMessageRequest {
    /// Optional metadata to update in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyMessageRequest {
    /// Creates a new `ModifyMessageRequest`.
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyMessageRequest {
    /// Provides a default implementation for `ModifyMessageRequest`.
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyMessageRequest,
    metadata: HashMap<String, String>
);

/// Represents a message object with various attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageObject {
    /// Unique identifier for the message.
    pub id: String,
    /// Object type, typically "message".
    pub object: String,
    /// Timestamp of when the message was created.
    pub created_at: i64,
    /// Identifier for the associated thread.
    pub thread_id: String,
    /// Role of the message sender.
    pub role: MessageRole,
    /// Content of the message.
    pub content: Vec<Content>,
    /// Optional identifier for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// Optional identifier for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// File IDs associated with the message.
    pub file_ids: Vec<String>,
    /// Metadata for the message.
    pub metadata: HashMap<String, String>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents the content of a message.
#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    /// Type of the content.
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content of the message.
    pub text: ContentText,
}

/// Represents text content within a message, including annotations.
#[derive(Debug, Deserialize, Serialize)]
pub struct ContentText {
    /// Text value of the content.
    pub value: String,
    /// Annotations for the text content.
    pub annotations: Vec<String>,
}

/// Represents a list of messages.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessage {
    /// Object type, typically "list".
    pub object: String,
    /// List of message objects.
    pub data: Vec<MessageObject>,
    /// Identifier for the first message in the list.
    pub first_id: String,
    /// Identifier for the last message in the list.
    pub last_id: String,
    /// Indicates if there are more messages available.
    pub has_more: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a file object associated with a message.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageFileObject {
    /// Unique identifier for the file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Identifier for the associated message.
    pub message_id: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a list of message file objects.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessageFile {
    /// Object type, typically "list".
    pub object: String,
    /// List of message file objects.
    pub data: Vec<MessageFileObject>,
    /// Identifier for the first file in the list.
    pub first_id: String,
    /// Identifier for the last file in the list.
    pub last_id: String,
    /// Indicates if there are more files available.
    pub has_more: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
