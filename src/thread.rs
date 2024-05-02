use crate::common::MessageRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateThreadRequest {
    pub fn new() -> Self {
        Self {
            messages: None,
            metadata: None,
        }
    }
}

impl Default for CreateThreadRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
  CreateThreadRequest,
  messages: Vec<Message>,
  metadata: HashMap<String, String>
);

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub metadata: HashMap<String, String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct ModifyThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyThreadRequest {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl_builder_methods!(
    ModifyThreadRequest,
    metadata: HashMap<String, String>
);
