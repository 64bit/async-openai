use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InviteRole {
    Owner,
    Reader,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    Accepted,
    Expired,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InviteRequest {
    email: String,
    role: InviteRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InviteListResponse {
    object: String,
    data: Vec<Invite>,
    first_id: Option<String>,
    last_id: Option<String>,
    has_more: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InviteDeleteResponse {
    /// The object type, which is always `organization.invite.deleted`
    object: String,
    id: String,
    deleted: bool,
}

/// Represents an individual `invite` to the organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Invite {
    /// The object type, which is always `organization.invite`
    object: String,
    /// The identifier, which can be referenced in API endpoints
    id: String,
    /// The email address of the individual to whom the invite was sent
    email: String,
    /// `owner` or `reader`
    role: InviteRole,
    /// `accepted`, `expired`, or `pending`
    status: InviteStatus,
    /// The Unix timestamp (in seconds) of when the invite was sent.
    invited_at: u32,
    /// The Unix timestamp (in seconds) of when the invite expires.
    expires_at: u32,
    /// The Unix timestamp (in seconds) of when the invite was accepted.
    accepted_at: u32,
}
