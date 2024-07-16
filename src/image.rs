//! This module defines the structures and methods for image generation, editing, and variations.
//! It includes:
//! - `ImageData`: Struct representing the data of an image, such as its URL.
//! - `ImageGenerationRequest`: Struct for creating a request to generate an image.
//! - `ImageGenerationResponse`: Struct for the response from an image generation request.
//! - `ImageEditRequest`: Struct for creating a request to edit an existing image.
//! - `ImageEditResponse`: Struct for the response from an image edit request.
//! - `ImageVariationRequest`: Struct for creating a request to generate variations of an image.
//! - `ImageVariationResponse`: Struct for the response from an image variation request.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::impl_builder_methods;

/// Represents the data of an image, such as its URL.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageData {
    /// URL of the image.
    pub url: String,
}

/// Represents a request to generate an image.
#[derive(Debug, Serialize, Clone)]
pub struct ImageGenerationRequest {
    /// Prompt for generating the image.
    pub prompt: String,
    /// Optional model to be used for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Optional size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Optional format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageGenerationRequest {
    /// Creates a new `ImageGenerationRequest` with the specified prompt.
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageGenerationRequest,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

/// Represents the response from an image generation request.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageGenerationResponse {
    /// Timestamp of when the image was created.
    pub created: i64,
    /// List of generated image data.
    pub data: Vec<ImageData>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to edit an existing image.
#[derive(Debug, Serialize, Clone)]
pub struct ImageEditRequest {
    /// Image to be edited.
    pub image: String,
    /// Optional mask to be applied to the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    /// Prompt for editing the image.
    pub prompt: String,
    /// Optional model to be used for image editing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Optional size of the edited image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Optional format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageEditRequest {
    /// Creates a new `ImageEditRequest` with the specified image and prompt.
    pub fn new(image: String, prompt: String) -> Self {
        Self {
            image,
            prompt,
            mask: None,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageEditRequest,
    mask: String,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

/// Represents the response from an image edit request.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageEditResponse {
    /// Timestamp of when the image was edited.
    pub created: i64,
    /// List of edited image data.
    pub data: Vec<ImageData>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to generate variations of an image.
#[derive(Debug, Serialize, Clone)]
pub struct ImageVariationRequest {
    /// Image to generate variations for.
    pub image: String,
    /// Optional number of variations to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// Optional model to be used for generating variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional size of the generated variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Optional format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Optional user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageVariationRequest {
    /// Creates a new `ImageVariationRequest` with the specified image.
    pub fn new(image: String) -> Self {
        Self {
            image,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageVariationRequest,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

/// Represents the response from an image variation request.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageVariationResponse {
    /// Timestamp of when the image variations were created.
    pub created: i64,
    /// List of generated image variation data.
    pub data: Vec<ImageData>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
