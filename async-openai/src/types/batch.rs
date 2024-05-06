use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "BatchRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct BatchRequest {
    /// The ID of an uploaded file that contains requests for the new batch.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.
    ///
    /// Your input file must be formatted as a [JSONL file](https://platform.openai.com/docs/api-reference/batch/requestInput), and must be uploaded with the purpose `batch`.
    pub input_file_id: String,

    /// The endpoint to be used for all requests in the batch. Currently `/v1/chat/completions` and `/v1/embeddings` are supported.
    pub endpoint: BatchEndpoint,

    /// The time frame within which the batch should be processed. Currently only `24h` is supported.
    pub completion_window: BatchCompletionWindow,

    /// Optional custom metadata for the batch.
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum BatchEndpoint {
    #[default]
    #[serde(rename = "/v1/chat/completions")]
    V1ChatCompletions,
    #[serde(rename = "/v1/embeddings")]
    V1Embeddings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum BatchCompletionWindow {
    #[default]
    #[serde(rename = "24h")]
    W24H,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Batch {
    pub id: String,
    /// The object type, which is always `batch`.
    pub object: String,
    /// The OpenAI API endpoint used by the batch.
    pub endpoint: String,
    pub errors: Option<BatchErrors>,
    /// The ID of the input file for the batch.
    pub input_file_id: String,
    /// The time frame within which the batch should be processed.
    pub completion_window: String,
    /// The current status of the batch.
    pub status: BatchStatus,
    /// The ID of the file containing the outputs of successfully executed requests.
    pub output_file_id: Option<String>,
    /// The ID of the file containing the outputs of requests with errors.
    pub error_file_id: Option<String>,
    /// The Unix timestamp (in seconds) for when the batch was created.
    pub created_at: u32,
    /// The Unix timestamp (in seconds) for when the batch started processing.
    pub in_progress_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch will expire.
    pub expires_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    pub finalizing_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch was completed.
    pub completed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch failed.
    pub failed_at: Option<u32>,
    /// he Unix timestamp (in seconds) for when the batch expired.
    pub expired_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    pub cancelling_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    pub cancelled_at: Option<u32>,
    /// The request counts for different statuses within the batch.
    pub request_counts: Option<BatchRequestCounts>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchErrors {
    /// The object type, which is always `list`.
    pub object: String,
    pub data: Vec<BatchError>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchError {
    /// An error code identifying the error type.
    pub code: String,
    /// A human-readable message providing more details about the error.
    pub message: String,
    /// The name of the parameter that caused the error, if applicable.
    pub param: Option<String>,
    /// The line number of the input file where the error occurred, if applicable.
    pub line: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    Validating,
    Failed,
    InProgress,
    Finalizing,
    Completed,
    Expired,
    Cancelling,
    Cancelled,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchRequestCounts {
    /// Total number of requests in the batch.
    pub total: u32,
    /// Number of requests that have been completed successfully.
    pub completed: u32,
    /// Number of requests that have failed.
    pub failed: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListBatchesResponse {
    pub data: Vec<Batch>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
    pub object: String,
}
