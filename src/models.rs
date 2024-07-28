//! This module defines various enums and structs representing different AI models, such as GPT-4, GPT-3, Dalle, Whisper, Clip, and Embeddings models.
//! Each enum variant corresponds to a specific model version or type, providing detailed information about the available models.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// Enum representing different versions of the GPT-4 model.
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT4 {
    /// The turbo version of GPT-4, optimized for performance.
    #[strum(serialize = "gpt-4-turbo")]
    GPT4Turbo,

    /// The preview version of GPT-4 turbo, available for early testing.
    #[strum(serialize = "gpt-4-turbo-preview")]
    GPT4TurboPreview,

    /// The standard GPT-4 model.
    #[strum(serialize = "gpt-4")]
    GPT4,

    /// A specific preview version of GPT-4 with the identifier 0125.
    #[strum(serialize = "gpt-4-0125-preview")]
    GPT40125Preview,

    /// An optimized version of GPT-4, referred to as GPT-4o.
    #[strum(serialize = "gpt-4o")]
    GPT4o,
}

/// Enum representing different versions of the GPT-3 model.
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT3 {
    /// The instruct version of GPT-3.5, designed for following instructions.
    #[strum(serialize = "gpt-3.5-turbo-instruct")]
    GPT35TurboInstruct,

    /// The turbo version of GPT-3.5, optimized for performance.
    #[strum(serialize = "gpt-3.5-turbo")]
    GPT35Turbo,

    /// A specific preview version of GPT-3.5 with the identifier 0125.
    #[strum(serialize = "gpt-3.5-0125-preview")]
    GPT350125Preview,
}

/// Enum representing different versions of the Dalle model for image generation.
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum Dalle {
    /// Dalle 2, known for generating high-quality images from textual descriptions.
    Dalle2,

    /// Dalle Mini, a smaller, more lightweight version of the Dalle model.
    DalleMini,

    /// Dalle Mega, a larger version of the Dalle model for more complex image generation.
    DalleMega,
}

/// Enum representing different versions of the Whisper model for speech recognition.
#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum Whisper {
    /// Large version of the Whisper model, offering high accuracy.
    WhisperLarge,

    /// Medium version of the Whisper model, balancing performance and resource usage.
    WhisperMedium,

    /// Small version of the Whisper model, optimized for speed and efficiency.
    WhisperSmall,

    /// Tiny version of the Whisper model, suitable for lightweight applications.
    WhisperTiny,
}

/// Enum representing different versions of the Clip model for image and text embeddings.
#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClipModels {
    /// Clip model with the VitBasePatch32 architecture, used for creating embeddings from images and text.
    ClipVitBasePatch32,
}

/// Enum representing different models for generating text embeddings.
#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum EmbeddingsModels {
    /// Ada version 002 for generating text embeddings.
    TextEmbeddingAda002,

    /// Babbage version 001 for generating text embeddings.
    TextEmbeddingBabbage001,

    /// Babbage version 002 for generating text embeddings.
    TextEmbeddingBabbage002,

    /// Curie version 001 for generating text embeddings.
    TextEmbeddingCurie001,

    /// Curie version 002 for generating text embeddings.
    TextEmbeddingCurie002,

    /// Davinci version 001 for generating text embeddings.
    TextEmbeddingDavinci001,

    /// Davinci version 002 for generating text embeddings.
    TextEmbeddingDavinci002,
}

/// Enum representing various AI models.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Model {
    /// GPT-4 models for advanced language processing.
    GPT4(GPT4),

    /// GPT-3 models for general language processing.
    GPT3(GPT3),

    /// Dalle models for image generation.
    Dalle(Dalle),

    /// Whisper models for speech recognition.
    Whisper(Whisper),

    /// Clip models for image and text embeddings.
    Clip(ClipModels),

    /// Embeddings models for generating text embeddings.
    Embedding(EmbeddingsModels),
}

impl Default for Model {
    /// Provides a default implementation for `Model`, defaulting to GPT-4o.
    fn default() -> Self {
        Model::GPT4(GPT4::GPT4o)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Model {
    /// Converts a `Model` enum variant to a string representation.
    fn to_string(&self) -> String {
        match self {
            Model::GPT4(model) => model.to_string(),
            Model::GPT3(model) => model.to_string(),
            Model::Dalle(model) => model.to_string(),
            Model::Whisper(model) => model.to_string(),
            Model::Clip(model) => model.to_string(),
            Model::Embedding(model) => model.to_string(),
        }
    }
}
