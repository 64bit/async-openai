use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::batches::ResponseUsage;
use crate::types::Metadata;

#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq, Deserialize)]
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
    /// Your input file must be formatted as a [JSONL file](https://platform.openai.com/docs/api-reference/batch/request-input), and must be uploaded with the purpose `batch`. The file can contain up to 50,000 requests, and can be up to 200 MB in size.
    pub input_file_id: String,

    /// The endpoint to be used for all requests in the batch. Currently `/v1/responses`,
    /// `/v1/chat/completions`, `/v1/embeddings`, `/v1/completions`, and `/v1/moderations` are
    /// supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000
    /// embedding inputs across all requests in the batch.
    pub endpoint: BatchEndpoint,

    /// The time frame within which the batch should be processed. Currently only `24h` is supported.
    pub completion_window: BatchCompletionWindow,

    /// Optional custom metadata for the batch.
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// The expiration policy for the output and/or error file that are generated for a batch.
    pub output_expires_after: Option<BatchFileExpirationAfter>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub enum BatchEndpoint {
    #[default]
    #[serde(rename = "/v1/responses")]
    V1Responses,
    #[serde(rename = "/v1/chat/completions")]
    V1ChatCompletions,
    #[serde(rename = "/v1/embeddings")]
    V1Embeddings,
    #[serde(rename = "/v1/completions")]
    V1Completions,
    #[serde(rename = "/v1/moderations")]
    V1Moderations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default, Deserialize)]
pub enum BatchCompletionWindow {
    #[default]
    #[serde(rename = "24h")]
    W24H,
}

/// File expiration policy
///
/// The expiration policy for the output and/or error file that are generated for a batch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchFileExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`. Note that the anchor is the file creation time, not the time the batch is created.
    pub anchor: BatchFileExpirationAnchor,
    /// The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days).
    pub seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchFileExpirationAnchor {
    CreatedAt,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Batch {
    pub id: String,
    /// The object type, which is always `batch`.
    pub object: String,
    /// The OpenAI API endpoint used by the batch.
    pub endpoint: String,
    /// Model ID used to process the batch, like `gpt-5-2025-08-07`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models) to browse and compare available models.
    pub model: Option<String>,
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
    pub created_at: u64,
    /// The Unix timestamp (in seconds) for when the batch started processing.
    pub in_progress_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch will expire.
    pub expires_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    pub finalizing_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch was completed.
    pub completed_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch failed.
    pub failed_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch expired.
    pub expired_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    pub cancelling_at: Option<u64>,
    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    pub cancelled_at: Option<u64>,
    /// The request counts for different statuses within the batch.
    pub request_counts: Option<BatchRequestCounts>,
    /// Represents token usage details including input tokens, output tokens, a breakdown of output tokens, and the total tokens used. Only populated on batches created after September 7, 2025.
    pub usage: Option<ResponseUsage>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<Metadata>,
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

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BatchRequestInputMethod {
    POST,
}

/// The per-line object of the batch input file
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchRequestInput {
    /// A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch.
    pub custom_id: String,
    /// The HTTP method to be used for the request. Currently only `POST` is supported.
    pub method: BatchRequestInputMethod,
    /// The OpenAI API relative URL to be used for the request. Currently `/v1/responses`,
    /// `/v1/chat/completions`, `/v1/embeddings`, `/v1/completions`, and `/v1/moderations` are supported.
    pub url: BatchEndpoint,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchRequestOutputResponse {
    /// The HTTP status code of the response
    pub status_code: u16,
    /// An unique identifier for the OpenAI API request. Please include this request ID when contacting support.
    pub request_id: String,
    /// The JSON body of the response
    pub body: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchRequestOutputError {
    /// A machine-readable error code.
    pub code: String,
    /// A human-readable error message.
    pub message: String,
}

/// The per-line object of the batch output and error files
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct BatchRequestOutput {
    pub id: String,
    /// A developer-provided per-request id that will be used to match outputs to inputs.
    pub custom_id: String,
    pub response: Option<BatchRequestOutputResponse>,
    ///  For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure.
    pub error: Option<BatchRequestOutputError>,
}
