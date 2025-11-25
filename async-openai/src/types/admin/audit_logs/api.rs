use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::Serialize;

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
