use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ProjectUser, ProjectUserCreateRequest, ProjectUserListResponse, ProjectUserUpdateRequest,
    },
    Client,
};

/// Manage users within a project, including adding, updating roles, and removing users.
/// Users cannot be removed from the Default project, unless they are being removed from the organization.
pub struct ProjectUsers<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> ProjectUsers<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Returns a list of users in the project.
    pub async fn list<Q>(
        &self,
        project_id: String,
        query: &Q,
    ) -> Result<ProjectUserListResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/organization/projects/{project_id}/users").as_str(),
                query,
            )
            .await
    }

    /// Adds a user to the project. Users must already be members of the organization to be added to a project.
    pub async fn create(
        &self,
        project_id: String,
        request: ProjectUserCreateRequest,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{project_id}/users").as_str(),
                request,
            )
            .await
    }

    /// Retrieves a user in the project.
    pub async fn retrieve(
        &self,
        project_id: String,
        user_id: String,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .get(format!("/organization/projects/{project_id}/users/{user_id}").as_str())
            .await
    }

    /// Modifies a user's role in the project.
    pub async fn modify(
        &self,
        project_id: String,
        request: ProjectUserUpdateRequest,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{project_id}").as_str(),
                request,
            )
            .await
    }

    /// Deletes a user from the project.
    pub async fn delete(
        &self,
        project_id: String,
        user_id: String,
    ) -> Result<ProjectUser, OpenAIError> {
        self.client
            .delete(format!("/organization/projects/{project_id}/users/{user_id}").as_str())
            .await
    }
}
