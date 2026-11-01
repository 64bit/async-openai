use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ValidateExternalKeyResource {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListExternalKeysResource {
    pub object: String,
    pub data: Vec<ExternalKeyResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalKeyResource {
    pub object: String,
    pub id: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ExternalKeyPurpose>,
    pub name: String,
    pub api_project_ids: Vec<String>,
    pub workspace_id: String,
    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_project_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_ring_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExternalKeyPurpose {
    Ekm,
    PspDecryption,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum V1OrganizationExternalKeysPostRequest {
    Aws(AwsExternalKeyParams),
    Gcp(GcpExternalKeyParams),
    Azure(AzureExternalKeyParams),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AzureExternalKeyParams {
    /// A friendly name to identify this config
    pub name: String,
    /// Your Azure tenant UUID
    pub tenant_id: String,
    /// The URI of the Azure vault containing the master key you manage
    pub vault_uri: String,
    /// The name of the Azure Key Vault master key that you manage
    pub key_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ExternalKeyPurpose>,
    /// An exact Azure Key Vault key version. Required for PSP decryption; not accepted for EKM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GcpExternalKeyParams {
    /// A friendly name to identify this config
    pub name: String,
    /// The numeric GCP project number where you registered OpenAI's workload identity
    pub workload_identity_project_number: String,
    /// The pool containing the Workload Identity provider that you registered for OpenAI
    pub workload_identity_pool_id: String,
    /// The Workload Identity provider that you registered for OpenAI
    pub workload_identity_provider_id: String,
    /// The string GCP project name where your KMS resides
    pub kms_project_id: String,
    /// The Key Management System key ring for the master key you manage
    pub kms_key_ring_name: String,
    /// The name of the Key Management System master key you manage
    pub kms_key_name: String,
    /// The region where your Key Management System master key is located
    pub kms_key_location: String,
    /// The aud field in the identity token OpenAI will pass in the call to the customer's GCP STS. This must be the OpenAI organization ID or an API project ID that belongs to the organization.
    pub audience: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ExternalKeyPurpose>,
    /// An exact CryptoKeyVersion number. Required for PSP decryption; not accepted for EKM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AwsExternalKeyParams {
    /// A friendly name to identify this config
    pub name: String,
    /// The Role ARN that OpenAI will assume to call your KMS
    pub role_arn: String,
    /// The Key Management System ARN for the master key you manage
    pub kms_arn: String,
    /// The ExternalID that OpenAI will pass in the AssumeRole call for the customer's AWS account. This must be the OpenAI organization ID or an API project ID that belongs to the organization.
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ExternalKeyPurpose>,
}

/// Query parameters for listing organization external keys.
#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(
    name = "ListExternalKeysQueryArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListExternalKeysQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ExternalKeyPurpose>,
}
