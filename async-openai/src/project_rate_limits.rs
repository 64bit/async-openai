use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::project_rate_limits::{
        ProjectRateLimit, ProjectRateLimitListResponse, ProjectRateLimitUpdateRequest,
    },
    Client, RequestOptions,
};

/// Manage rate limits for a given project. Supports listing and updating rate limits per model.
pub struct ProjectRateLimits<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectRateLimits<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns the rate limits per model for a project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectRateLimitListResponse, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/rate_limits", self.project_id).as_str(),
                &self.request_options,
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
                &self.request_options,
            )
            .await
    }
}
