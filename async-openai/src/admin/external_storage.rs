use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::external_storage::{
        CreateExternalStorageBody, ExternalStorageDeletedResource, ExternalStorageListResource,
        ExternalStorageResponse,
    },
    Client, RequestOptions,
};

/// Manage organization external storage configurations.
pub struct ExternalStorage<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ExternalStorage<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// List external storage configurations.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ExternalStorageListResource, OpenAIError> {
        self.client
            .get("/organization/external_storage", &self.request_options)
            .await
    }

    /// Create an external storage configuration.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateExternalStorageBody,
    ) -> Result<ExternalStorageResponse, OpenAIError> {
        self.client
            .post(
                "/organization/external_storage",
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve an external storage configuration.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, id: &str) -> Result<ExternalStorageResponse, OpenAIError> {
        self.client
            .get(
                &format!("/organization/external_storage/{id}"),
                &self.request_options,
            )
            .await
    }

    /// Delete an external storage configuration.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, id: &str) -> Result<ExternalStorageDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!("/organization/external_storage/{id}"),
                &self.request_options,
            )
            .await
    }

    /// Validate an external storage configuration.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn validate(&self, id: &str) -> Result<ExternalStorageResponse, OpenAIError> {
        self.client
            .post(
                &format!("/organization/external_storage/{id}/validate"),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }
}
