//! This module defines the structures and methods for handling edit requests and responses.
//! It includes:
//! - `EditRequest`: Struct for creating a request to edit text.
//! - `EditChoice`: Struct representing a single edit choice from the response.
//! - `EditResponse`: Struct for the response from an edit request.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::common;
use crate::impl_builder_methods;

/// Represents a request to edit text.
#[derive(Debug, Serialize, Clone)]
pub struct EditRequest {
    /// Model to be used for editing.
    pub model: String,
    /// Optional input text to be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// Instruction for editing the text.
    pub instruction: String,
    /// Optional number of edits to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Optional temperature setting for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Optional top-p setting for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

impl EditRequest {
    /// Creates a new `EditRequest` with the specified model and instruction.
    pub fn new(model: String, instruction: String) -> Self {
        Self {
            model,
            instruction,
            input: None,
            n: None,
            temperature: None,
            top_p: None,
        }
    }
}

impl_builder_methods!(
    EditRequest,
    input: String,
    n: i32,
    temperature: f32,
    top_p: f32
);

/// Represents a single edit choice from the response.
#[derive(Debug, Deserialize, Serialize)]
pub struct EditChoice {
    /// Edited text.
    pub text: String,
    /// Index of the edit choice.
    pub index: i32,
}

/// Represents the response from an edit request.
#[derive(Debug, Deserialize, Serialize)]
pub struct EditResponse {
    /// Object type, typically "edit".
    pub object: String,
    /// Timestamp of when the edit response was created.
    pub created: i64,
    /// Usage information for the edit request.
    pub usage: common::Usage,
    /// List of edit choices.
    pub choices: Vec<EditChoice>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
