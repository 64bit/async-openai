use serde::{Deserialize, Serialize};

use super::{SpendLimitCurrency, SpendLimitEnforcement, SpendLimitInterval};

/// Confirmation payload returned after deleting an organization hard spend limit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationSpendLimitDeletedResource {
    /// The object type, which is always `organization.spend_limit.deleted`.
    pub object: String,
    /// Whether the hard spend limit was deleted.
    pub deleted: bool,
}

/// Represents a hard spend limit configured at the organization level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationSpendLimitResource {
    /// The object type, which is always `organization.spend_limit`.
    pub object: String,
    /// The hard spend limit amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount. Currently, only `USD` is supported.
    pub currency: SpendLimitCurrency,
    /// The time interval for evaluating spend against the threshold. Currently, only `month` is supported.
    pub interval: SpendLimitInterval,
    /// The current enforcement state of the hard spend limit.
    pub enforcement: SpendLimitEnforcement,
}

/// Parameters for the hard spend limit you want to create or replace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateOrganizationSpendLimitBody {
    /// The hard spend limit amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount. Currently, only `USD` is supported.
    pub currency: SpendLimitCurrency,
    /// The time interval for evaluating spend against the threshold. Currently, only `month` is supported.
    pub interval: SpendLimitInterval,
}
