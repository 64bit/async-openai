use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents a project rate limit config.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectRateLimit {
    /// The object type, which is always `project.rate_limit`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The model this rate limit applies to.
    pub model: String,
    /// The maximum requests per minute.
    pub max_requests_per_1_minute: i64,
    /// The maximum tokens per minute.
    pub max_tokens_per_1_minute: i64,
    /// The maximum images per minute. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images_per_1_minute: Option<i64>,
    /// The maximum audio megabytes per minute. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_megabytes_per_1_minute: Option<i64>,
    /// The maximum requests per day. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_day: Option<i64>,
    /// The maximum batch input tokens per day. Only present for relevant models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_1_day_max_input_tokens: Option<i64>,
}

/// Represents the response object for listing project rate limits.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectRateLimitListResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of project rate limits.
    pub data: Vec<ProjectRateLimit>,
    /// The ID of the first project rate limit in the list.
    pub first_id: String,
    /// The ID of the last project rate limit in the list.
    pub last_id: String,
    /// Indicates if there are more project rate limits available.
    pub has_more: bool,
}

/// The project rate limit update request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "ProjectRateLimitUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectRateLimitUpdateRequest {
    /// The maximum requests per minute.
    pub max_requests_per_1_minute: Option<i64>,
    /// The maximum tokens per minute.
    pub max_tokens_per_1_minute: Option<i64>,
    /// The maximum images per minute. Only relevant for certain models.
    pub max_images_per_1_minute: Option<i64>,
    /// The maximum audio megabytes per minute. Only relevant for certain models.
    pub max_audio_megabytes_per_1_minute: Option<i64>,
    /// The maximum requests per day. Only relevant for certain models.
    pub max_requests_per_1_day: Option<i64>,
    /// The maximum batch input tokens per day. Only relevant for certain models.
    pub batch_1_day_max_input_tokens: Option<i64>,
}
