use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateVectorStoreRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateVectorStoreRequest {
    /// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// The name of the vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The expiration policy for a vector store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<VectorStoreExpirationAfter>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Vector store expiration policy
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct VectorStoreExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`.
    pub anchor: String,
    /// The number of days after the anchor time that the vector store will expire.
    pub days: u16, // min: 1, max: 365
}

/// A vector store is a collection of processed files can be used by the `file_search` tool.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct VectorStoreObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `vector_store`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the vector store was created.
    pub created_at: u32,
    /// The name of the vector store.
    pub name: String,
    /// The total number of bytes used by the files in the vector store.
    pub usage_bytes: u64,
    pub file_counts: VectorStoreFileCounts,
    /// The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use.
    pub status: VectorStoreStatus,
    pub expires_after: Option<VectorStoreExpirationAfter>,
    /// The Unix timestamp (in seconds) for when the vector store will expire.
    pub expires_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the vector store was last active.
    pub last_active_at: Option<u32>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreStatus {
    Expired,
    InProgress,
    Completed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct VectorStoreFileCounts {
    /// The number of files that are currently being processed.
    pub in_progress: u32,
    /// The number of files that have been successfully processed.
    pub completed: u32,
    /// The number of files that have failed to process.
    pub failed: u32,
    /// The number of files that were cancelled.
    pub cancelled: u32,
    /// The total number of files.
    pub total: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListVectorStoresResponse {
    pub object: String,
    pub data: Vec<VectorStoreObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteVectorStoreResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "UpdateVectorStoreRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateVectorStoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
