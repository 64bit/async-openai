use crate::error::OpenAIError;
use crate::types::admin::roles::OrganizationRole;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InviteStatus {
    Accepted,
    Expired,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProjectMembership {
    Owner,
    #[default]
    Member,
}

/// Project membership entry attached to invites.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteProjectMembership {
    /// Project's public ID
    pub id: String,
    /// Project membership role: `owner` or `member`
    pub role: ProjectMembership,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "InviteRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InviteRequest {
    pub email: String,
    pub role: OrganizationRole,
    /// An array of projects to which membership is granted at the same time the
    /// org invite is accepted. If omitted, the user will be invited to the
    /// default project for compatibility with legacy behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<InviteProjectMembership>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteListResponse {
    pub object: String,
    pub data: Vec<Invite>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteDeleteResponse {
    /// The object type, which is always `organization.invite.deleted`
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

/// Represents an individual `invite` to the organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Invite {
    /// The object type, which is always `organization.invite`
    pub object: String,
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The email address of the individual to whom the invite was sent
    pub email: String,
    /// `owner` or `reader`
    pub role: OrganizationRole,
    /// `accepted`, `expired`, or `pending`
    pub status: InviteStatus,
    /// The Unix timestamp (in seconds) of when the invite was sent.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the invite expires.
    pub expires_at: Option<u64>,
    /// The Unix timestamp (in seconds) of when the invite was accepted.
    pub accepted_at: Option<u64>,
    /// The projects that were granted membership upon acceptance of the invite.
    pub projects: Vec<InviteProjectMembership>,
}
