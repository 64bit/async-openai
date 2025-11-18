use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::usage::{UsageBucketWidth, UsageGroupBy};

/// Sort order for listing admin API keys.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListAdminApiKeysOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing admin API keys.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListAdminApiKeysQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListAdminApiKeysQuery {
    /// Return keys with IDs that come after this ID in the pagination order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Order results by creation time, ascending or descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdminApiKeysOrder>,
    /// Maximum number of keys to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Query parameters for listing audit logs.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListAuditLogsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListAuditLogsQuery {
    /// Return only events whose `effective_at` (Unix seconds) is in this range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<serde_json::Value>,
    /// Return only events for these projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Return only events with a `type` in one of these values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    /// Return only events performed by these actors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<String>>,
    /// Return only events performed by users with these emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_emails: Option<Vec<String>>,
    /// Return only events performed on these targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Sort order for listing organization certificates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListOrganizationCertificatesOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing organization certificates.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListOrganizationCertificatesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListOrganizationCertificatesQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListOrganizationCertificatesOrder>,
}

/// Query parameters for listing invites.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListInvitesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListInvitesQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Query parameters for listing projects.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// If `true` returns all projects including those that have been `archived`. Archived projects are not included by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
}

/// Query parameters for listing project API keys.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectApiKeysQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectApiKeysQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Sort order for listing project certificates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListProjectCertificatesOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Query parameters for listing project certificates.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectCertificatesQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectCertificatesQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListProjectCertificatesOrder>,
}

/// Query parameters for listing project rate limits.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectRateLimitsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectRateLimitsQuery {
    /// A limit on the number of objects to be returned. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// Query parameters for listing project service accounts.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectServiceAccountsQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectServiceAccountsQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Query parameters for listing project users.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListProjectUsersQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListProjectUsersQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Query parameters for listing users.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ListUsersQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListUsersQuery {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Filter by the email address of users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

/// Query parameters for organization usage endpoints.
#[derive(Debug, Clone, Serialize, Default)]
pub struct UsageQueryParams {
    /// Start time (Unix seconds) of the query time range, inclusive.
    pub start_time: u64,
    /// End time (Unix seconds) of the query time range, exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Width of each time bucket in response. Currently `1m`, `1h` and `1d` are supported, default to `1d`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_width: Option<UsageBucketWidth>,
    /// Return only usage for these projects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Return only usage for these users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// Return only usage for these API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_ids: Option<Vec<String>>,
    /// Return only usage for these models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    /// If `true`, return batch jobs only. If `false`, return non-batch jobs only. By default, return both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
    /// Group the usage data by the specified fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<UsageGroupBy>>,
    /// Specifies the number of buckets to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// A cursor for use in pagination. Corresponding to the `next_page` field from the previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

/// Query parameters for getting a certificate.
#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "GetCertificateQueryArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct GetCertificateQuery {
    /// A list of additional fields to include in the response. Currently the only supported value is `content` to fetch the PEM content of the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

