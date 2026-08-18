use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::permissions::{
        ProjectModelPermissions, ProjectModelPermissionsDeleteResponse,
        ProjectModelPermissionsUpdateRequest,
    },
    Client, RequestOptions,
};

/// Manage model permissions for one project.
pub struct ProjectModelPermissionsApi<'c, C: Config> {
    client: &'c Client<C>,
    project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectModelPermissionsApi<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns model permissions for the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectModelPermissions, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{}/model_permissions",
                    self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates model permissions for the project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: ProjectModelPermissionsUpdateRequest,
    ) -> Result<ProjectModelPermissions, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{}/model_permissions",
                    self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes model permissions for the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn delete(&self) -> Result<ProjectModelPermissionsDeleteResponse, OpenAIError> {
        self.client
            .delete(
                &format!(
                    "/organization/projects/{}/model_permissions",
                    self.project_id
                ),
                &self.request_options,
            )
            .await
    }
}
