use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::api_keys::{
        AdminApiKey, AdminApiKeyDeleteResponse, ApiKeyList, CreateAdminApiKeyRequest,
    },
    Client, RequestOptions,
};

/// Admin API keys enable Organization Owners to programmatically manage various aspects of their
/// organization, including users, projects, and API keys. These keys provide administrative capabilities,
/// allowing you to automate organization management tasks.
pub struct AdminAPIKeys<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> AdminAPIKeys<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// List all organization and project API keys.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ApiKeyList, OpenAIError> {
        self.client
            .get("/organization/admin_api_keys", &self.request_options)
            .await
    }

    /// Create an organization admin API key.
    pub async fn create(
        &self,
        request: CreateAdminApiKeyRequest,
    ) -> Result<AdminApiKey, OpenAIError> {
        self.client
            .post(
                "/organization/admin_api_keys",
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve a single organization API key.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, key_id: &str) -> Result<AdminApiKey, OpenAIError> {
        self.client
            .get(
                format!("/organization/admin_api_keys/{key_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Delete an organization admin API key.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, key_id: &str) -> Result<AdminApiKeyDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!("/organization/admin_api_keys/{key_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
