use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::project_service_accounts::{
        ProjectServiceAccount, ProjectServiceAccountCreateRequest,
        ProjectServiceAccountCreateResponse, ProjectServiceAccountDeleteResponse,
        ProjectServiceAccountListResponse,
    },
    Client, RequestOptions,
};

/// Manage service accounts within a project. A service account is a bot user that is not
/// associated with a user. If a user leaves an organization, their keys and membership in projects
/// will no longer work. Service accounts do not have this limitation.
/// However, service accounts can also be deleted from a project.
pub struct ProjectServiceAccounts<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectServiceAccounts<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns a list of service accounts in the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectServiceAccountListResponse, OpenAIError> {
        self.client
            .get(
                format!(
                    "/organization/projects/{}/service_accounts",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }

    /// Creates a new service account in the project. This also returns an unredacted API key for the service account.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: ProjectServiceAccountCreateRequest,
    ) -> Result<ProjectServiceAccountCreateResponse, OpenAIError> {
        self.client
            .post(
                format!(
                    "/organization/projects/{}/service_accounts",
                    self.project_id
                )
                .as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieves a service account in the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        service_account_id: &str,
    ) -> Result<ProjectServiceAccount, OpenAIError> {
        self.client
            .get(
                format!(
                    "/organization/projects/{}/service_accounts/{service_account_id}",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }

    /// Deletes a service account from the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        service_account_id: &str,
    ) -> Result<ProjectServiceAccountDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!(
                    "/organization/projects/{}/service_accounts/{service_account_id}",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }
}
