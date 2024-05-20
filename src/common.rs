use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Deserialize, EnumString, Serialize, Clone, PartialEq, Eq, AsRefStr, Display,
)]
pub enum MessageRole {
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    #[serde(rename = "system")]
    #[strum(serialize = "system")]
    System,
    #[serde(rename = "assistant")]
    #[strum(serialize = "assistant")]
    Assistant,
    #[serde(rename = "function")]
    #[strum(serialize = "function")]
    Function,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[macro_export]
macro_rules! impl_builder_methods {
    ($builder:ident, $($field:ident: $field_type:ty),*) => {
        impl $builder {
            $(
                pub fn $field(mut self, $field: $field_type) -> Self {
                    self.$field = Some($field);
                    self
                }
            )*
        }
    };
}
