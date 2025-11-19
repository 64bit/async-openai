use crate::types::admin::roles::Role;
use crate::types::OpenAIError;
use crate::types::OrganizationRole;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Query parameters for listing users.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListUsersQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListUsersQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Filter by the email address of users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

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
