use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::InputSource;

#[cfg_attr(not(feature = "wasm"), derive(Default))]
#[derive(Debug, Clone, PartialEq)]
pub struct FileInput {
    pub source: InputSource,
}

#[cfg_attr(not(feature = "wasm"), derive(Default, Builder))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(feature = "wasm"), builder(name = "CreateFileRequestArgs"))]
#[cfg_attr(not(feature = "wasm"), builder(pattern = "mutable"))]
#[cfg_attr(not(feature = "wasm"), builder(setter(into, strip_option), default))]
#[cfg_attr(not(feature = "wasm"), builder(derive(Debug)))]
#[cfg_attr(not(feature = "wasm"), builder(build_fn(error = "OpenAIError")))]
pub struct CreateFileRequest {
    /// The file object to be uploaded.
    ///
    /// If the `purpose` is set to "fine-tune", the file will be used for fine-tuning.
    pub file: FileInput,

    /// The intended purpose of the uploaded file.
    ///
    /// Use "fine-tune" for [fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning).
    /// This allows us to validate the format of the uploaded file is correct for fine-tuning.
    pub purpose: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFilesResponse {
    pub object: String,
    pub data: Vec<OpenAIFile>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum OpenAIFilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
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
    pub created_at: u32,
    /// The name of the file.
    pub filename: String,
    /// The intended purpose of the file. Supported values are `fine-tune`, `fine-tune-results`, `assistants`, and `assistants_output`.
    pub purpose: OpenAIFilePurpose,
    /// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
    #[deprecated]
    pub status: Option<String>,
    /// Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`.
    #[deprecated]
    pub status_details: Option<String>, // nullable: true
}
