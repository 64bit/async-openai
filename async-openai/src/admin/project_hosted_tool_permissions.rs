use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::permissions::{
        ProjectHostedToolPermissions, ProjectHostedToolPermissionsUpdateRequest,
    },
    Client, RequestOptions,
};

/// Manage hosted tool permissions for one project.
pub struct ProjectHostedToolPermissionsApi<'c, C: Config> {
    client: &'c Client<C>,
    project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectHostedToolPermissionsApi<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns hosted tool permissions for the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectHostedToolPermissions, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{}/hosted_tool_permissions",
                    self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates hosted tool permissions for the project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: ProjectHostedToolPermissionsUpdateRequest,
    ) -> Result<ProjectHostedToolPermissions, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{}/hosted_tool_permissions",
                    self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }
}
