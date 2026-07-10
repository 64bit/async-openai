use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SpendAlertCurrency {
    USD,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpendAlertInterval {
    Month,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpendAlertNotificationChannelType {
    Email,
}

/// Email notification settings for a spend alert.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "SpendAlertNotificationChannelArgs",
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "OpenAIError")
)]
pub struct SpendAlertNotificationChannel {
    #[serde(rename = "type")]
    pub type_: SpendAlertNotificationChannelType,
    pub recipients: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_prefix: Option<String>,
}

/// Parameters for creating or updating a spend alert.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "CreateSpendAlertArgs",
    pattern = "mutable",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct CreateSpendAlertBody {
    /// Alert threshold amount in cents.
    pub threshold_amount: u64,
    pub currency: SpendAlertCurrency,
    pub interval: SpendAlertInterval,
    pub notification_channel: SpendAlertNotificationChannel,
}

/// Query parameters for listing spend alerts.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Default, Builder)]
#[builder(
    name = "ListSpendAlertsQueryArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default,
    build_fn(error = "OpenAIError")
)]
pub struct ListSpendAlertsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListSpendAlertsOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ListSpendAlertsOrder {
    Asc,
    Desc,
}

macro_rules! spend_alert_types {
    ($alert:ident, $list:ident, $deleted:ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        pub struct $alert {
            pub id: String,
            pub object: String,
            pub threshold_amount: u64,
            pub currency: SpendAlertCurrency,
            pub interval: SpendAlertInterval,
            pub notification_channel: SpendAlertNotificationChannel,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        pub struct $list {
            pub object: String,
            pub data: Vec<$alert>,
            pub first_id: Option<String>,
            pub last_id: Option<String>,
            pub has_more: bool,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        pub struct $deleted {
            pub id: String,
            pub object: String,
            pub deleted: bool,
        }
    };
}

spend_alert_types!(
    OrganizationSpendAlert,
    OrganizationSpendAlertListResource,
    OrganizationSpendAlertDeletedResource
);
spend_alert_types!(
    ProjectSpendAlert,
    ProjectSpendAlertListResource,
    ProjectSpendAlertDeletedResource
);
