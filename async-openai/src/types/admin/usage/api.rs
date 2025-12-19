use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Query parameters for organization usage endpoints.
#[derive(Debug, Clone, Serialize, Default, Builder)]
#[builder(name = "UsageQueryParamsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UsageQueryParams {
    /// Start time (Unix seconds) of the query time range, inclusive.
    pub start_time: u64,
    /// End time (Unix seconds) of the query time range, exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Width of each time bucket in response. Currently `1m`, `1h` and `1d` are supported, default to `1d`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_width: Option<UsageBucketWidth>,
    /// Return only usage for these projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Return only usage for these users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// Return only usage for these API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_ids: Option<Vec<String>>,
    /// Return only usage for these models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    /// If `true`, return batch jobs only. If `false`, return non-batch jobs only. By default, return both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
    /// Group the usage data by the specified fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<UsageGroupBy>>,
    /// Specifies the number of buckets to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. Corresponding to the `next_page` field from the previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

/// Width of each time bucket in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageBucketWidth {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "1d")]
    OneDay,
}

/// Fields to group usage data by.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageGroupBy {
    ProjectId,
    UserId,
    ApiKeyId,
    Model,
    Batch,
    ServiceTier,
}
