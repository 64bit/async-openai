use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::responses::InputItem;

/// Represents a conversation object.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ConversationObject {
    /// The unique ID of the conversation.
    pub id: String,
    /// The object type, which is always `conversation`.
    pub object: String,
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, serde_json::Value>,
    /// The time at which the conversation was created, measured in seconds since the Unix epoch.
    pub created_at: i64,
}

/// Request to create a conversation.
#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateConversationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateConversationRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    
    /// Initial items to include in the conversation context. You may add up to 20 items at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<InputItem>>,
}

/// Request to update a conversation.
#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "UpdateConversationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateConversationRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Represents a deleted conversation.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct DeleteConversationResponse {
    /// The unique ID of the deleted conversation.
    pub id: String,
    /// The object type, which is always `conversation.deleted`.
    pub object: String,
    /// Whether the conversation was successfully deleted.
    pub deleted: bool,
}

/// Request to create conversation items.
#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateConversationItemsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateConversationItemsRequest {
    /// The items to add to the conversation. You may add up to 20 items at a time.
    pub items: Vec<InputItem>,
}

/// A list of Conversation items.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ConversationItemList {
    /// The type of object returned, must be `list`.
    pub object: String,
    /// A list of conversation items.
    pub data: Vec<ConversationItem>,
    /// Whether there are more items available.
    pub has_more: bool,
    /// The ID of the first item in the list.
    pub first_id: String,
    /// The ID of the last item in the list.
    pub last_id: String,
}

/// A single item within a conversation.
/// This is an alias to the Item type from the responses module, 
/// as conversation items use the same structure.
pub use super::responses::Item as ConversationItem;

/// Additional fields to include in the response.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncludeParam {
    /// Include the sources of the web search tool call.
    #[serde(rename = "web_search_call.action.sources")]
    WebSearchCallActionSources,
    /// Include the outputs of python code execution in code interpreter tool call items.
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
    /// Include image urls from the computer call output.
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    /// Include the search results of the file search tool call.
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    /// Include image urls from the input message.
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    /// Include logprobs with assistant messages.
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
    /// Include an encrypted version of reasoning tokens in reasoning item outputs.
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

/// Query parameters for listing conversation items.
#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "ListConversationItemsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListConversationItemsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    
    /// The order to return the input items in. Default is `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListOrder>,
    
    /// An item ID to list items after, used in pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    
    /// Specify additional output data to include in the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<IncludeParam>>,
}

/// The order to return items in.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListOrder {
    /// Return items in ascending order.
    Asc,
    /// Return items in descending order.
    Desc,
}

