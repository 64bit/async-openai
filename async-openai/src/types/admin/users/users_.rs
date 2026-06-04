use crate::error::OpenAIError;
use crate::types::admin::roles::Role;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Nested user details for [`User`].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserNested {
    pub object: String,
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub enabled: Option<bool>,
    pub banned: Option<bool>,
    pub banned_at: Option<u64>,
}

/// A project entry inside [`UserProjects::data`].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserProjectEntry {
    pub id: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
}

/// Projects associated with the user, if included.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserProjects {
    pub object: String,
    pub data: Vec<UserProjectEntry>,
}

/// Represents an individual `user` within an organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    /// The object type, which is always `organization.user`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The name of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email address of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// `owner` or `reader`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The Unix timestamp (in seconds) of when the user was added.
    pub added_at: u64,
    /// Whether this is the organization's default user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The Unix timestamp (in seconds) of when the user was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<u64>,
    /// Nested user details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserNested>,
    /// Whether the user is a service account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_service_account: Option<bool>,
    /// Whether the user is an authorized purchaser for Scale Tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_scale_tier_authorized_purchaser: Option<bool>,
    /// Whether the user is managed through SCIM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_scim_managed: Option<bool>,
    /// The Unix timestamp (in seconds) of the user's last API key usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key_last_used_at: Option<u64>,
    /// The technical level metadata for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technical_level: Option<String>,
    /// The developer persona metadata for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub developer_persona: Option<String>,
    /// Projects associated with the user, if included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<UserProjects>,
}

/// A list of `User` objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserListResponse {
    pub object: String,
    pub data: Vec<User>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "UserRoleUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UserRoleUpdateRequest {
    /// `owner` or `reader`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Role ID to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// Technical level metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_level: Option<String>,
    /// Developer persona metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_persona: Option<String>,
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
