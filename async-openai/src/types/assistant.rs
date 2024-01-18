use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::FunctionObject;

/// Represents an `assistant` that can call the model and use tools.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct AssistantObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `assistant`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub created_at: i32,
    /// The name of the assistant. The maximum length is 256 characters.
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 512 characters.
    pub description: Option<String>,
    pub model: String,
    /// The system instructions that the assistant uses. The maximum length is 32768 characters.
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    ///  Tools can be of types `code_interpreter`, `retrieval`, or `function`.
    pub tools: Vec<AssistantTools>,
    /// A list of [file](/docs/api-reference/files) IDs attached to this assistant.
    /// There can be a maximum of 20 files attached to the assistant. Files are ordered by their creation date in ascending order.
    pub file_ids: Vec<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Code interpreter tool
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct AssistantToolsCode {
    pub r#type: String,
}

/// Retrieval tool
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct AssistantToolsRetrieval {
    pub r#type: String,
}

/// Function tool
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct AssistantToolsFunction {
    pub r#type: String,
    pub function: FunctionObject,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AssistantTools {
    Code(AssistantToolsCode),
    Retrieval(AssistantToolsRetrieval),
    Function(AssistantToolsFunction),
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateAssistantRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateAssistantRequest {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTools>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "ModifyAssistantRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ModifyAssistantRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTools>>,

    /// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs attached to this assistant. There can be a maximum of 20 files attached to the assistant. Files are ordered by their creation date in ascending order. If a file was previously attached to the list but does not show up in the list, it will be deleted from the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct DeleteAssistantResponse {
    pub id: String,
    pub deleted: bool,
    pub object: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ListAssistantsResponse {
    pub object: String,
    pub data: Vec<AssistantObject>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
