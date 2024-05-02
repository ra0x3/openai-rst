use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

// https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone)]
pub enum GPT4 {
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    GPT4Turbo20240409,
    #[serde(rename = "gpt-4-turbo-preview")]
    GPT4TurboPreview,
    #[serde(rename = "gpt-4-0125-preview")]
    GPT40125Preview,
    #[serde(rename = "gpt-4-1106-preview")]
    GPT41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    GPT4VisionPreview,
    #[serde(rename = "gpt-4-1106-vision-preview")]
    GPT41106VisionPreview,
    #[serde(rename = "gpt-4")]
    GPT4,
    #[serde(rename = "gpt-4-0613")]
    GPT40613,
    #[serde(rename = "gpt-4-32k")]
    GPT432K,
    #[serde(rename = "gpt-4-32k-0613")]
    GPT432K0613,
}

// https://platform.openai.com/docs/models/gpt-3-5
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone)]
pub enum GPT3 {
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35Turbo,
    #[serde(rename = "gpt-3")]
    GPT3,
    #[serde(rename = "davinci")]
    Davinci,
    #[serde(rename = "curie")]
    Curie,
    #[serde(rename = "babbage")]
    Babbage,
    #[serde(rename = "ada")]
    Ada,
}

// https://platform.openai.com/docs/api-reference/images/object
#[derive(EnumString, Debug, Serialize, Deserialize, AsRefStr, Clone)]
pub enum Dalle {
    #[serde(rename = "dalle-2")]
    Dalle2,
    #[serde(rename = "dalle-mini")]
    DalleMini,
    #[serde(rename = "dalle-mega")]
    DalleMega,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone)]
pub enum Whisper {
    #[serde(rename = "whisper-large")]
    WhisperLarge,
    #[serde(rename = "whisper-medium")]
    WhisperMedium,
    #[serde(rename = "whisper-small")]
    WhisperSmall,
    #[serde(rename = "whisper-tiny")]
    WhisperTiny,
}

#[derive(EnumString, Debug, Serialize, Deserialize, Clone)]
pub enum ClipModels {
    #[serde(rename = "clip-vit-base-patch32")]
    ClipVitBasePatch32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Model {
    GPT4(GPT4),
    GPT3(GPT3),
    Dalle(Dalle),
    Whisper(Whisper),
    ClipModels(ClipModels),
}
