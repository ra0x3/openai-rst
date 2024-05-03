use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

// https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT4 {
    #[serde(rename = "gpt-4-turbo")]
    #[strum(serialize = "gpt-4-turbo")]
    GPT4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    #[strum(serialize = "gpt-4-turbo-2024-04-09")]
    GPT4Turbo20240409,
    #[serde(rename = "gpt-4-turbo-preview")]
    #[strum(serialize = "gpt-4-turbo-preview")]
    GPT4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    #[strum(serialize = "gpt-4-0125-preview")]
    GPT40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    #[strum(serialize = "gpt-4-1106-preview")]
    GPT41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    #[strum(serialize = "gpt-4-vision-preview")]
    GPT4VisionPreview,
    #[serde(rename = "gpt-4-1106-vision-preview")]
    #[strum(serialize = "gpt-4-1106-vision-preview")]
    GPT41106VisionPreview,
    #[serde(rename = "gpt-4")]
    #[strum(serialize = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-4-0613")]
    #[strum(serialize = "gpt-4-0613")]
    GPT40613,
    #[serde(rename = "gpt-4-32k")]
    #[strum(serialize = "gpt-4-32k")]
    GPT432K,
    #[serde(rename = "gpt-4-32k-0613")]
    #[strum(serialize = "gpt-4-32k-0613")]
    GPT432K0613,
}

// https://platform.openai.com/docs/models/gpt-3-5
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum GPT3 {
    #[serde(rename = "gpt-3.5-turbo")]
    #[strum(serialize = "gpt-3.5-turbo")]
    GPT35Turbo,
    #[serde(rename = "gpt-3")]
    #[strum(serialize = "gpt-3")]
    GPT3,
    #[serde(rename = "davinci")]
    #[strum(serialize = "davinci")]
    Davinci,
    #[serde(rename = "curie")]
    #[strum(serialize = "curie")]
    Curie,
    #[serde(rename = "babbage")]
    #[strum(serialize = "babbage")]
    Babbage,
    #[serde(rename = "ada")]
    #[strum(serialize = "ada")]
    Ada,
}

// https://platform.openai.com/docs/api-reference/images/object
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone, Display)]
pub enum Dalle {
    #[serde(rename = "dalle-2")]
    #[strum(serialize = "dalle-2")]
    Dalle2,
    #[serde(rename = "dalle-mini")]
    #[strum(serialize = "dalle-mini")]
    DalleMini,
    #[serde(rename = "dalle-mega")]
    #[strum(serialize = "dalle-mega")]
    DalleMega,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum Whisper {
    #[serde(rename = "whisper-large")]
    #[strum(serialize = "whisper-large")]
    WhisperLarge,
    #[serde(rename = "whisper-medium")]
    #[strum(serialize = "whisper-medium")]
    WhisperMedium,
    #[serde(rename = "whisper-small")]
    #[strum(serialize = "whisper-small")]
    WhisperSmall,
    #[serde(rename = "whisper-tiny")]
    #[strum(serialize = "whisper-tiny")]
    WhisperTiny,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum ClipModels {
    #[serde(rename = "clip-vit-base-patch32")]
    #[strum(serialize = "clip-vit-base-patch32")]
    ClipVitBasePatch32,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone, Display)]
pub enum EmbeddingsModels {
    #[serde(rename = "text-embedding-ada-002")]
    #[strum(serialize = "text-embedding-ada-002")]
    TextEmbeddingAda002,
    #[serde(rename = "text-embedding-babbage-001")]
    #[strum(serialize = "text-embedding-babbage-001")]
    TextEmbeddingBabbage001,
    #[serde(rename = "text-embedding-babbage-002")]
    #[strum(serialize = "text-embedding-babbage-002")]
    TextEmbeddingBabbage002,
    #[serde(rename = "text-embedding-curie-001")]
    #[strum(serialize = "text-embedding-curie-001")]
    TextEmbeddingCurie001,
    #[serde(rename = "text-embedding-curie-002")]
    #[strum(serialize = "text-embedding-curie-002")]
    TextEmbeddingCurie002,
    #[serde(rename = "text-embedding-davinci-001")]
    #[strum(serialize = "text-embedding-davinci-001")]
    TextEmbeddingDavinci001,
    #[serde(rename = "text-embedding-davinci-002")]
    #[strum(serialize = "text-embedding-davinci-002")]
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
