use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT4 {
    #[strum(serialize = "gpt-4-turbo")]
    GPT4Turbo,

    #[strum(serialize = "gpt-4-turbo-preview")]
    GPT4TurboPreview,

    #[strum(serialize = "gpt-4")]
    GPT4,

    #[strum(serialize = "gpt-4-0125-preview")]
    GPT40125Preview,

    #[strum(serialize = "gpt-4o")]
    GPT4o,
}

#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT3 {
    #[strum(serialize = "gpt-3.5-turbo-instruct")]
    GPT35TurboInstruct,

    #[strum(serialize = "gpt-3.5-turbo")]
    GPT35Turbo,

    #[strum(serialize = "gpt-3.5-0125-preview")]
    GPT350125Preview,
}

#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum Dalle {
    Dalle2,
    DalleMini,
    DalleMega,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum Whisper {
    WhisperLarge,
    WhisperMedium,
    WhisperSmall,
    WhisperTiny,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClipModels {
    ClipVitBasePatch32,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum EmbeddingsModels {
    TextEmbeddingAda002,
    TextEmbeddingBabbage001,
    TextEmbeddingBabbage002,
    TextEmbeddingCurie001,
    TextEmbeddingCurie002,
    TextEmbeddingDavinci001,
    TextEmbeddingDavinci002,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum Model {
    GPT4(GPT4),
    GPT3(GPT3),
    Dalle(Dalle),
    Whisper(Whisper),
    Clip(ClipModels),
    Embedding(EmbeddingsModels),
}
