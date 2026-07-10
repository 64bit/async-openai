use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Permission state for a hosted tool on a project.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct HostedToolPermission {
    pub enabled: bool,
}

pub type HostedToolPermissionUpdate = HostedToolPermission;

/// Hosted tool permissions for a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProjectHostedToolPermissions {
    pub file_search: HostedToolPermission,
    pub web_search: HostedToolPermission,
    pub image_generation: HostedToolPermission,
    pub mcp: HostedToolPermission,
    pub code_interpreter: HostedToolPermission,
}

/// Hosted tool permission changes for a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Builder)]
#[builder(
    name = "ProjectHostedToolPermissionsUpdateRequestArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct ProjectHostedToolPermissionsUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<HostedToolPermissionUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<HostedToolPermissionUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_generation: Option<HostedToolPermissionUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<HostedToolPermissionUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_interpreter: Option<HostedToolPermissionUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectModelPermissionMode {
    AllowList,
    DenyList,
}

/// Model allowlist or denylist policy for a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProjectModelPermissions {
    pub object: String,
    pub mode: ProjectModelPermissionMode,
    pub model_ids: Vec<String>,
}

/// Parameters for updating a project's model permissions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "ProjectModelPermissionsUpdateRequestArgs",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct ProjectModelPermissionsUpdateRequest {
    pub mode: ProjectModelPermissionMode,
    pub model_ids: Vec<String>,
}

/// Confirmation returned after deleting project model permissions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProjectModelPermissionsDeleteResponse {
    pub object: String,
    pub deleted: bool,
}
