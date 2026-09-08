use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{ProjectDataRetention, UpdateProjectDataRetentionBody},
    Client, RequestOptions,
};

pub struct ProjectDataRetentions<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
    pub project_id: String,
}
impl<'c, C: Config> ProjectDataRetentions<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
            project_id: project_id.into(),
        }
    }

    /// Retrieves project data retention controls.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectDataRetention, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{project_id}/data_retention",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates project data retention controls.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateProjectDataRetentionBody,
    ) -> Result<ProjectDataRetention, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{project_id}/data_retention",
                    project_id = self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }
}
