use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always `organization.project`
    pub object: String,
    /// The name of the project. This appears in reporting.
    pub name: Option<String>,
    /// The Unix timestamp (in seconds) of when the project was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the project was archived or `null`.
    pub archived_at: Option<u64>,
    /// `active` or `archived`
    pub status: Option<String>,
    /// The external key associated with the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
}

/// A list of Project objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectListResponse {
    pub object: String,
    pub data: Vec<Project>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: String,
}

/// The project create request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectCreateRequest {
    /// The friendly name of the project, this name appears in reports.
    pub name: String,
    /// Create the project with the specified data residency region. Your
    /// organization must have access to Data residency functionality in
    /// order to use. See [data residency controls](https://developers.openai.com/docs/guides/your-data#data-residency-controls) 
    /// to review the functionality and limitations of setting this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geography: Option<String>,
    /// External key ID to associate with the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
}

/// The project update request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "ProjectUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUpdateRequest {
    /// The updated name of the project, this name appears in reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Geography for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geography: Option<String>,
    /// External key ID to associate with the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
}

/// Details about a group's membership in a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroup {
    /// The object type, which is always `project.group`.
    pub object: String,
    /// Identifier of the project.
    pub project_id: String,
    /// Identifier of the group that has access to the project.
    pub group_id: String,
    /// Display name of the group.
    pub group_name: String,
    /// Unix timestamp (in seconds) when the group was granted project access.
    pub created_at: u64,
    /// The type of the group.
    pub group_type: String,
}

/// Paginated list of groups that have access to a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroupListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Project group memberships returned in the current page.
    pub data: Vec<ProjectGroup>,
    /// Whether additional project group memberships are available.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no more results.
    pub next: Option<String>,
}

/// Confirmation payload returned after removing a group from a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroupDeletedResource {
    /// The object type, which is always `project.group.deleted`.
    pub object: String,
    /// Whether the group membership in the project was removed.
    pub deleted: bool,
}

/// Request payload for granting a group access to a project.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "InviteProjectGroupRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InviteProjectGroupBody {
    /// Identifier of the group to add to the project.
    pub group_id: String,
    /// Identifier of the project role to grant to the group.
    pub role: String,
}
