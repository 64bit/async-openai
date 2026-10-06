use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "CreateExternalStorageBodyArgs",
    pattern = "mutable",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct CreateExternalStorageBody {
    pub project_id: String,
    pub provider: ExternalStorageProviderParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExternalStorageProviderParams {
    Aws(AwsExternalStorageProviderParams),
    Azure(AzureExternalStorageProviderParams),
    Gcp(GcpExternalStorageProviderParams),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsExternalStorageProviderParams {
    pub bucket: String,
    pub role_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AzureExternalStorageProviderParams {
    pub tenant_id: String,
    pub subscription_id: String,
    pub resource_group: String,
    pub account_name: String,
    pub container: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcpExternalStorageProviderParams {
    pub bucket: String,
    pub workload_identity_project_number: String,
    pub workload_identity_pool_id: String,
    pub workload_identity_provider_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalStorageResponse {
    pub object: String,
    pub id: String,
    pub project_id: String,
    pub provider: ExternalStorageProviderResponse,
    pub geography: String,
    pub status: ExternalStorageStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExternalStorageProviderResponse {
    Aws(AwsExternalStorageProviderResponse),
    Azure(AzureExternalStorageProviderResponse),
    Gcp(GcpExternalStorageProviderResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsExternalStorageProviderResponse {
    pub account_id: String,
    pub region: String,
    pub bucket: String,
    pub role_arn: String,
    pub external_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AzureExternalStorageProviderResponse {
    pub tenant_id: String,
    pub subscription_id: String,
    pub resource_group: String,
    pub account_name: String,
    pub container: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcpExternalStorageProviderResponse {
    pub bucket: String,
    pub workload_identity_project_number: String,
    pub workload_identity_pool_id: String,
    pub workload_identity_provider_id: String,
    pub region: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExternalStorageStatus {
    Pending,
    Validated,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalStorageDeletedResource {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalStorageListResource {
    pub object: String,
    pub data: Vec<ExternalStorageResponse>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExternalStorageOrder {
    Asc,
    Desc,
}

/// Query parameters for listing external storage configurations.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Builder)]
#[builder(
    name = "ListExternalStorageQueryArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct ListExternalStorageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ExternalStorageOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_union_is_internally_tagged() {
        let provider = ExternalStorageProviderParams::Aws(AwsExternalStorageProviderParams {
            bucket: "logs".into(),
            role_arn: "arn:aws:iam::123:role/openai".into(),
        });
        let value = serde_json::to_value(provider).unwrap();
        assert_eq!(value["type"], "aws");
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
