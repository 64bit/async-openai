use crate::types::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual user in a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectUser {
    /// The object type, which is always `organization.project.user`
    object: String,
    /// The identifier, which can be referenced in API endpoints
    id: String,
    /// The name of the project
    name: String,
    /// The email address of the user
    email: String,
    /// `owner` or `member`
    role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the project was added.
    added_at: u32,
}

/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectUserRole {
    Owner,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectUserListResponse {
    object: String,
    data: Vec<ProjectUser>,
    first_id: String,
    last_id: String,
    has_more: String,
}

/// The project user create request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectUserCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
#[serde(rename_all = "snake_case")]
pub struct ProjectUserCreateRequest {
    /// The ID of the user.
    user_id: String,
    /// `owner` or `member`
    role: ProjectUserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectUserUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
#[serde(rename_all = "snake_case")]
pub struct ProjectUserUpdateRequest {
    /// `owner` or `member`
    role: ProjectUserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectUserDeleteResponse {
    object: String,
    id: String,
    deleted: bool,
}
