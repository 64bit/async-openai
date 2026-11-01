use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Query parameters for listing project API keys.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectApiKeysQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectApiKeysQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Filter by owner project access: active, inactive, or any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_project_access: Option<String>,
}

/// Update scope restrictions or expiration. Supply at least one field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder, PartialEq)]
#[builder(
    name = "UpdateProjectApiKeyBodyArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateProjectApiKeyBody {
    /// Expire the key this many seconds from the request (1–31536000). Retrying resets expiry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<u64>,
    /// Replace scope restrictions. An empty array removes key restrictions, subject to owner permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}
