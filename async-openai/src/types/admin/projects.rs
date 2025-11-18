use crate::types::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Query parameters for listing projects.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// If `true` returns all projects including those that have been `archived`. Archived projects are not included by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
}

/// `active` or `archived`
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Active,
    Archived,
}

/// Represents an individual project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always `organization.project`
    pub object: String,
    /// The name of the project. This appears in reporting.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the project was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the project was archived or `null`.
    pub archived_at: Option<u64>,
    /// `active` or `archived`
    pub status: ProjectStatus,
}

/// A list of Project objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectListResponse {
    pub object: String,
    pub data: Vec<Project>,
    pub first_id: String,
    pub last_id: String,
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
}

/// The project update request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUpdateRequest {
    /// The updated name of the project, this name appears in reports.
    pub name: String,
}
