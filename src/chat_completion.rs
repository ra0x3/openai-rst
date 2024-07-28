//! This module defines the structures and methods for handling chat completion requests and responses.
//! It includes the `ChatCompletionRequest`, `ChatCompletionResponse`, `ChatCompletionMessage`,
//! `ChatCompletionChoice`, `Function`, `FunctionParameters`, `JSONSchemaType`, `JSONSchemaDefine`,
//! `FinishReason`, `FinishDetails`, `ToolCall`, `ToolCallFunction`, and `Tool` structs along with their associated methods.
//! These structures facilitate the creation, serialization, and deserialization of chat completion requests and responses
//! in various formats, allowing for customizable and extensible interactions with chat models.

use crate::{
    common::{MessageRole, Usage},
    impl_builder_methods,
    models::Model,
};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;

/// Represents the type of tool choice in the request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ToolChoiceType {
    /// No tool chosen.
    None,
    /// Automatic tool choice.
    Auto,
    /// Specific tool choice.
    ToolChoice {
        /// The chosen tool.
        tool: Tool,
    },
}

/// Represents a request for chat completion.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    /// Model to be used for the completion.
    pub model: String,
    /// List of messages for the completion.
    pub messages: Vec<ChatCompletionMessage>,
    /// Sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Number of completions to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    /// Whether to stream back partial progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Positive values penalize new tokens based on their existing frequency in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Positive values penalize new tokens based on their frequency in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Seed for random number generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Tools available for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Choice of tool for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_tool_choice")]
    pub tool_choice: Option<ToolChoiceType>,
}

impl ChatCompletionRequest {
    /// Creates a new `ChatCompletionRequest` with a single message.
    pub fn new(model: Model, message: ChatCompletionMessage) -> Self {
        let model = model.to_string();
        Self {
            model,
            messages: vec![message],
            temperature: None,
            top_p: None,
            stream: None,
            n: None,
            response_format: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            seed: None,
            tools: None,
            tool_choice: None,
        }
    }

    /// Creates a new `ChatCompletionRequest` with multiple messages.
    pub fn new_multi(model: Model, messages: Vec<ChatCompletionMessage>) -> Self {
        let model = model.to_string();
        Self {
            model,
            messages,
            temperature: None,
            top_p: None,
            stream: None,
            n: None,
            response_format: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            seed: None,
            tools: None,
            tool_choice: None,
        }
    }
}

impl From<&str> for ChatCompletionRequest {
    /// Converts a string into a `ChatCompletionRequest`.
    fn from(text: &str) -> Self {
        ChatCompletionRequest::new(
            Model::GPT4(crate::models::GPT4::GPT4o),
            ChatCompletionMessage {
                role: MessageRole::User,
                content: Content::Text(text.to_string()),
                name: None,
            },
        )
    }
}

impl From<String> for ChatCompletionRequest {
    /// Converts a string into a `ChatCompletionRequest`.
    fn from(text: String) -> Self {
        ChatCompletionRequest::new(
            Model::GPT4(crate::models::GPT4::GPT4o),
            ChatCompletionMessage {
                role: MessageRole::User,
                content: Content::Text(text),
                name: None,
            },
        )
    }
}

impl_builder_methods!(
    ChatCompletionRequest,
    temperature: f64,
    top_p: f64,
    n: i64,
    response_format: Value,
    stream: bool,
    stop: Vec<String>,
    max_tokens: i64,
    presence_penalty: f64,
    frequency_penalty: f64,
    logit_bias: HashMap<String, i32>,
    user: String,
    seed: i64,
    tools: Vec<Tool>,
    tool_choice: ToolChoiceType
);

/// Represents the content of a message.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Content {
    /// Text content.
    Text(String),
    /// URL to an image.
    ImageUrl(Vec<ImageUrl>),
}

impl From<&str> for Content {
    /// Converts a string into `Content::Text`.
    fn from(text: &str) -> Self {
        Content::Text(text.to_string())
    }
}

impl From<Vec<&str>> for Content {
    /// Converts a vector of strings into `Content::ImageUrl`.
    fn from(texts: Vec<&str>) -> Self {
        Content::ImageUrl(
            texts
                .iter()
                .map(|text| ImageUrl {
                    r#type: ContentType::image_url,
                    text: None,
                    image_url: Some(ImageUrlType {
                        url: text.to_string(),
                    }),
                })
                .collect(),
        )
    }
}

impl serde::Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Content::Text(ref text) => serializer.serialize_str(text),
            Content::ImageUrl(ref image_url) => image_url.serialize(serializer),
        }
    }
}

