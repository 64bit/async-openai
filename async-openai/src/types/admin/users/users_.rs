use crate::error::OpenAIError;
use crate::types::admin::roles::OrganizationRole;
use crate::types::admin::roles::Role;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual `user` within an organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    /// The object type, which is always `organization.user`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The name of the user
    pub name: String,
    /// The email address of the user
    pub email: String,
    /// `owner` or `reader`
    pub role: OrganizationRole,
    /// The Unix timestamp (in seconds) of when the users was added.
    pub added_at: u64,
}

/// A list of `User` objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserListResponse {
    pub object: String,
    pub data: Vec<User>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "UserRoleUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UserRoleUpdateRequest {
    /// `owner` or `reader`
    pub role: OrganizationRole,
}

/// Confirmation of the deleted user
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDeleteResponse {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

/// Role assignment linking a user to a role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRoleAssignment {
    /// The object type, which is always `user.role`.
    pub object: String,
    /// The user.
    pub user: User,
    /// The role.
    pub role: Role,
}

/// Paginated list of role assignments for a user.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRoleAssignmentListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Role assignments returned in the current page.
    pub data: Vec<UserRoleAssignment>,
    /// Whether additional assignments are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no more assignments.
    pub next: Option<String>,
}

/// Request payload for assigning a role to a user.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "AssignUserRoleRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct PublicAssignOrganizationUserRoleBody {
    /// Identifier of the role to assign.
    pub role_id: String,
}
