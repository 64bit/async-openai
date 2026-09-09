use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpendLimitCurrency {
    #[serde(rename = "USD")]
    USD,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpendLimitInterval {
    #[serde(rename = "month")]
    Month,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpendLimitEnforcementStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "enforcing")]
    Enforcing,
    #[serde(untagged)]
    Other(String),
}

/// The current enforcement state of a hard spend limit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpendLimitEnforcement {
    /// Whether the hard spend limit is currently enforcing.
    pub status: SpendLimitEnforcementStatus,
}
