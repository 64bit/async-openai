use serde::{Deserialize, Serialize};

/// A list of files attached to a `message`.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct MessageFileObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.message.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message file was created.
    pub created_at: i32,

    /// The ID of the [message](https://platform.openai.com/docs/api-reference/messages) that the [File](https://platform.openai.com/docs/api-reference/files) is attached to.
    pub message_id: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ListMessageFilesResponse {
    pub object: String,
    pub data: Vec<MessageFileObject>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
