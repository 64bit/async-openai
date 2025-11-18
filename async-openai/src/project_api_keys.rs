use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::project_api_keys::{
        ProjectApiKey, ProjectApiKeyDeleteResponse, ProjectApiKeyListResponse,
    },
    Client, RequestOptions,
};

/// Manage API keys for a given project. Supports listing and deleting keys for users.
/// This API does not allow issuing keys for users, as users need to authorize themselves to generate keys.
pub struct ProjectAPIKeys<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectAPIKeys<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Returns a list of API keys in the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectApiKeyListResponse, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/api_keys", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Retrieves an API key in the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, api_key: &str) -> Result<ProjectApiKey, OpenAIError> {
        self.client
            .get(
                format!(
                    "/organization/projects/{}/api_keys/{api_key}",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }

    /// Deletes an API key from the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, api_key: &str) -> Result<ProjectApiKeyDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!(
                    "/organization/projects/{}/api_keys/{api_key}",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }
}
