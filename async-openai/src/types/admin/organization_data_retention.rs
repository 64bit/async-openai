use serde::{Deserialize, Serialize};

/// Represents the organization's data retention control setting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationDataRetention {
    /// The object type, which is always `organization.data_retention`.
    pub object: String,
    /// The configured organization data retention type.
    pub r#type: OrganizationDataRetentionType,
}

/// The configured organization data retention type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrganizationDataRetentionType {
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}

/// Parameters for updating organization data retention controls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateOrganizationDataRetentionBody {
    /// The desired organization data retention type.
    pub retention_type: OrganizationDataRetentionType,
}
