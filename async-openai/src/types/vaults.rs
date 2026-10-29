use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "CreateVaultParamsArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct CreateVaultParams {
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "UpdateVaultParamsArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct UpdateVaultParams {
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultResource {
    pub id: String,
    pub object: String,
    pub name: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultListResource {
    pub object: String,
    pub data: Vec<VaultResource>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VaultStatusParam {
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum VaultStatusFilterParam {
    Status(VaultStatusParam),
    Statuses(Vec<VaultStatusParam>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeletedVaultResource {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "CreateVaultCredentialParamsArgs",
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "OpenAIError")
)]
pub struct CreateVaultCredentialParams {
    pub name: String,
    pub auth: CreateVaultCredentialAuthParam,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "RotateVaultCredentialParamsArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct RotateVaultCredentialParams {
    pub auth: Option<RotateVaultCredentialAuthParam>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultCredentialResource {
    pub id: String,
    pub object: String,
    pub vault_id: String,
    pub name: String,
    pub auth: VaultCredentialAuthResource,
    pub metadata: HashMap<String, String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultCredentialListResource {
    pub object: String,
    pub data: Vec<VaultCredentialResource>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeletedVaultCredentialResource {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

/// Authentication credentials for an MCP server or hosted environment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateVaultCredentialAuthParam {
    McpOauth(CreateMcpOauthCredential),
    StaticBearer(CreateStaticBearerCredential),
    EnvironmentVariable(CreateEnvironmentVariableCredential),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMcpOauthCredential {
    pub mcp_server_url: String,
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<CreateMcpOauthRefreshParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStaticBearerCredential {
    pub mcp_server_url: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateEnvironmentVariableCredential {
    pub secret_name: String,
    pub secret_value: String,
    pub networking: VaultCredentialNetworking,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RotateVaultCredentialAuthParam {
    McpOauth(RotateMcpOauthCredential),
    StaticBearer(RotateStaticBearerCredential),
    EnvironmentVariable(RotateEnvironmentVariableCredential),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RotateMcpOauthCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<RotateMcpOauthRefreshParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RotateStaticBearerCredential {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RotateEnvironmentVariableCredential {
    pub secret_value: String,
}

/// Public authentication configuration; secret values are never returned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VaultCredentialAuthResource {
    McpOauth(McpOauthCredentialResource),
    StaticBearer(StaticBearerCredentialResource),
    EnvironmentVariable(EnvironmentVariableCredentialResource),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpOauthCredentialResource {
    pub mcp_server_url: String,
    pub expires_at: Option<String>,
    pub refresh: Option<McpOauthRefreshResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StaticBearerCredentialResource {
    pub mcp_server_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVariableCredentialResource {
    pub secret_name: String,
    pub networking: VaultCredentialNetworking,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VaultCredentialNetworking {
    Unrestricted,
    Limited { allowed_hosts: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMcpOauthRefreshParam {
    pub token_endpoint: String,
    pub client_id: String,
    pub resource: Option<String>,
    pub scope: Option<String>,
    pub refresh_token: String,
    pub token_endpoint_auth: CreateMcpOauthTokenEndpointAuthParam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateMcpOauthTokenEndpointAuthParam {
    None,
    ClientSecretBasic { client_secret: String },
    ClientSecretPost { client_secret: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RotateMcpOauthRefreshParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth: Option<RotateMcpOauthTokenEndpointAuthParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RotateMcpOauthTokenEndpointAuthParam {
    ClientSecretBasic { client_secret: Option<String> },
    ClientSecretPost { client_secret: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpOauthRefreshResource {
    pub token_endpoint: String,
    pub client_id: String,
    pub resource: Option<String>,
    pub scope: Option<String>,
    pub token_endpoint_auth: McpOauthTokenEndpointAuthResource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum McpOauthTokenEndpointAuthResource {
    None,
    ClientSecretBasic,
    ClientSecretPost,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_union_is_internally_tagged() {
        let auth = CreateVaultCredentialAuthParam::StaticBearer(CreateStaticBearerCredential {
            mcp_server_url: "https://mcp.example.com".into(),
            token: "secret".into(),
        });
        let value = serde_json::to_value(auth).unwrap();
        assert_eq!(value["type"], "static_bearer");
        assert_eq!(
            value
                .as_object()
                .unwrap()
                .keys()
                .filter(|k| *k == "type")
                .count(),
            1
        );
    }
}
