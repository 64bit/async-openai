use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use crate::types::InputSource;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContainerResource {
    /// Unique identifier for the container.
    pub id: String,
    /// The type of this object.
    pub object: String,
    /// Name of the container.
    pub name: String,
    /// Unix timestamp (in seconds) when the container was created.
    pub created_at: u64,
    /// Status of the container (e.g., active, deleted).
    pub status: String,
    /// The container will expire after this time period. The anchor is the reference point for the expiration.
    /// The minutes is the number of minutes after the anchor before the container expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ContainerExpiresAfter>,
    /// Unix timestamp (in seconds) when the container was last active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<u64>,
}

/// Expiration policy for containers.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContainerExpiresAfter {
    /// Time anchor for the expiration time. Currently only 'last_active_at' is supported.
    pub anchor: ContainerExpiresAfterAnchor,
    pub minutes: u32,
}

/// Anchor for container expiration.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContainerExpiresAfterAnchor {
    LastActiveAt,
}

/// Request to create a container.
/// openapi spec type: CreateContainerBody
#[derive(Debug, Default, Clone, Builder, PartialEq, Serialize)]
#[builder(name = "CreateContainerRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateContainerRequest {
    /// Name of the container to create.
    pub name: String,
    /// IDs of files to copy to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Container expiration time in minutes relative to the 'anchor' time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ContainerExpiresAfter>,
}

/// Response when listing containers.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContainerListResource {
    /// The type of object returned, must be 'list'.
    pub object: String,
    /// A list of containers.
    pub data: Vec<ContainerResource>,
    /// The ID of the first container in the list.
    pub first_id: Option<String>,
    /// The ID of the last container in the list.
    pub last_id: Option<String>,
    /// Whether there are more containers available.
    pub has_more: bool,
}

/// Response when deleting a container.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteContainerResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

// Container File types

/// The container file object represents a file in a container.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContainerFileResource {
    /// Unique identifier for the file.
    pub id: String,
    /// The type of this object (`container.file`).
    pub object: String,
    /// The container this file belongs to.
    pub container_id: String,
    /// Unix timestamp (in seconds) when the file was created.
    pub created_at: u64,
    /// Size of the file in bytes.
    pub bytes: u32,
    /// Path of the file in the container.
    pub path: String,
    /// Source of the file (e.g., `user`, `assistant`).
    pub source: String,
}

/// Request to create a container file.
/// openapi spec type: CreateContainerFileBody
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreateContainerFileRequest {
    /// The File object (not file name) to be uploaded.
    pub file: Option<InputSource>,
    /// Name of the file to create.
    pub file_id: Option<String>,
}

/// Response when listing container files.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContainerFileListResource {
    /// The type of object returned, must be 'list'.
    pub object: String,
    /// A list of container files.
    pub data: Vec<ContainerFileResource>,
    /// The ID of the first file in the list.
    pub first_id: Option<String>,
    /// The ID of the last file in the list.
    pub last_id: Option<String>,
    /// Whether there are more files available.
    pub has_more: bool,
}

/// Response when deleting a container file.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteContainerFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
