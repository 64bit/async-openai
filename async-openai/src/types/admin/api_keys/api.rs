use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing admin API keys.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListAdminApiKeysOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing admin API keys.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListAdminApiKeysQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListAdminApiKeysQuery {
    /// Return keys with IDs that come after this ID in the pagination order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Order results by creation time, ascending or descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdminApiKeysOrder>,
    /// Maximum number of keys to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
