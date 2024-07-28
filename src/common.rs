//! This module defines enums, structs, and macros for handling message roles and usage metrics.
//! It includes:
//! - `MessageRole`: Enum representing different roles in a messaging system.
//! - `Usage`: Struct for tracking token usage in prompts and completions.
//! - `impl_builder_methods!`: Macro for generating builder methods for structs.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// Represents different roles in a messaging system.
#[derive(
    Debug, Deserialize, EnumString, Serialize, Clone, PartialEq, Eq, AsRefStr, Display,
)]
pub enum MessageRole {
    /// Represents a user role.
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    /// Represents a system role.
    #[serde(rename = "system")]
    #[strum(serialize = "system")]
    System,
    /// Represents an assistant role.
    #[serde(rename = "assistant")]
    #[strum(serialize = "assistant")]
    Assistant,
    /// Represents a function role.
    #[serde(rename = "function")]
    #[strum(serialize = "function")]
    Function,
}

/// Struct for tracking token usage.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Usage {
    /// Number of tokens used in the prompt.
    pub prompt_tokens: i32,
    /// Number of tokens used in the completion.
    pub completion_tokens: i32,
    /// Total number of tokens used.
    pub total_tokens: i32,
}

/// Macro for generating builder methods for a struct.
#[macro_export]
macro_rules! impl_builder_methods {
    ($builder:ident, $($field:ident: $field_type:ty),*) => {
        impl $builder {
            $(
                /// Sets the value of the specified field.
                pub fn $field(mut self, $field: $field_type) -> Self {
                    self.$field = Some($field);
                    self
                }
            )*
        }
    };
}
