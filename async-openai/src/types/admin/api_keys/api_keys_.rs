use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual Admin API key in an org.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminApiKey {
    /// The object type, which is always `organization.admin_api_key`.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the API key.
    pub name: String,
    /// The redacted value of the API key.
    pub redacted_value: String,
    /// The value of the API key. Only shown on create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The Unix timestamp (in seconds) of when the API key was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the API key was last used.
    pub last_used_at: Option<u64>,
    /// The owner of the API key.
    pub owner: AdminApiKeyOwner,
}

/// Represents the owner of an admin API key.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminApiKeyOwner {
    pub r#type: String,

    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the owner.
    pub name: String,
    /// The Unix timestamp (in seconds) of when the owner was created.
    pub created_at: u64,

    pub role: String,
}

/// Represents the response object for listing admin API keys.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApiKeyList {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of admin API keys.
    pub data: Vec<AdminApiKey>,
    /// Indicates if there are more admin API keys available.
    pub has_more: bool,
    /// The ID of the first admin API key in the list.
    pub first_id: String,
    /// The ID of the last admin API key in the list.
    pub last_id: String,
}

/// Represents the request object for creating an admin API key.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "CreateAdminApiKeyRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateAdminApiKeyRequest {
    /// The name of the API key being created.
    pub name: String,
}

/// Represents the response object for deleting an admin API key.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminApiKeyDeleteResponse {
    /// The object type, which is always `organization.admin_api_key.deleted`.
    pub object: String,
    /// The ID of the deleted API key.
    pub id: String,
    /// Indicates if the API key was successfully deleted.
    pub deleted: bool,
}
