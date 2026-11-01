use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub use crate::types::shared::{
    AwsExternalStorageProviderResponse, AzureExternalStorageProviderResponse,
    ExternalStorageResponseProvider, GcpExternalStorageProviderResponse,
    OciExternalStorageProviderResponse,
};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalStorageDeletedResource {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalStorageListResource {
    pub object: String,
    pub data: Vec<ExternalStorageResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalStorageResponse {
    pub object: String,
    pub id: String,
    pub project_id: String,
    pub provider: ExternalStorageResponseProvider,
    pub geography: String,
    pub status: ExternalStorageStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExternalStorageStatus {
    Pending,
    Validated,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "CreateExternalStorageBodyArgs",
    pattern = "mutable",
    setter(into, strip_option)
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateExternalStorageBody {
    pub project_id: String,
    pub provider: CreateExternalStorageBodyProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateExternalStorageBodyProvider {
    Aws(AwsExternalStorageProviderParams),
    Azure(AzureExternalStorageProviderParams),
    Gcp(GcpExternalStorageProviderParams),
    Oci(OciExternalStorageProviderParams),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OciExternalStorageProviderParams {
    pub tenancy_ocid: String,
    pub region: String,
    pub bucket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GcpExternalStorageProviderParams {
    pub bucket: String,
    pub workload_identity_project_number: String,
    pub workload_identity_pool_id: String,
    pub workload_identity_provider_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AzureExternalStorageProviderParams {
    pub tenant_id: String,
    pub subscription_id: String,
    pub resource_group: String,
    pub account_name: String,
    pub container: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AwsExternalStorageProviderParams {
    pub bucket: String,
    pub role_arn: String,
}

/// Query parameters for listing organization external storage.
#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(
    name = "ListExternalStorageQueryArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListExternalStorageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
