use crate::types::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Query parameters for listing project users.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectUsersQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectUsersQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Represents an individual user in a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUser {
    /// The object type, which is always `organization.project.user`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The name of the project
    pub name: String,
    /// The email address of the user
    pub email: String,
    /// `owner` or `member`
    pub role: ProjectUserRole,
    /// The Unix timestamp (in seconds) of when the project was added.
    pub added_at: u64,
}

/// `owner` or `member`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectUserRole {
    Owner,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserListResponse {
    pub object: String,
    pub data: Vec<ProjectUser>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: String,
}

/// The project user create request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectUserCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUserCreateRequest {
    /// The ID of the user.
    pub user_id: String,
    /// `owner` or `member`
    pub role: ProjectUserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectUserUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUserUpdateRequest {
    /// `owner` or `member`
    pub role: ProjectUserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectUserDeleteResponse {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}
