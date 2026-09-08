#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SafetyAlertErrorType {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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
}
