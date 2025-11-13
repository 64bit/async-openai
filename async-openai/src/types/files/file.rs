use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use crate::types::InputSource;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileInput {
    pub source: InputSource,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum FilePurpose {
    Assistants,
    Batch,
    #[default]
    FineTune,
    Vision,
    UserData,
    Evals,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum FileExpirationAfterAnchor {
    #[default]
    #[serde(rename = "created_at")]
    CreatedAt,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct FileExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`.
    pub anchor: FileExpirationAfterAnchor,

    /// The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days).
    pub seconds: u32,
}

#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateFileRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFileRequest {
    /// The File object (not file name) to be uploaded.
    pub file: FileInput,

    /// The intended purpose of the uploaded file.
    ///
    /// Use "assistants" for [Assistants](https://platform.openai.com/docs/api-reference/assistants) and [Message](https://platform.openai.com/docs/api-reference/messages) files, "vision" for Assistants image file inputs, "batch" for [Batch API](https://platform.openai.com/docs/guides/batch), "fine-tune" for [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning), "user_data" for flexible file type for any purpose, and "evals" for eval data sets.
    pub purpose: FilePurpose,

    /// The expiration policy for a file. By default, files with `purpose=batch` expire after 30 days and all other files are persisted until they are manually deleted.
    pub expires_after: Option<FileExpirationAfter>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFilesResponse {
    pub object: String,
    pub data: Vec<OpenAIFile>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum OpenAIFilePurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "batch_output")]
    BatchOutput,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "user_data")]
    UserData,
}

/// The `File` object represents a document that has been uploaded to OpenAI.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OpenAIFile {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The object type, which is always "file".
    pub object: String,
    /// The size of the file in bytes.
    pub bytes: u32,
    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) for when the file will expire.
    pub expires_at: Option<u64>,
    /// The name of the file.
    pub filename: String,
    /// The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results`, `vision`, and `user_data`.
    pub purpose: OpenAIFilePurpose,
    /// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
    #[deprecated]
    pub status: Option<String>,
    /// Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`.
    #[deprecated]
    pub status_details: Option<String>, // nullable: true
}
