//! This module defines the structures and methods for creating and handling moderation requests and responses.
//! It includes:
//! - `CreateModerationRequest`: Struct for creating a moderation request with optional model specification.
//! - `CreateModerationResponse`: Struct for the response from a moderation request.
//! - `ModerationResult`: Struct representing the result of moderation, including categories and scores.
//! - `ModerationCategories`: Struct for categorizing the types of content flagged by moderation.
//! - `ModerationCategoryScores`: Struct for scoring the likelihood of each moderation category.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

/// Represents a request to create a moderation check.
#[derive(Debug, Serialize, Clone)]
pub struct CreateModerationRequest {
    /// Input text to be moderated.
    pub input: String,
    /// Optional model to be used for moderation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl CreateModerationRequest {
    /// Creates a new `CreateModerationRequest` with the specified input text.
    pub fn new(input: String) -> Self {
        Self { input, model: None }
    }
}

impl_builder_methods!(
    CreateModerationRequest,
    model: String
);

/// Represents the response from a moderation check.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateModerationResponse {
    /// Unique identifier for the moderation response.
    pub id: String,
    /// Model used for the moderation.
    pub model: String,
    /// Results of the moderation check.
    pub results: Vec<ModerationResult>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a single result from a moderation check.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModerationResult {
    /// Categories of content flagged by moderation.
    pub categories: ModerationCategories,
    /// Scores indicating the likelihood of each category.
    pub category_scores: ModerationCategoryScores,
    /// Indicates if the content was flagged.
    pub flagged: bool,
}

/// Represents the categories of content flagged by moderation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModerationCategories {
    /// Indicates if the content is categorized as hate speech.
    #[serde(rename = "hate")]
    pub is_hate: bool,
    /// Indicates if the content is categorized as threatening hate speech.
    #[serde(rename = "hate/threatening")]
    pub is_hate_threatening: bool,
    /// Indicates if the content is categorized as self-harm.
    #[serde(rename = "self-harm")]
    pub is_self_harm: bool,
    /// Indicates if the content is categorized as sexual.
    pub sexual: bool,
    /// Indicates if the content is categorized as sexual content involving minors.
    #[serde(rename = "sexual/minors")]
    pub is_sexual_minors: bool,
    /// Indicates if the content is categorized as violent.
    pub violence: bool,
    /// Indicates if the content is categorized as graphic violence.
    #[serde(rename = "violence/graphic")]
    pub is_violence_graphic: bool,
}

/// Represents the scores indicating the likelihood of each moderation category.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModerationCategoryScores {
    /// Likelihood score for hate speech.
    #[serde(rename = "hate")]
    pub hate_score: f64,
    /// Likelihood score for threatening hate speech.
    #[serde(rename = "hate/threatening")]
    pub hate_threatening_score: f64,
    /// Likelihood score for self-harm content.
    #[serde(rename = "self-harm")]
    pub self_harm_score: f64,
    /// Likelihood score for sexual content.
    pub sexual: f64,
    /// Likelihood score for sexual content involving minors.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors_score: f64,
    /// Likelihood score for violent content.
    pub violence: f64,
    /// Likelihood score for graphic violence.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic_score: f64,
}
