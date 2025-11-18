use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing chat completions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListChatCompletionsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing chat completions.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListChatCompletionsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListChatCompletionsQuery {
    /// The model used to generate the Chat Completions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// A list of metadata keys to filter the Chat Completions by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Identifier for the last chat completion from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of Chat Completions to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for Chat Completions by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListChatCompletionsOrder>,
}

/// Sort order for listing chat completion messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GetChatCompletionMessagesOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for getting chat completion messages.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "GetChatCompletionMessagesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct GetChatCompletionMessagesQuery {
    /// Identifier for the last message from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of messages to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for messages by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<GetChatCompletionMessagesOrder>,
}
