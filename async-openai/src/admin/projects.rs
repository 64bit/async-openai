use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        Project, ProjectCreateRequest, ProjectListResponse, ProjectUpdateRequest,
    },
    Client, ProjectAPIKeys, ProjectCertificates, ProjectGroupRoles, ProjectGroups,
    ProjectRateLimits, ProjectRoles, ProjectServiceAccounts, ProjectUserRoles, ProjectUsers,
    RequestOptions,
};

/// Manage the projects within an organization includes creation, updating, and archiving or projects.
/// The Default project cannot be modified or archived.
pub struct Projects<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Projects<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    // call [ProjectUsers] group APIs
    pub fn users(&self, project_id: &str) -> ProjectUsers<'_, C> {
        ProjectUsers::new(self.client, project_id)
    }

    // call [ProjectServiceAccounts] group APIs
    pub fn service_accounts(&self, project_id: &str) -> ProjectServiceAccounts<'_, C> {
        ProjectServiceAccounts::new(self.client, project_id)
    }

    // call [ProjectAPIKeys] group APIs
    pub fn api_keys(&self, project_id: &str) -> ProjectAPIKeys<'_, C> {
        ProjectAPIKeys::new(self.client, project_id)
    }

    // call [ProjectRateLimits] group APIs
    pub fn rate_limits(&self, project_id: &str) -> ProjectRateLimits<'_, C> {
        ProjectRateLimits::new(self.client, project_id)
    }

    // call [ProjectCertificates] group APIs
    pub fn certificates(&self, project_id: &str) -> ProjectCertificates<'_, C> {
        ProjectCertificates::new(self.client, project_id)
    }

    // call [ProjectGroups] group APIs
    pub fn groups(&self, project_id: &str) -> ProjectGroups<'_, C> {
        ProjectGroups::new(self.client, project_id)
    }

    // call [ProjectRoles] group APIs
    pub fn roles(&self, project_id: &str) -> ProjectRoles<'_, C> {
        ProjectRoles::new(self.client, project_id)
    }

    // call [ProjectUserRoles] group APIs
    pub fn user_roles(&self, project_id: &str, user_id: &str) -> ProjectUserRoles<'_, C> {
        ProjectUserRoles::new(self.client, project_id, user_id)
    }

    // call [ProjectGroupRoles] group APIs
    pub fn group_roles(&self, project_id: &str, group_id: &str) -> ProjectGroupRoles<'_, C> {
        ProjectGroupRoles::new(self.client, project_id, group_id)
    }

    /// Returns a list of projects.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectListResponse, OpenAIError> {
        self.client
            .get("/organization/projects", &self.request_options)
            .await
    }

    /// Create a new project in the organization. Projects can be created and archived, but cannot be deleted.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: ProjectCreateRequest) -> Result<Project, OpenAIError> {
        self.client
            .post("/organization/projects", request, &self.request_options)
            .await
    }

    /// Retrieves a project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, project_id: String) -> Result<Project, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{project_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Modifies a project in the organization.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn modify(
        &self,
        project_id: String,
        request: ProjectUpdateRequest,
    ) -> Result<Project, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{project_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Archives a project in the organization. Archived projects cannot be used or updated.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn archive(&self, project_id: String) -> Result<Project, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{project_id}/archive").as_str(),
                (),
                &self.request_options,
            )
            .await
    }
}
