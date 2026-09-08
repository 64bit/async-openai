#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MisalignmentErrorDetailsResource {
    /// An optional classification; clients must accept additional values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<MisalignmentErrorType>,
    /// The public explanation for this block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_explanation: Option<String>,
    /// An optional public continuation instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steer: Option<MisalignmentSteer>,
}

pub type MisalignmentErrorType = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MisalignmentSteer {
    /// The public continuation instruction.
    pub message: String,
}
