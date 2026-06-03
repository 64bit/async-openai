use crate::{error::OpenAIError, types::admin::invites::ProjectMembership};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual user in a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUser {
    /// The object type, which is always `organization.project.user`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The name of the user
    pub name: Option<String>,
    /// The email address of the user
    pub email: Option<String>,
    /// `owner` or `member`
    pub role: ProjectMembership,
    /// The Unix timestamp (in seconds) of when the project was added.
    pub added_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserListResponse {
    pub object: String,
    pub data: Vec<ProjectUser>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: String,
}

/// The project user create request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "ProjectUserCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUserCreateRequest {
    /// The ID of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Email of the user to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// `owner` or `member`
    pub role: ProjectMembership,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "ProjectUserUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUserUpdateRequest {
    /// `owner` or `member`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ProjectMembership>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserDeleteResponse {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}
