use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Sort order for listing vector stores.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListVectorStoresOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing vector stores.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListVectorStoresQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListVectorStoresQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVectorStoresOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Sort order for listing files in vector store batch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListFilesInVectorStoreBatchOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Filter by file status for files in vector store batch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListFilesInVectorStoreBatchFilter {
    /// In progress status
    InProgress,
    /// Completed status
    Completed,
    /// Failed status
    Failed,
    /// Cancelled status
    Cancelled,
}

/// Query parameters for listing files in vector store batch.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListFilesInVectorStoreBatchQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListFilesInVectorStoreBatchQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListFilesInVectorStoreBatchOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter by file status. One of `in_progress`, `completed`, `failed`, `cancelled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListFilesInVectorStoreBatchFilter>,
}

/// Sort order for listing vector store files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListVectorStoreFilesOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Filter by file status for vector store files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListVectorStoreFilesFilter {
    /// In progress status
    InProgress,
    /// Completed status
    Completed,
    /// Failed status
    Failed,
    /// Cancelled status
    Cancelled,
}

/// Query parameters for listing vector store files.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListVectorStoreFilesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListVectorStoreFilesQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVectorStoreFilesOrder>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter by file status. One of `in_progress`, `completed`, `failed`, `cancelled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListVectorStoreFilesFilter>,
}
