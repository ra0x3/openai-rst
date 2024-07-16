//! This library provides a comprehensive set of modules for interacting with various AI models and services.
//! It includes functionality for handling assistants, audio processing, chat completions, and more.
//! Each submodule is designed to encapsulate specific features and operations, making it easy to integrate and use in different applications.

/// Module for managing assistants and related operations.
pub mod assistant;

/// Module for audio processing, including transcription, translation, and speech synthesis.
pub mod audio;

/// Module for handling chat completion requests and responses.
pub mod chat_completion;

/// Module for the main client interface to interact with the services.
pub mod client;

/// Common utilities and types used across multiple modules.
pub mod common;

/// Module for handling text completion requests and responses.
pub mod completion;

/// Module for editing operations.
pub mod edit;

/// Module for generating text embeddings.
pub mod embedding;

/// Module for error handling.
pub mod error;

/// Module for managing files and related operations.
pub mod file;

/// Module for fine-tuning models.
pub mod fine_tuning;

/// Module for image generation and processing.
pub mod image;

/// Module for creating, modifying, and managing messages.
pub mod message;

/// Module defining various AI models.
pub mod models;

/// Module for moderation checks and responses.
pub mod moderation;

/// Module for creating and managing runs.
pub mod run;

/// Module for creating and managing threads.
pub mod thread;
