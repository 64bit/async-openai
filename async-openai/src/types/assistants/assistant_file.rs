use serde::{Deserialize, Serialize};

/// A list of [Files](https://platform.openai.com/docs/api-reference/files) attached to an `assistant`.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct AssistantFileObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `assistant.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant file was created.
    pub created_at: i32,

    ///  The assistant ID that the file is attached to.
    pub assistant_id: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct CreateAssistantFileRequest {
    /// A [File](https://platform.openai.com/docs/api-reference/files) ID (with `purpose="assistants"`) that the assistant should use. Useful for tools like `retrieval` and `code_interpreter` that can access files.
    pub file_id: String,
}

/// Deletes the association between the assistant and the file, but does not delete the [File](https://platform.openai.com/docs/api-reference/files) object itself.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct DeleteAssistantFileResponse {
    pub id: String,
    pub deleted: bool,
    pub object: String,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ListAssistantFilesResponse {
    pub object: String,
    pub data: Vec<AssistantFileObject>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
