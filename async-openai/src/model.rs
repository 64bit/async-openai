use crate::{
    config::Config,
    error::OpenAIError,
    types::models::{DeleteModelResponse, ListModelResponse, Model},
    Client, RequestOptions,
};

/// List and describe the various models available in the API.
/// You can refer to the [Models](https://platform.openai.com/docs/models) documentation to understand what
/// models are available and the differences between them.
pub struct Models<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Models<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Lists the currently available models, and provides basic information
    /// about each one such as the owner and availability.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListModelResponse, OpenAIError> {
        self.client.get("/models", &self.request_options).await
    }

    /// Retrieves a model instance, providing basic information about the model
    /// such as the owner and permissioning.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, model: &str) -> Result<Model, OpenAIError> {
        self.client
            .get(format!("/models/{model}").as_str(), &self.request_options)
            .await
    }

    /// Delete a fine-tuned model. You must have the Owner role in your organization.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, model: &str) -> Result<DeleteModelResponse, OpenAIError> {
        self.client
            .delete(format!("/models/{model}").as_str(), &self.request_options)
            .await
    }
}
