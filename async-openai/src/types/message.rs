use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
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
    /// The entity that produced the message. One of `user` or `assistant`.
    pub role: MessageRole,

    /// The content of the message in array of text and/or images.
    pub content: Vec<MessageContent>,

    /// If applicable, the ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) that authored this message.
    pub assistant_id: Option<String>,

    /// If applicable, the ID of the [run](https://platform.openai.com/docs/api-reference/runs) associated with the authoring of this message.
    pub run_id: Option<String>,

    /// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs that the assistant should use. Useful for tools like retrieval and code_interpreter that can access files. A maximum of 10 files can be attached to a message.
    pub file_ids: Vec<String>,

    pub metadata: Option<HashMap<String, serde_json::Value>>,
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
    /// The role of the entity that is creating the message. Currently only `user` is supported.
    #[builder(default = "\"user\".into()")]
    pub role: String,
    /// The content of the message.
    pub content: String,
    /// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs that the message should use. There can be a maximum of 10 files attached to a message. Useful for tools like `retrieval` and `code_interpreter` that can access and use files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

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
