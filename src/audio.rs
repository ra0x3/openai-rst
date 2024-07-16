//! This module defines the structures and methods for handling audio-based requests and responses.
//! It includes functionality for audio transcription, translation, and speech synthesis.
//! The module facilitates the creation, serialization, and deserialization of requests and responses
//! related to these audio operations.
//!
//! Features include:
//! - AudioTranscriptionRequest: Handles requests for audio file transcription using specified models.
//! - AudioTranscriptionResponse: Returns the transcription text along with optional headers.
//! - AudioTranslationRequest: Manages requests for translating audio content using specific models.
//! - AudioTranslationResponse: Delivers translated text and optional response headers.
//! - AudioSpeechRequest: Manages requests for generating speech from text using designated voice models.
//! - AudioSpeechResponse: Provides the success status of the speech synthesis operation and optional headers.
//!
//! Constants for model and voice identifiers are also defined to standardize the references used across requests.

use crate::impl_builder_methods;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const WHISPER_1: &str = "whisper-1";

/// Represents a request for audio transcription.
#[derive(Debug, Serialize, Clone)]
pub struct AudioTranscriptionRequest {
    /// Path to the audio file to be transcribed.
    pub file: String,
    /// Model to be used for transcription.
    pub model: String,
    /// Optional prompt to guide the transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Optional format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Optional temperature setting for the transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Optional language of the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl AudioTranscriptionRequest {
    /// Creates a new `AudioTranscriptionRequest` with the specified file and model.
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
        }
    }
}

impl_builder_methods!(
    AudioTranscriptionRequest,
    prompt: String,
    response_format: String,
    temperature: f32,
    language: String
);

/// Represents the response from an audio transcription request.
#[derive(Debug, Deserialize, Serialize)]
pub struct AudioTranscriptionResponse {
    /// Transcribed text from the audio file.
    pub text: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request for audio translation.
#[derive(Debug, Serialize, Clone)]
pub struct AudioTranslationRequest {
    /// Path to the audio file to be translated.
    pub file: String,
    /// Model to be used for translation.
    pub model: String,
    /// Optional prompt to guide the translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Optional format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Optional temperature setting for the translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl AudioTranslationRequest {
    /// Creates a new `AudioTranslationRequest` with the specified file and model.
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }
}

impl_builder_methods!(
    AudioTranslationRequest,
    prompt: String,
    response_format: String,
    temperature: f32
);

/// Represents the response from an audio translation request.
#[derive(Debug, Deserialize, Serialize)]
pub struct AudioTranslationResponse {
    /// Translated text from the audio file.
    pub text: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Constant for the TTS-1 model identifier.
pub const TTS_1: &str = "tts-1";
/// Constant for the TTS-1 HD model identifier.
pub const TTS_1_HD: &str = "tts-1-hd";

/// Constant for the Alloy voice model.
pub const VOICE_ALLOY: &str = "alloy";
/// Constant for the Echo voice model.
pub const VOICE_ECHO: &str = "echo";
/// Constant for the Fable voice model.
pub const VOICE_FABLE: &str = "fable";
/// Constant for the Onyx voice model.
pub const VOICE_ONYX: &str = "onyx";
/// Constant for the Nova voice model.
pub const VOICE_NOVA: &str = "nova";
/// Constant for the Shimmer voice model.
pub const VOICE_SHIMMER: &str = "shimmer";

/// Represents a request for text-to-speech synthesis.
#[derive(Debug, Serialize, Clone)]
pub struct AudioSpeechRequest {
    /// Model to be used for speech synthesis.
    pub model: String,
    /// Input text to be synthesized.
    pub input: String,
    /// Voice model to be used for the synthesis.
    pub voice: String,
    /// Output format for the synthesized speech.
    pub output: String,
}

impl AudioSpeechRequest {
    /// Creates a new `AudioSpeechRequest` with the specified model, input, voice, and output format.
    pub fn new(model: String, input: String, voice: String, output: String) -> Self {
        Self {
            model,
            input,
            voice,
            output,
        }
    }
}

impl_builder_methods!(AudioSpeechRequest,);

/// Represents the response from a text-to-speech synthesis request.
#[derive(Debug, Deserialize, Serialize)]
pub struct AudioSpeechResponse {
    /// Indicates whether the synthesis was successful.
    pub result: bool,
}
