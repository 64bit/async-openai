use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SafetyAlertErrorType {
    PotentiallyUnintendedDataTransfer,
    PotentiallyUnintendedDataAccess,
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafetyAlertResource {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub request_id: String,
    pub response_id: String,
    pub model: String,
    /// Whether block registration succeeded for this request. This does not confirm that response execution
    /// stopped.
    pub request_paused: bool,
    pub error_type: SafetyAlertErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Public explanation of the safety alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafetyCaseResource {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub entity_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub notice: SafetyCaseNotice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafetyCaseNotice {
    pub r#type: SafetyCaseNoticeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SafetyCaseNoticeType {
    Warning,
    Deactivation,
}
