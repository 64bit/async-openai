use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        ProjectSpendLimitDeletedResource, ProjectSpendLimitResource, UpdateProjectSpendLimitBody,
    },
    Client, RequestOptions,
};

pub struct ProjectSpendLimit<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
    pub project_id: String,
}

impl<'c, C: Config> ProjectSpendLimit<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
            project_id: project_id.into(),
        }
    }

    /// Get a project's hard spend limit.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectSpendLimitResource, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{project_id}/spend_limit",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Delete a project's hard spend limit.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn delete(&self) -> Result<ProjectSpendLimitDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!(
                    "/organization/projects/{project_id}/spend_limit",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Create or replace a project's hard spend limit.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateProjectSpendLimitBody,
    ) -> Result<ProjectSpendLimitResource, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{project_id}/spend_limit",
                    project_id = self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }
}