/// Represents the type of content.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ContentType {
    /// Text content type.
    text,
    /// Image URL content type.
    image_url,
}

/// Represents the URL of an image.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct ImageUrlType {
    /// URL of the image.
    pub url: String,
}

/// Represents an image URL.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct ImageUrl {
    /// Type of content.
    pub r#type: ContentType,
    /// Optional text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional image URL type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrlType>,
}

/// Represents a chat completion message.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    /// Role of the message sender.
    pub role: MessageRole,
    /// Content of the message.
    pub content: Content,
    /// Optional name of the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<&str> for ChatCompletionMessage {
    /// Converts a string into a `ChatCompletionMessage`.
    fn from(text: &str) -> Self {
        ChatCompletionMessage {
            role: MessageRole::User,
            content: Content::Text(text.to_string()),
            name: None,
        }
    }
}

/// Represents a chat completion message for a response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionMessageForResponse {
    /// Role of the message sender.
    pub role: MessageRole,
    /// Optional content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Optional name of the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional tool calls related to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Represents a choice in a chat completion response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionChoice {
    /// Index of the choice.
    pub index: i64,
    /// Message corresponding to the choice.
    pub message: ChatCompletionMessageForResponse,
    /// Reason for finishing the response.
    pub finish_reason: Option<FinishReason>,
    /// Additional details for the finish reason.
    pub finish_details: Option<FinishDetails>,
}

/// Represents a chat completion response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for the response.
    pub id: String,
    /// Object type.
    pub object: String,
    /// Creation timestamp.
    pub created: i64,
    /// Model used for the completion.
    pub model: String,
    /// List of choices in the response.
    pub choices: Vec<ChatCompletionChoice>,
    /// Usage information.
    pub usage: Usage,
    /// Optional system fingerprint.
    pub system_fingerprint: Option<String>,
    /// Optional headers in the response.
    pub headers: Option<HashMap<String, String>>,
}

impl ChatCompletionResponse {
    /// Gets the content of the first choice.
    pub fn get_choice(&self) -> String {
        self.choices[0].message.content.clone().unwrap_or_default()
    }
}
/// Represents a function definition.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Function {
    /// Name of the function.
    pub name: String,
    /// Optional description of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters of the function.
    pub parameters: FunctionParameters,
}

/// Represents the JSON schema type.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JSONSchemaType {
    /// Object type.
    Object,
    /// Number type.
    Number,
    /// String type.
    String,
    /// Array type.
    Array,
    /// Null type.
    Null,
    /// Boolean type.
    Boolean,
}

/// Defines the structure of a JSON schema.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct JSONSchemaDefine {
    /// Type of the schema.
    #[serde(rename = "type")]
    pub schema_type: Option<JSONSchemaType>,
    /// Optional description of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional enumeration values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    /// Optional properties of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    /// Optional required properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// Optional items in the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JSONSchemaDefine>>,
}

/// Represents the parameters of a function using JSON schema.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FunctionParameters {
    /// Schema type of the parameters.
    #[serde(rename = "type")]
    pub schema_type: JSONSchemaType,
    /// Optional properties of the parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    /// Optional required properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// Reason for finishing the response.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FinishReason {
    /// Finished due to reaching stop condition.
    stop,
    /// Finished due to reaching maximum length.
    length,
    /// Finished due to content filtering.
    content_filter,
    /// Finished due to tool calls.
    tool_calls,
    /// Null finish reason.
    null,
}

/// Additional details for the finish reason.
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub struct FinishDetails {
    /// Type of finish reason.
    pub r#type: FinishReason,
    /// Stop condition.
    pub stop: String,
}

/// Represents a tool call in the chat completion response.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCall {
    /// Unique identifier for the tool call.
    pub id: String,
    /// Type of tool call.
    pub r#type: String,
    /// Function associated with the tool call.
    pub function: ToolCallFunction,
}

/// Represents a function associated with a tool call.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCallFunction {
    /// Optional name of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional arguments for the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

/// Serializes the tool choice type.
fn serialize_tool_choice<S>(
    value: &Option<ToolChoiceType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(ToolChoiceType::None) => serializer.serialize_str("none"),
        Some(ToolChoiceType::Auto) => serializer.serialize_str("auto"),
        Some(ToolChoiceType::ToolChoice { tool }) => {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &tool.r#type)?;
            map.serialize_entry("function", &tool.function)?;
            map.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Represents a tool in the request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tool {
    /// Type of the tool.
    pub r#type: ToolType,
    /// Function of the tool.
    pub function: Function,
}

/// Enum for different types of tools.
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Represents a function tool type.
    Function,
}
