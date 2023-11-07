use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::{AssistantTools, CreateMessageRequest};

/// Represents a thread that contains [messages](https://platform.openai.com/docs/api-reference/messages).
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ThreadObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `thread`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: i32,

    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateThreadRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateThreadRequest {
    /// A list of [messages](https://platform.openai.com/docs/api-reference/messages) to start the thread with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<CreateMessageRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ModifyThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct DeleteThreadResponse {
    pub id: String,
    pub deleted: bool,
    pub object: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateThreadAndRunRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateThreadAndRunRequest {
    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) to use to execute this run.
    pub assistant_id: String,

    /// If no thread is provided, an empty thread will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<CreateThreadRequest>,

    /// The ID of the [Model](https://platform.openai.com/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTools>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}
