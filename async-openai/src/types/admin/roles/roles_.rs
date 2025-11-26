use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Owner,
    Reader,
}

/// Details about a role that can be assigned through the public Roles API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    /// The object type, which is always `role`.
    pub object: String,
    /// Identifier for the role.
    pub id: String,
    /// Unique name for the role.
    pub name: String,
    /// Optional description of the role.
    pub description: Option<String>,
    /// Permissions granted by the role.
    pub permissions: Vec<String>,
    /// Resource type the role is bound to (for example `api.organization` or `api.project`).
    pub resource_type: String,
    /// Whether the role is predefined and managed by OpenAI.
    pub predefined_role: bool,
}

/// Paginated list of roles available on an organization or project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PublicRoleListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Roles returned in the current page.
    pub data: Vec<Role>,
    /// Whether more roles are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no additional roles.
    pub next: Option<String>,
}

/// Request payload for creating a custom role.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "CreateOrganizationRoleRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct PublicCreateOrganizationRoleBody {
    /// Unique name for the role.
    pub role_name: String,
    /// Permissions to grant to the role.
    pub permissions: Vec<String>,
    /// Optional description of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request payload for updating an existing role.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "UpdateOrganizationRoleRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct PublicUpdateOrganizationRoleBody {
    /// Updated set of permissions for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// New description for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New name for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

/// Confirmation payload returned after deleting a role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleDeletedResource {
    /// The object type, which is always `role.deleted`.
    pub object: String,
    /// Identifier of the deleted role.
    pub id: String,
    /// Whether the role was deleted.
    pub deleted: bool,
}

/// Detailed information about a role assignment entry returned when listing assignments.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssignedRoleDetails {
    /// Identifier for the role.
    pub id: String,
    /// Name of the role.
    pub name: String,
    /// Permissions associated with the role.
    pub permissions: Vec<String>,
    /// Resource type the role applies to.
    pub resource_type: String,
    /// Whether the role is predefined by OpenAI.
    pub predefined_role: bool,
    /// Description of the role.
    pub description: Option<String>,
    /// When the role was created.
    pub created_at: Option<u64>,
    /// When the role was last updated.
    pub updated_at: Option<u64>,
    /// Identifier of the actor who created the role.
    pub created_by: Option<String>,
    /// User details for the actor that created the role, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_obj: Option<serde_json::Value>,
    /// Arbitrary metadata stored on the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Paginated list of roles assigned to a principal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Role assignments returned in the current page.
    pub data: Vec<AssignedRoleDetails>,
    /// Whether additional assignments are available when paginating.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no more assignments.
    pub next: Option<String>,
}

/// Confirmation payload returned after unassigning a role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeletedRoleAssignmentResource {
    /// Identifier for the deleted assignment, such as `group.role.deleted` or `user.role.deleted`.
    pub object: String,
    /// Whether the assignment was removed.
    pub deleted: bool,
}
