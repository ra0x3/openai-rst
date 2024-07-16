//! This module defines the structures and methods for handling text completion requests and responses.
//! It includes:
//! - `CompletionRequest`: Struct for creating a request to generate text completions.
//! - `CompletionChoice`: Struct representing a single completion choice from the response.
//! - `LogprobResult`: Struct for log probability results associated with completions.
//! - `CompletionResponse`: Struct for the response from a completion request.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::{common, impl_builder_methods, models::Model};

/// Represents a request to generate text completions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionRequest {
    /// Model to be used for generating completions.
    pub model: Model,
    /// Prompt for generating the completions.
    pub prompt: String,
    /// Optional suffix that comes after the generated text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Optional maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    /// Optional temperature setting for sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Optional top-p setting for nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Optional number of completions to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Optional flag to stream the completions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Optional number of log probabilities to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i32>,
    /// Optional flag to echo the prompt in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    /// Optional sequences where the generation will stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Optional penalty for presence of tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Optional penalty for frequency of tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Optional number of best completions to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,
    /// Optional bias for log probabilities of specific tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CompletionRequest {
    /// Creates a new `CompletionRequest` with the specified model and prompt.
    pub fn new(model: Model, prompt: String) -> Self {
        Self {
            model,
            prompt,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    CompletionRequest,
    suffix: String,
    max_tokens: i32,
    temperature: f32,
    top_p: f32,
    n: i32,
    stream: bool,
    logprobs: i32,
    echo: bool,
    stop: Vec<String>,
    presence_penalty: f32,
    frequency_penalty: f32,
    best_of: i32,
    logit_bias: HashMap<String, i32>,
    user: String
);

/// Represents a single completion choice from the response.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionChoice {
    /// Generated text for the completion.
    pub text: String,
    /// Index of the completion choice.
    pub index: i64,
    /// Reason why the completion finished.
    pub finish_reason: String,
    /// Optional log probability results for the tokens.
    pub logprobs: Option<LogprobResult>,
}

/// Represents log probability results associated with completions.
#[derive(Debug, Deserialize, Serialize)]
pub struct LogprobResult {
    /// Tokens generated in the completion.
    pub tokens: Vec<String>,
    /// Log probabilities of the tokens.
    pub token_logprobs: Vec<f32>,
    /// Top log probabilities for the tokens.
    pub top_logprobs: Vec<HashMap<String, f32>>,
    /// Text offsets for the tokens.
    pub text_offset: Vec<i32>,
}

/// Represents the response from a completion request.
#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionResponse {
    /// Unique identifier for the completion response.
    pub id: String,
    /// Object type, typically "completion".
    pub object: String,
    /// Timestamp of when the completion was created.
    pub created: i64,
    /// Model used for generating the completion.
    pub model: String,
    /// List of completion choices.
    pub choices: Vec<CompletionChoice>,
    /// Usage information for the completion request.
    pub usage: common::Usage,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
