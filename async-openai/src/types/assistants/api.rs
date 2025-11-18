use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListMessagesOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing messages.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListMessagesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListMessagesQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMessagesOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter messages by the run ID that generated them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// Sort order for listing runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListRunsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing runs.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListRunsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListRunsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListRunsOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Sort order for listing run steps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListRunStepsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing run steps.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListRunStepsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListRunStepsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListRunStepsOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// A list of additional fields to include in the response. Currently the only supported value is `step_details.tool_calls[*].file_search.results[*].content` to fetch the file search result content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// Query parameters for retrieving a run step.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "GetRunStepQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct GetRunStepQuery {
    /// A list of additional fields to include in the response. Currently the only supported value is `step_details.tool_calls[*].file_search.results[*].content` to fetch the file search result content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}
