use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::project_rate_limits::{
        ProjectRateLimit, ProjectRateLimitListResponse, ProjectRateLimitUpdateRequest,
    },
    Client,
};

/// Manage rate limits for a given project. Supports listing and updating rate limits per model.
pub struct ProjectRateLimits<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
}

impl<'c, C: Config> ProjectRateLimits<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }

    /// Returns the rate limits per model for a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ProjectRateLimitListResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/organization/projects/{}/rate_limits", self.project_id).as_str(),
                &query,
            )
            .await
    }

    /// Updates a project rate limit.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        rate_limit_id: &str,
        request: ProjectRateLimitUpdateRequest,
    ) -> Result<ProjectRateLimit, OpenAIError> {
        self.client
            .post(
                format!(
                    "/organization/projects/{}/rate_limits/{rate_limit_id}",
                    self.project_id
                )
                .as_str(),
                request,
            )
            .await
    }
}
