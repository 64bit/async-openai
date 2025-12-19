use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::Serialize;

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
