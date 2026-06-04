use serde::{Deserialize, Serialize};

/// The user that owns a project API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKeyOwnerUser {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The email address of the user.
    pub email: String,
    /// The name of the user.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the user was created.
    pub created_at: u64,
    /// The user's project role.
    pub role: String,
}

/// The service account that owns a project API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKeyOwnerServiceAccount {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the service account.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the service account was created.
    pub created_at: u64,
    /// The service account's project role.
    pub role: String,
}

/// Represents an individual API key in a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKey {
    /// The object type, which is always `organization.project.api_key`.
    pub object: String,
    /// The redacted value of the API key.
    pub redacted_value: String,
    /// The name of the API key.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the API key was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the API key was last used.
    pub last_used_at: Option<u64>,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The owner of the API key.
    pub owner: ProjectApiKeyOwner,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectApiKeyOwnerType {
    User,
    ServiceAccount,
}

/// Represents the owner of a project API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKeyOwner {
    /// The type of owner, which is either `user` or `service_account`.
    pub r#type: ProjectApiKeyOwnerType,
    /// The user owner of the API key, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<ProjectApiKeyOwnerUser>,
    /// The service account owner of the API key, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ProjectApiKeyOwnerServiceAccount>,
}

/// Represents the response object for listing project API keys.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKeyListResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of project API keys.
    pub data: Vec<ProjectApiKey>,
    /// The ID of the first project API key in the list.
    pub first_id: Option<String>,
    /// The ID of the last project API key in the list.
    pub last_id: Option<String>,
    /// Indicates if there are more project API keys available.
    pub has_more: bool,
}

/// Represents the response object for deleting a project API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectApiKeyDeleteResponse {
    /// The object type, which is always `organization.project.api_key.deleted`.
    pub object: String,
    /// The ID of the deleted API key.
    pub id: String,
    /// Indicates if the API key was successfully deleted.
    pub deleted: bool,
}
