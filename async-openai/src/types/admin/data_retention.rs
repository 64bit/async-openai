use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationDataRetentionType {
    ZeroDataRetention,
    ModifiedAbuseMonitoring,
    EnhancedZeroDataRetention,
    EnhancedModifiedAbuseMonitoring,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectDataRetentionType {
    OrganizationDefault,
    None,
    ZeroDataRetention,
    ModifiedAbuseMonitoring,
    EnhancedZeroDataRetention,
    EnhancedModifiedAbuseMonitoring,
}

/// Organization data-retention control setting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OrganizationDataRetention {
    pub object: String,
    #[serde(rename = "type")]
    pub type_: OrganizationDataRetentionType,
}

/// Project data-retention control setting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProjectDataRetention {
    pub object: String,
    #[serde(rename = "type")]
    pub type_: ProjectDataRetentionType,
}

/// Parameters for updating organization data-retention controls.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "UpdateOrganizationDataRetentionArgs",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct UpdateOrganizationDataRetentionBody {
    pub retention_type: OrganizationDataRetentionType,
}

/// Parameters for updating project data-retention controls.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "UpdateProjectDataRetentionArgs",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct UpdateProjectDataRetentionBody {
    pub retention_type: ProjectDataRetentionType,
}
