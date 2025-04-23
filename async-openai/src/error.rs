//! Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// OpenAI returns error object with details of API call failure
    #[error("{0}")]
    ApiError(ApiError),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("Failed to deserialize API response: {0}\nResponse body: {1}")]
    JSONDeserialize(serde_json::Error, String),
    /// Error on the client side when saving file to file system
    #[error("failed to save file: {0}")]
    FileSaveError(String),
    /// Error on the client side when reading file from file system
    #[error("failed to read file: {0}")]
    FileReadError(String),
    /// Error on SSE streaming
    #[error("stream failed: {0}")]
    StreamError(String),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize, Clone)]
pub struct ApiError {
    pub message: String,
    pub r#type: Option<String>,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl std::fmt::Display for ApiError {
    /// If all fields are available, `ApiError` is formatted as:
    /// `{type}: {message} (param: {param}) (code: {code})`
    /// Otherwise, missing fields will be ignored.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if let Some(r#type) = &self.r#type {
            parts.push(format!("{}:", r#type));
        }

        parts.push(self.message.clone());

        if let Some(param) = &self.param {
            parts.push(format!("(param: {param})"));
        }

        if let Some(code) = &self.code {
            parts.push(format!("(code: {code})"));
        }

        write!(f, "{}", parts.join(" "))
    }
}

/// Wrapper to deserialize the error object nested in "error" JSON key
#[derive(Debug, Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}

/// Flat error object returned by vLLM API (not wrapped in "error")
#[derive(Debug, Deserialize)]
struct FlatApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<u16>,
}

/// Attempts to parse the response body as an OpenAI error before falling back to
/// a generic deserialization error with the full response body included for debugging.
pub(crate) fn map_deserialization_error(err: serde_json::Error, bytes: &[u8]) -> OpenAIError {
    let response_text = String::from_utf8_lossy(bytes).to_string();

    // Try to parse as an OpenAI API error first (wrapped)
    if let Ok(wrapped_error) = serde_json::from_slice::<WrappedError>(bytes) {
        return OpenAIError::ApiError(wrapped_error.error);
    }

    // Try to parse as a flat error object
    if let Ok(flat_error) = serde_json::from_slice::<FlatApiError>(bytes) {
        let api_error = ApiError {
            message: flat_error.message,
            r#type: Some(flat_error.r#type),
            param: flat_error.param,
            code: flat_error.code.map(|c| c.to_string()),
        };
        return OpenAIError::ApiError(api_error);
    }

    // Log the full error detail with original response
    tracing::error!(
        error = %err,
        "Failed to deserialize response: {}",
        response_text
    );

    // Include the response body in the error for better debugging
    OpenAIError::JSONDeserialize(err, response_text)
}
