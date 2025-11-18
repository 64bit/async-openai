use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing videos.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListVideosOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing videos.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListVideosQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListVideosQuery {
    /// Number of items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order of results by timestamp. Use `asc` for ascending order or `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVideosOrder>,
    /// Identifier for the last item from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}
