use crate::{
    config::Config, error::OpenAIError, types::admin::external_keys::*, Client, RequestOptions,
};
/// Manage organization external keys.
pub struct ExternalKeys<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> ExternalKeys<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create external keys.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: V1OrganizationExternalKeysPostRequest,
    ) -> Result<ExternalKeyResource, OpenAIError> {
        self.client
            .post(
                "/organization/external_keys",
                request,
                &self.request_options,
            )
            .await
    }

    /// List external keys.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListExternalKeysResource, OpenAIError> {
        self.client
            .get("/organization/external_keys", &self.request_options)
            .await
    }

    /// Retrieve external keys.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, id: &str) -> Result<ExternalKeyResource, OpenAIError> {
        self.client
            .get(
                &format!("/organization/external_keys/{id}"),
                &self.request_options,
            )
            .await
    }

    /// Delete external keys.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, id: &str) -> Result<serde_json::Value, OpenAIError> {
        self.client
            .delete(
                &format!("/organization/external_keys/{id}"),
                &self.request_options,
            )
            .await
    }

    /// Validate external keys.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn validate(&self, id: &str) -> Result<ValidateExternalKeyResource, OpenAIError> {
        self.client
            .post(
                &format!("/organization/external_keys/{id}/validate"),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }
}
