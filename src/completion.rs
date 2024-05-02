use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::{common, impl_builder_methods, models::Model};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionRequest {
    pub model: Model,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CompletionRequest {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: i64,
    pub finish_reason: String,
    pub logprobs: Option<LogprobResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogprobResult {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<f32>,
    pub top_logprobs: Vec<HashMap<String, f32>>,
    pub text_offset: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: common::Usage,
    pub headers: Option<HashMap<String, String>>,
}
