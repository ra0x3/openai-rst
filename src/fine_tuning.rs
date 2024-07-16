//! This module defines the structures and methods for managing fine-tuning jobs.
//! It includes:
//! - `CreateFineTuningJobRequest`: Struct for creating a new fine-tuning job.
//! - `ListFineTuningJobsRequest`: Struct for listing fine-tuning jobs.
//! - `ListFineTuningJobEventsRequest`: Struct for listing events of a fine-tuning job.
//! - `RetrieveFineTuningJobRequest`: Struct for retrieving a specific fine-tuning job.
//! - `CancelFineTuningJobRequest`: Struct for canceling a fine-tuning job.
//! - `FineTuningPagination`: Struct for handling pagination in fine-tuning job responses.
//! - `FineTuningJobObject`: Struct representing a fine-tuning job object with various attributes.
//! - `FineTuningJobError`: Struct for handling errors related to fine-tuning jobs.
//! - `FineTuningJobEvent`: Struct for events associated with fine-tuning jobs.
//! - `HyperParameters`: Struct for specifying hyperparameters in fine-tuning jobs.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

/// Represents a request to create a new fine-tuning job.
#[derive(Debug, Serialize, Clone)]
pub struct CreateFineTuningJobRequest {
    /// Model to be fine-tuned.
    pub model: String,
    /// File containing the training data.
    pub training_file: String,
    /// Optional hyperparameters for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<HyperParameters>,
    /// Optional suffix for the fine-tuned model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Optional file containing validation data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

impl CreateFineTuningJobRequest {
    /// Creates a new `CreateFineTuningJobRequest` with the specified model and training file.
    pub fn new(model: String, training_file: String) -> Self {
        Self {
            model,
            training_file,
            hyperparameters: None,
            suffix: None,
            validation_file: None,
        }
    }
}

impl_builder_methods!(
    CreateFineTuningJobRequest,
    hyperparameters: HyperParameters,
    suffix: String,
    validation_file: String
);

/// Represents a request to list fine-tuning jobs.
#[derive(Debug, Serialize, Default)]
pub struct ListFineTuningJobsRequest {
    /// Optional cursor for pagination, specifying the starting point after a specific item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Optional limit for the number of items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListFineTuningJobsRequest {
    /// Creates a new `ListFineTuningJobsRequest`.
    pub fn new() -> Self {
        Self {
            after: None,
            limit: None,
        }
    }
}

/// Represents a request to list events of a specific fine-tuning job.
#[derive(Debug, Serialize)]
pub struct ListFineTuningJobEventsRequest {
    /// Identifier for the fine-tuning job.
    pub fine_tuning_job_id: String,
    /// Optional cursor for pagination, specifying the starting point after a specific item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Optional limit for the number of items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListFineTuningJobEventsRequest {
    /// Creates a new `ListFineTuningJobEventsRequest` with the specified fine-tuning job ID.
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self {
            fine_tuning_job_id,
            after: None,
            limit: None,
        }
    }
}

/// Represents a request to retrieve a specific fine-tuning job.
#[derive(Debug, Serialize)]
pub struct RetrieveFineTuningJobRequest {
    /// Identifier for the fine-tuning job.
    pub fine_tuning_job_id: String,
}

impl RetrieveFineTuningJobRequest {
    /// Creates a new `RetrieveFineTuningJobRequest` with the specified fine-tuning job ID.
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self { fine_tuning_job_id }
    }
}

/// Represents a request to cancel a fine-tuning job.
#[derive(Debug, Serialize)]
pub struct CancelFineTuningJobRequest {
    /// Identifier for the fine-tuning job.
    pub fine_tuning_job_id: String,
}

impl CancelFineTuningJobRequest {
    /// Creates a new `CancelFineTuningJobRequest` with the specified fine-tuning job ID.
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self { fine_tuning_job_id }
    }
}

/// Represents pagination information in fine-tuning job responses.
#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningPagination<T> {
    /// Object type, typically "list".
    pub object: String,
    /// Data contained in the current page.
    pub data: Vec<T>,
    /// Indicates if there are more items available.
    pub has_more: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a fine-tuning job object with various attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobObject {
    /// Unique identifier for the fine-tuning job.
    pub id: String,
    /// Timestamp of when the fine-tuning job was created.
    pub created_at: i64,
    /// Optional error information if the job failed.
    pub error: Option<FineTuningJobError>,
    /// Optional identifier for the fine-tuned model.
    pub fine_tuned_model: Option<String>,
    /// Optional timestamp of when the job finished.
    pub finished_at: Option<String>,
    /// Hyperparameters used for the fine-tuning job.
    pub hyperparameters: HyperParameters,
    /// Model being fine-tuned.
    pub model: String,
    /// Object type, typically "fine-tuning-job".
    pub object: String,
    /// Identifier for the organization owning the job.
    pub organization_id: String,
    /// List of files resulting from the fine-tuning job.
    pub result_files: Vec<String>,
    /// Status of the fine-tuning job.
    pub status: String,
    /// Optional number of tokens trained.
    pub trained_tokens: Option<i64>,
    /// File containing the training data.
    pub training_file: String,
    /// Optional file containing validation data.
    pub validation_file: Option<String>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents an error associated with a fine-tuning job.
#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobError {
    /// Error code.
    pub code: String,
    /// Error message.
    pub message: String,
    /// Optional parameter associated with the error.
    pub param: Option<String>,
}

/// Represents an event associated with a fine-tuning job.
#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobEvent {
    /// Unique identifier for the event.
    pub id: String,
    /// Timestamp of when the event was created.
    pub created_at: i64,
    /// Severity level of the event.
    pub level: String,
    /// Message describing the event.
    pub message: String,
    /// Object type, typically "fine-tuning-job-event".
    pub object: String,
}

/// Represents hyperparameters for a fine-tuning job.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HyperParameters {
    /// Optional batch size for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<String>,
    /// Optional learning rate multiplier for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<String>,
    /// Optional number of epochs for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<String>,
}
