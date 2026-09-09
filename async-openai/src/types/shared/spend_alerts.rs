use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Pagination options for organization and project spend alerts.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder, PartialEq)]
#[builder(name = "ListSpendAlertsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "crate::error::OpenAIError"))]
pub struct ListSpendAlertsQuery {
    /// Maximum number of alerts, between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Ascending or descending order. Defaults to ascending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SpendAlertOrder>,
    /// Return alerts after this cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return alerts before this cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SpendAlertOrder {
    Asc,
    Desc,
}

/// Parameters for creating or updating a spend alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSpendAlertBody {
    /// The alert threshold amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount.
    pub currency: CreateSpendAlertBodyCurrency,
    /// The time interval for evaluating spend against the threshold.
    pub interval: CreateSpendAlertBodyInterval,
    pub notification_channel: SpendAlertNotificationChannel,
}

/// The currency for the threshold amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreateSpendAlertBodyCurrency {
    #[serde(rename = "USD")]
    USD,
}

/// The time interval for evaluating spend against the threshold.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreateSpendAlertBodyInterval {
    #[serde(rename = "month")]
    Month,
}

/// Email notification settings for a spend alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpendAlertNotificationChannel {
    /// The notification channel type. Currently only `email` is supported.
    pub r#type: SpendAlertNotificationChannelType,
    /// Email addresses that receive the spend alert notification.
    pub recipients: Vec<String>,
    /// Optional subject prefix for alert emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_prefix: Option<String>,
}

/// The notification channel type. Currently only `email` is supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpendAlertNotificationChannelType {
    #[serde(rename = "email")]
    Email,
}
