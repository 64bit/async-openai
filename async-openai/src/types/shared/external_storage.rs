use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExternalStorageResponseProvider {
    Aws(AwsExternalStorageProviderResponse),
    Azure(AzureExternalStorageProviderResponse),
    Gcp(GcpExternalStorageProviderResponse),
    Oci(OciExternalStorageProviderResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AwsExternalStorageProviderResponse {
    pub account_id: String,
    pub region: String,
    pub bucket: String,
    pub role_arn: String,
    pub external_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AzureExternalStorageProviderResponse {
    pub tenant_id: String,
    pub subscription_id: String,
    pub resource_group: String,
    pub account_name: String,
    pub container: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GcpExternalStorageProviderResponse {
    pub bucket: String,
    pub workload_identity_project_number: String,
    pub workload_identity_pool_id: String,
    pub workload_identity_provider_id: String,
    pub region: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OciExternalStorageProviderResponse {
    pub tenancy_ocid: String,
    pub region: String,
    pub bucket: String,
}
