use serde::{Deserialize, Serialize};

/// An existing item reference or a new message or function call item for a Realtime response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeResponseInputItem {
    Message(RealtimeResponseInputMessage),
    FunctionCall(RealtimeResponseInputFunctionCall),
    FunctionCallOutput(RealtimeResponseInputFunctionCallOutput),
    ItemReference(RealtimeResponseItemReference),
}

/// Reference an existing item in this Realtime session as response.create input.
/// Item references are only accepted in the response input array, not as new
/// conversation items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseItemReference {
    /// The ID of an existing item in the same Realtime session.
    pub id: String,
}

/// The result of a function call supplied as input to a Realtime response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputFunctionCallOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputFunctionCallOutputStatus>,
    pub call_id: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseInputFunctionCallOutputStatus {
    #[default]
    Completed,
    Incomplete,
}

/// A function call supplied as input to a Realtime response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputFunctionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputFunctionCallStatus>,
    pub call_id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseInputFunctionCallStatus {
    #[default]
    Completed,
    Incomplete,
}

/// A message supplied as input to a Realtime response. The role determines the allowed content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum RealtimeResponseInputMessage {
    System(RealtimeResponseInputMessageSystem),
    Developer(RealtimeResponseInputMessageDeveloper),
    User(RealtimeResponseInputMessageUser),
    Assistant(RealtimeResponseInputMessageAssistant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputMessageAssistant {
    /// An optional client ID. The value `root` is reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional input status. Omission or null defaults to completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputMessageStatus>,
    pub content: Vec<RealtimeResponseInputMessageAssistantContentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealtimeResponseInputMessageAssistantContentItem {
    pub r#type: RealtimeResponseInputMessageAssistantContentItemType,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseInputMessageAssistantContentItemType {
    OutputText,
}

/// Optional input status. Omission or null defaults to completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseInputMessageStatus {
    Completed,
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputMessageUser {
    /// An optional client ID. The value `root` is reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional input status. Omission or null defaults to completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputMessageStatus>,
    pub content: Vec<RealtimeResponseInputMessageUserContentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeResponseInputMessageUserContentItem {
    InputText(RealtimeResponseInputText),
    InputAudio(RealtimeResponseInputAudio),
    InputImage(RealtimeResponseInputImage),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputImage {
    /// A base64 data URL with an image MIME type; use PNG or JPEG.
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<RealtimeResponseInputImageDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseInputImageDetail {
    Low,
    High,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputAudio {
    /// Base64 audio bytes in the session input audio format.
    pub audio: String,
    /// Optional text to associate with the input audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputText {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputMessageDeveloper {
    /// An optional client ID. The value `root` is reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional input status. Omission or null defaults to completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputMessageStatus>,
    pub content: Vec<RealtimeResponseInputTextPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RealtimeResponseInputMessageSystem {
    /// An optional client ID. The value `root` is reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional input status. Omission or null defaults to completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RealtimeResponseInputMessageStatus>,
    pub content: Vec<RealtimeResponseInputTextPart>,
}

/// Input-text content for system and developer messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeResponseInputTextPart {
    InputText(RealtimeResponseInputText),
}
