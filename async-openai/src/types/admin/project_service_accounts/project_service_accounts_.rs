use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectServiceAccountRole {
    Owner,
    Member,
    None,
}

/// Represents an individual service account in a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccount {
    /// The object type, which is always `organization.project.service_account`.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the service account.
    pub name: String,
    /// `owner`, `member`, or `none`
    pub role: ProjectServiceAccountRole,
    /// The Unix timestamp (in seconds) of when the service account was created.
    pub created_at: u64,
}

/// Represents the response object for listing project service accounts.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccountListResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of project service accounts.
    pub data: Vec<ProjectServiceAccount>,
    /// The ID of the first project service account in the list.
    pub first_id: Option<String>,
    /// The ID of the last project service account in the list.
    pub last_id: Option<String>,
    /// Indicates if there are more project service accounts available.
    pub has_more: bool,
}

/// Represents the request object for creating a project service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccountCreateRequest {
    /// The name of the service account being created.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_service_account_only: Option<bool>,
}

/// Represents the response object for creating a project service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccountCreateResponse {
    /// The object type, which is always `organization.project.service_account`.
    pub object: String,
    /// The ID of the created service account.
    pub id: String,
    /// The name of the created service account.
    pub name: String,
    /// Service accounts created with default project membership have role `member`. Accounts created with
    /// `create_service_account_only` have role `none`.
    pub role: String,
    /// The Unix timestamp (in seconds) of when the service account was created.
    pub created_at: u64,
    /// The API key associated with the created service account.
    pub api_key: Option<ProjectServiceAccountApiKey>,
}

/// Represents the API key associated with a project service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccountApiKey {
    /// The object type, which is always `organization.project.service_account.api_key`.
    pub object: String,
    /// The value of the API key.
    pub value: String,
    /// The name of the API key.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the API key was created.
    pub created_at: u64,
    /// The ID of the API key.
    pub id: String,
}

/// Represents the response object for deleting a project service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectServiceAccountDeleteResponse {
    /// The object type, which is always `organization.project.service_account.deleted`.
    pub object: String,
    /// The ID of the deleted service account.
    pub id: String,
    /// Indicates if the service account was successfully deleted.
    pub deleted: bool,
}

/// The service account API key create request payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProjectServiceAccountApiKeyBody {
    /// API key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// API key scopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Number of seconds until the API key expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceAccountApiKeyBody {
    /// The object type, which is always `organization.project.service_account.api_key`
    pub object: String,
    /// The unredacted API key value.
    pub value: String,
    /// The name of the API key.
    pub name: String,
    /// The Unix timestamp (in seconds) when the API key was created.
    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// The identifier of the API key.
    pub id: String,
}

/// Parameters for updating a project service account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProjectServiceAccountBody {
    /// The updated service account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The updated service account role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdateProjectServiceAccountBodyRole>,
}

/// The updated service account role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateProjectServiceAccountBodyRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
