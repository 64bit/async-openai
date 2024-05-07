use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::{AssistantToolsCode, AssistantToolsFileSearch};

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    #[default]
    User,
    Assistant,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    InProgress,
    Incomplete,
    Completed,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageIncompleteDetailsType {
    ContentFilter,
    MaxTokens,
    RunCancelled,
    RunExpired,
    RunFailed,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageIncompleteDetails {
    /// The reason the message is incomplete.
    pub reason: MessageIncompleteDetailsType,
}

///  Represents a message within a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `thread.message`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the message was created.
    pub created_at: i32,
    /// The [thread](https://platform.openai.com/docs/api-reference/threads) ID that this message belongs to.
    pub thread_id: String,

    /// The status of the message, which can be either `in_progress`, `incomplete`, or `completed`.
    pub status: Option<MessageStatus>,

    /// On an incomplete message, details about why the message is incomplete.
    pub incomplete_details: Option<MessageIncompleteDetails>,

    /// The Unix timestamp (in seconds) for when the message was completed.
    pub completed_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the message was marked as incomplete.
    pub incomplete_at: Option<u32>,

    /// The entity that produced the message. One of `user` or `assistant`.
    pub role: MessageRole,

    /// The content of the message in array of text and/or images.
    pub content: Vec<MessageContent>,

    /// If applicable, the ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) that authored this message.
    pub assistant_id: Option<String>,

    /// The ID of the [run](https://platform.openai.com/docs/api-reference/runs) associated with the creation of this message. Value is `null` when messages are created manually using the create message or create thread endpoints.
    pub run_id: Option<String>,

    /// A list of files attached to the message, and the tools they were added to.
    pub attachments: Option<Vec<MessageAttachment>>,

    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageAttachment {
    /// The ID of the file to attach to the message.
    pub file_id: String,
    /// The tools to add this file to.
    pub tools: Vec<MessageAttachmentTool>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageAttachmentTool {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Text(MessageContentTextObject),
    ImageFile(MessageContentImageFileObject),
}

/// The text content that is part of a message.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageContentTextObject {
    /// Always `text`.
    pub r#type: String,
    pub text: TextData,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct TextData {
    /// The data that makes up the text.
    pub value: String,
    pub annotations: Vec<MessageContentTextAnnotations>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageContentTextAnnotations {
    /// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "retrieval" tool to search files.
    FileCitation(MessageContentTextAnnotationsFileCitationObject),
    /// A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file.
    FilePath(MessageContentTextAnnotationsFilePathObject),
}

/// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageContentTextAnnotationsFileCitationObject {
    /// Always `file_citation`.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: String,
    pub file_citation: FileCitation,
    pub start_index: u32,
    pub end_index: u32,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct FileCitation {
    /// The ID of the specific File the citation is from.
    pub file_id: String,
    /// The specific quote in the file.
    pub quote: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageContentTextAnnotationsFilePathObject {
    /// Always `file_path`.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: String,
    pub file_path: FilePath,
    pub start_index: u32,
    pub end_index: u32,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct FilePath {
    /// The ID of the file that was generated.
    pub file_id: String,
}

/// References an image [File](https://platform.openai.com/docs/api-reference/files) in the content of a message.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageContentImageFileObject {
    /// Always `image_file`.
    pub r#type: String,
    pub image_file: ImageFile,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ImageFile {
    /// The [File](https://platform.openai.com/docs/api-reference/files) ID of the image in the message content.
    pub file_id: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "CreateMessageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateMessageRequest {
    /// The role of the entity that is creating the message. Allowed values include:
    /// - `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.
    /// - `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.
    pub role: MessageRole,
    /// The content of the message.
    pub content: String,

    /// A list of files attached to the message, and the tools they should be added to.
    pub attachments: Option<Vec<MessageAttachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ModifyMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct DeleteMessageResponse {
    pub id: String,
    pub deleted: bool,
    pub object: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ListMessagesResponse {
    pub object: String,
    pub data: Vec<MessageObject>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// Represents a message delta i.e. any changed fields on a message during streaming.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaObject {
    /// The identifier of the message, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `thread.message.delta`.
    pub object: String,
    /// The delta containing the fields that have changed on the Message.
    pub delta: MessageDelta,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDelta {
    /// The entity that produced the message. One of `user` or `assistant`.
    pub role: Option<MessageRole>,
    ///  The content of the message in array of text and/or images.
    pub content: Option<Vec<MessageDeltaContent>>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageDeltaContent {
    ImageFile(MessageDeltaContentImageFileObject),
    Text(MessageDeltaContentTextObject),
}

/// The text content that is part of a message.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaContentTextObject {
    /// The index of the content part in the message.
    pub index: u32,
    ///  Always `text`.
    pub r#type: String,

    pub text: MessageDeltaContentText,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaContentText {
    /// The data that makes up the text.
    pub value: String,
    pub annotations: Vec<MessageDeltaContentTextAnnotations>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageDeltaContentTextAnnotations {
    FileCitation(MessageDeltaContentTextAnnotationsFileCitationObject),
    FilePath(MessageDeltaContentTextAnnotationsFilePathObject),
}

/// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    /// The index of the annotation in the text content part.
    pub index: u32,
    /// Always `file_citation`.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: Option<String>,
    pub file_citation: Option<FileCitation>,
    pub start_index: Option<u32>,
    pub end_index: Option<u32>,
}

/// A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    /// The index of the annotation in the text content part.
    pub index: u32,
    /// Always `file_path`.
    pub r#type: String,
    /// The text in the message content that needs to be replaced.
    pub text: Option<String>,
    pub file_path: Option<FilePath>,
    pub start_index: Option<u32>,
    pub end_index: Option<u32>,
}

/// References an image [File](https://platform.openai.com/docs/api-reference/files) in the content of a message.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageDeltaContentImageFileObject {
    /// The index of the content part in the message.
    pub index: u32,
    ///  Always `image_file`.
    pub r#type: String,

    pub image_file: Option<ImageFile>,
}
