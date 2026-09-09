use serde::{Deserialize, Serialize};

use super::SpendAlertNotificationChannel;

/// Represents a spend alert configured at the organization level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationSpendAlert {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `organization.spend_alert`.
    pub object: String,
    /// The alert threshold amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount.
    pub currency: OrganizationSpendAlertCurrency,
    /// The time interval for evaluating spend against the threshold.
    pub interval: OrganizationSpendAlertInterval,
    pub notification_channel: SpendAlertNotificationChannel,
}

/// The currency for the threshold amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrganizationSpendAlertCurrency {
    #[serde(rename = "USD")]
    USD,
}

/// Confirmation payload returned after deleting an organization spend alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationSpendAlertDeletedResource {
    /// The deleted spend alert ID.
    pub id: String,
    /// Always `organization.spend_alert.deleted`.
    pub object: String,
    /// Whether the spend alert was deleted.
    pub deleted: bool,
}

/// The time interval for evaluating spend against the threshold.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrganizationSpendAlertInterval {
    #[serde(rename = "month")]
    Month,
}

/// Paginated list of organization spend alerts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationSpendAlertListResource {
    /// Always `list`.
    pub object: String,
    /// Spend alerts returned in the current page.
    pub data: Vec<OrganizationSpendAlert>,
    /// The ID of the first spend alert in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last spend alert in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether more spend alerts are available when paginating.
    pub has_more: bool,
}
