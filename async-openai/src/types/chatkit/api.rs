use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing ChatKit threads.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListChatKitThreadsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing ChatKit threads.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListChatKitThreadsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListChatKitThreadsQuery {
    /// Maximum number of thread items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for results by creation time. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListChatKitThreadsOrder>,
    /// List items created after this thread item ID. Defaults to null for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// List items created before this thread item ID. Defaults to null for the newest results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter threads that belong to this user identifier. Defaults to null to return all users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Sort order for listing ChatKit thread items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListChatKitThreadItemsOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing ChatKit thread items.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListChatKitThreadItemsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListChatKitThreadItemsQuery {
    /// Maximum number of thread items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order for results by creation time. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListChatKitThreadItemsOrder>,
    /// List items created after this thread item ID. Defaults to null for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// List items created before this thread item ID. Defaults to null for the newest results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}
