use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        ProjectModelPermissions, ProjectModelPermissionsDeleteResponse,
        ProjectModelPermissionsUpdateRequest,
    },
    Client, RequestOptions,
};

pub struct ProjectModelPermission<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
    pub project_id: String,
}

impl<'c, C: Config> ProjectModelPermission<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
            project_id: project_id.into(),
        }
    }

    /// Returns model permissions for a project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectModelPermissions, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{project_id}/model_permissions",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates model permissions for a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: ProjectModelPermissionsUpdateRequest,
    ) -> Result<ProjectModelPermissions, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{project_id}/model_permissions",
                    project_id = self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes model permissions for a project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn delete(&self) -> Result<ProjectModelPermissionsDeleteResponse, OpenAIError> {
        self.client
            .delete(
                &format!(
                    "/organization/projects/{project_id}/model_permissions",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }
}
