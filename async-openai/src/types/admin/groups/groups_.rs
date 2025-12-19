use crate::error::OpenAIError;
use crate::types::admin::roles::Role;
use crate::types::admin::users::User;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Summary information about a group returned in role assignment responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Group {
    /// The object type, which is always `group`.
    pub object: String,
    /// Identifier for the group.
    pub id: String,
    /// Display name of the group.
    pub name: String,
    /// Unix timestamp (in seconds) when the group was created.
    pub created_at: u64,
    /// Whether the group is managed through SCIM.
    pub scim_managed: bool,
}

/// Details about an organization group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupResponse {
    /// Identifier for the group.
    pub id: String,
    /// Display name of the group.
    pub name: String,
    /// Unix timestamp (in seconds) when the group was created.
    pub created_at: u64,
    /// Whether the group is managed through SCIM and controlled by your identity provider.
    pub is_scim_managed: bool,
}

/// Paginated list of organization groups.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Groups returned in the current page.
    pub data: Vec<GroupResponse>,
    /// Whether additional groups are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` if there are no more results.
    pub next: Option<String>,
}

/// Confirmation payload returned after deleting a group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupDeletedResource {
    /// The object type, which is always `group.deleted`.
    pub object: String,
    /// Identifier of the deleted group.
    pub id: String,
    /// Whether the group was deleted.
    pub deleted: bool,
}

/// Response returned after updating a group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupResourceWithSuccess {
    /// Identifier for the group.
    pub id: String,
    /// Updated display name for the group.
    pub name: String,
    /// Unix timestamp (in seconds) when the group was created.
    pub created_at: u64,
    /// Whether the group is managed through SCIM and controlled by your identity provider.
    pub is_scim_managed: bool,
}

/// Request payload for creating a new group in the organization.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "CreateGroupRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateGroupBody {
    /// Human readable name for the group.
    pub name: String,
}

/// Request payload for updating the details of an existing group.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "UpdateGroupRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateGroupBody {
    /// New display name for the group.
    pub name: String,
}

/// Request payload for adding a user to a group.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "CreateGroupUserRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateGroupUserBody {
    /// Identifier of the user to add to the group.
    pub user_id: String,
}

/// Confirmation payload returned after adding a user to a group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupUserAssignment {
    /// The object type, which is always `group.user`.
    pub object: String,
    /// Identifier of the user that was added.
    pub user_id: String,
    /// Identifier of the group the user was added to.
    pub group_id: String,
}

/// Confirmation payload returned after removing a user from a group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupUserDeletedResource {
    /// The object type, which is always `group.user.deleted`.
    pub object: String,
    /// Whether the group membership was removed.
    pub deleted: bool,
}

/// Role assignment linking a group to a role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupRoleAssignment {
    /// The object type, which is always `group.role`.
    pub object: String,
    /// The group.
    pub group: Group,
    /// The role.
    pub role: Role,
}

/// Paginated list of role assignments for a group.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupRoleAssignmentListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Role assignments returned in the current page.
    pub data: Vec<GroupRoleAssignment>,
    /// Whether additional assignments are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no more assignments.
    pub next: Option<String>,
}

/// Request payload for assigning a role to a group.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "AssignGroupRoleRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct PublicAssignOrganizationGroupRoleBody {
    /// Identifier of the role to assign.
    pub role_id: String,
}

/// Paginated list of user objects returned when inspecting group membership.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Users in the current page.
    pub data: Vec<User>,
    /// Whether more users are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when no further users are available.
    pub next: Option<String>,
}
