use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::project_users::{
        ProjectUser, ProjectUserCreateRequest, ProjectUserDeleteResponse, ProjectUserListResponse,
        ProjectUserUpdateRequest,
    },
    Client, RequestOptions,
};

/// Manage users within a project, including adding, updating roles, and removing users.
/// Users cannot be removed from the Default project, unless they are being removed from the organization.
pub struct ProjectUsers<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectUsers<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns a list of users in the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectUserListResponse, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/users", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Adds a user to the project. Users must already be members of the organization to be added to a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: ProjectUserCreateRequest,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{}/users", self.project_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieves a user in the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, user_id: &str) -> Result<ProjectUser, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/users/{user_id}", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Modifies a user's role in the project.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn modify(
        &self,
        user_id: &str,
        request: ProjectUserUpdateRequest,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{}/users/{user_id}", self.project_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a user from the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, user_id: &str) -> Result<ProjectUserDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!("/organization/projects/{}/users/{user_id}", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }
}
