use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing evals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListEvalsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Order by field for listing evals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListEvalsOrderBy {
    /// Order by creation time
    CreatedAt,
    /// Order by last updated time
    UpdatedAt,
}

/// Query parameters for listing evals.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListEvalsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListEvalsQuery {
    /// Identifier for the last eval from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of evals to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for evals by timestamp. Use `asc` for ascending order or `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListEvalsOrder>,
    /// Evals can be ordered by creation time or last updated time. Use `created_at` for creation time or `updated_at` for last updated time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListEvalsOrderBy>,
}

/// Sort order for getting eval runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GetEvalRunsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Status filter for eval runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GetEvalRunsStatus {
    /// Queued status
    Queued,
    /// In progress status
    InProgress,
    /// Completed status
    Completed,
    /// Canceled status
    Canceled,
    /// Failed status
    Failed,
}

/// Query parameters for getting eval runs.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "GetEvalRunsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct GetEvalRunsQuery {
    /// Identifier for the last run from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of runs to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for runs by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<GetEvalRunsOrder>,
    /// Filter runs by status. One of `queued` | `in_progress` | `failed` | `completed` | `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetEvalRunsStatus>,
}

/// Sort order for getting eval run output items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GetEvalRunOutputItemsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Status filter for eval run output items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GetEvalRunOutputItemsStatus {
    /// Failed status
    Fail,
    /// Pass status
    Pass,
}

/// Query parameters for getting eval run output items.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "GetEvalRunOutputItemsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct GetEvalRunOutputItemsQuery {
    /// Identifier for the last output item from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of output items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Filter output items by status. Use `failed` to filter by failed output items or `pass` to filter by passed output items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetEvalRunOutputItemsStatus>,
    /// Sort order for output items by timestamp. Use `asc` for ascending order or `desc` for descending order. Defaults to `asc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<GetEvalRunOutputItemsOrder>,
}
