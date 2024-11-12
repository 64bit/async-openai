use crate::types::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::OrganizationRole;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InviteStatus {
    Accepted,
    Expired,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "InviteRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InviteRequest {
    email: String,
    role: OrganizationRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteListResponse {
    object: String,
    data: Vec<Invite>,
    first_id: Option<String>,
    last_id: Option<String>,
    has_more: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteDeleteResponse {
    /// The object type, which is always `organization.invite.deleted`
    object: String,
    id: String,
    deleted: bool,
}

/// Represents an individual `invite` to the organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Invite {
    /// The object type, which is always `organization.invite`
    object: String,
    /// The identifier, which can be referenced in API endpoints
    id: String,
    /// The email address of the individual to whom the invite was sent
    email: String,
    /// `owner` or `reader`
    role: OrganizationRole,
    /// `accepted`, `expired`, or `pending`
    status: InviteStatus,
    /// The Unix timestamp (in seconds) of when the invite was sent.
    invited_at: u32,
    /// The Unix timestamp (in seconds) of when the invite expires.
    expires_at: u32,
    /// The Unix timestamp (in seconds) of when the invite was accepted.
    accepted_at: Option<u32>,
}
