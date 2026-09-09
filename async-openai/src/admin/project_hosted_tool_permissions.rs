use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        ProjectHostedToolPermissions, ProjectHostedToolPermissionsUpdateRequest,
    },
    Client, RequestOptions,
};

pub struct ProjectHostedToolPermission<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
    pub project_id: String,
}

impl<'c, C: Config> ProjectHostedToolPermission<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
            project_id: project_id.into(),
        }
    }

    /// Returns hosted tool permissions for a project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectHostedToolPermissions, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{project_id}/hosted_tool_permissions",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates hosted tool permissions for a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: ProjectHostedToolPermissionsUpdateRequest,
    ) -> Result<ProjectHostedToolPermissions, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{project_id}/hosted_tool_permissions",
                    project_id = self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }
}
