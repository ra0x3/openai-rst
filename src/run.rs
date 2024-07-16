//! This module defines the structures and methods for creating and modifying runs,
//! as well as handling run-related requests and responses.
//! It includes:
//! - `CreateRunRequest`: Struct for creating a new run with optional parameters.
//! - `ModifyRunRequest`: Struct for modifying an existing run's metadata.
//! - `RunObject`: Struct representing a run object with various attributes.
//! - `ListRun`: Struct for listing multiple runs.
//! - `CreateThreadAndRunRequest`: Struct for creating a thread and a run simultaneously.
//! - `RunStepObject`: Struct representing a step within a run.
//! - `ListRunStep`: Struct for listing multiple run steps.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use super::thread::CreateThreadRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

/// Represents a request to create a new run.
#[derive(Debug, Serialize, Clone)]
pub struct CreateRunRequest {
    /// Identifier for the assistant.
    assistant_id: String,
    /// Optional model to be used for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional instructions for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Optional tools to be used during the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    /// Optional metadata for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateRunRequest {
    /// Creates a new `CreateRunRequest` with the specified assistant ID.
    pub fn new(assistant_id: String) -> Self {
        Self {
            assistant_id,
            model: None,
            instructions: None,
            tools: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    CreateRunRequest,
    model: String,
    instructions: String,
    tools: Vec<HashMap<String, String>>,
    metadata: HashMap<String, String>
);

/// Represents a request to modify an existing run's metadata.
#[derive(Debug, Serialize, Clone)]
pub struct ModifyRunRequest {
    /// Optional metadata to update in the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyRunRequest {
    /// Creates a new `ModifyRunRequest`.
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyRunRequest {
    /// Provides a default implementation for `ModifyRunRequest`.
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyRunRequest,
    metadata: HashMap<String, String>
);

/// Represents a run object with various attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct RunObject {
    /// Unique identifier for the run.
    pub id: String,
    /// Object type, typically "run".
    pub object: String,
    /// Timestamp of when the run was created.
    pub created_at: i64,
    /// Identifier for the associated thread.
    pub thread_id: String,
    /// Identifier for the assistant.
    pub assistant_id: String,
    /// Status of the run.
    pub status: String,
    /// Optional required actions for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_action: Option<HashMap<String, String>>,
    /// Optional last error encountered during the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// Optional expiration timestamp of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Optional start timestamp of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// Optional cancellation timestamp of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    /// Optional failure timestamp of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    /// Optional completion timestamp of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// Model used in the run.
    pub model: String,
    /// Optional instructions for the run.
    pub instructions: Option<String>,
    /// Tools used during the run.
    pub tools: Vec<HashMap<String, String>>,
    /// File IDs associated with the run.
    pub file_ids: Vec<String>,
    /// Metadata for the run.
    pub metadata: HashMap<String, String>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a list of runs.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListRun {
    /// Object type, typically "list".
    pub object: String,
    /// List of run objects.
    pub data: Vec<RunObject>,
    /// Identifier for the first run in the list.
    pub first_id: String,
    /// Identifier for the last run in the list.
    pub last_id: String,
    /// Indicates if there are more runs available.
    pub has_more: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to create a thread and a run simultaneously.
#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadAndRunRequest {
    /// Identifier for the assistant.
    pub assistant_id: String,
    /// Optional request to create a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<CreateThreadRequest>,
    /// Optional model to be used for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional instructions for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Optional tools to be used during the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    /// Optional metadata for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Represents a step within a run.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RunStepObject {
    /// Unique identifier for the run step.
    pub id: String,
    /// Object type, typically "run_step".
    pub object: String,
    /// Timestamp of when the run step was created.
    pub created_at: i64,
    /// Identifier for the assistant.
    pub assistant_id: String,
    /// Identifier for the associated thread.
    pub thread_id: String,
    /// Identifier for the run.
    pub run_id: String,
    /// Type of the run step.
    #[serde(rename = "type")]
    pub run_step_type: String,
    /// Status of the run step.
    pub status: String,
    /// Details about the run step.
    pub step_details: HashMap<String, String>,
    /// Optional last error encountered during the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// Optional expiration timestamp of the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// Optional start timestamp of the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// Optional cancellation timestamp of the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    /// Optional failure timestamp of the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    /// Optional completion timestamp of the run step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// Metadata for the run step.
    pub metadata: HashMap<String, String>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a list of run steps.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRunStep {
    /// Object type, typically "list".
    pub object: String,
    /// List of run step objects.
    pub data: Vec<RunStepObject>,
    /// Identifier for the first run step in the list.
    pub first_id: String,
    /// Identifier for the last run step in the list.
    pub last_id: String,
    /// Indicates if there are more run steps available.
    pub has_more: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
