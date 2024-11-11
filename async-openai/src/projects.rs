use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{Project, ProjectCreateRequest, ProjectListResponse, ProjectUpdateRequest},
    Client,
};

/// Manage the projects within an organization includes creation, updating, and archiving or projects.
/// The Default project cannot be modified or archived.
pub struct Projects<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Projects<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Returns a list of projects.
    pub async fn list<Q>(&self, query: &Q) -> Result<ProjectListResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/projects", query)
            .await
    }

    /// Create a new project in the organization. Projects can be created and archived, but cannot be deleted.
    pub async fn create(&self, request: ProjectCreateRequest) -> Result<Project, OpenAIError> {
        self.client.post("/organization/projects", request).await
    }

    /// Retrieves a project.
    pub async fn retrieve(&self, project_id: String) -> Result<Project, OpenAIError> {
        self.client
            .get(format!("/organization/projects/{project_id}").as_str())
            .await
    }

    /// Modifies a project in the organization.
    pub async fn modify(
        &self,
        project_id: String,
        request: ProjectUpdateRequest,
    ) -> Result<Project, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{project_id}").as_str(),
                request,
            )
            .await
    }

    /// Archives a project in the organization. Archived projects cannot be used or updated.
    pub async fn archive(&self, project_id: String) -> Result<Project, OpenAIError> {
        self.client
            .post(format!("/organization/projects/{project_id}").as_str(), ())
            .await
    }
}
