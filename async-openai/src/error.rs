//! Errors originating from API calls, parsing responses, and reading-or-writing to the file system.

use serde::{Deserialize, Serialize};

/// Raw API error string from providers with custom error formats
///
/// Only available with the `string-errors` feature.
#[cfg(feature = "string-errors")]
#[derive(Debug, Clone)]
pub struct RawApiError(pub String);

#[cfg(feature = "string-errors")]
impl std::fmt::Display for RawApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "string-errors")]
impl RawApiError {
    /// Parse the raw error string into a custom type
    pub fn parse<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// OpenAI returns error object with details of API call failure
    #[cfg(not(feature = "string-errors"))]
    #[error("{0}")]
    ApiError(ApiError),
    /// Some OpenAI compatible services return error messages in diverge in error formats.
    /// This feature leaves deserialization to the user, not even assuming json.
    #[cfg(feature = "string-errors")]
    #[error("{0}")]
    ApiError(RawApiError),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("failed to deserialize api response: error:{0} content:{1}")]
    JSONDeserialize(serde_json::Error, String),
    /// Error on the client side when saving file to file system
    #[error("failed to save file: {0}")]
    FileSaveError(String),
    /// Error on the client side when reading file from file system
    #[error("failed to read file: {0}")]
    FileReadError(String),
    /// Error on SSE streaming
    #[error("stream failed: {0}")]
    StreamError(StreamError),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    /// Underlying error from reqwest_eventsource library when reading the stream
    #[error("{0}")]
    ReqwestEventSource(#[from] reqwest_eventsource::Error),
    /// Error when a stream event does not match one of the expected values
    #[error("Unknown event: {0:#?}")]
    UnknownEvent(eventsource_stream::Event),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct WrappedError {
    pub error: ApiError,
}

pub(crate) fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> OpenAIError {
    let json_content = String::from_utf8_lossy(bytes);
    tracing::error!("failed deserialization of: {}", json_content);

    OpenAIError::JSONDeserialize(e, json_content.to_string())
}
