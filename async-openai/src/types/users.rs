use crate::types::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::OrganizationRole;

/// Represents an individual `user` within an organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// The object type, which is always `organization.user`
    object: String,
    /// The identifier, which can be referenced in API endpoints
    id: String,
    /// The name of the user
    name: String,
    /// The email address of the user
    email: String,
    /// `owner` or `reader`
    role: OrganizationRole,
    /// The Unix timestamp (in seconds) of when the users was added.
    added_at: u32,
}

/// A list of `User` objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UserListResponse {
    object: String,
    data: Vec<User>,
    first_id: String,
    last_id: String,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
#[serde(rename_all = "snake_case")]
pub struct UserRoleUpdateRequest {
    /// `owner` or `reader`
    role: OrganizationRole,
}

/// Confirmation of the deleted user
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UserDeleteResponse {
    object: String,
    id: String,
    deleted: bool,
}
