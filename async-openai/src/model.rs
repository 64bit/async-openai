use crate::{
    error::OpenAIError,
    types::{DeleteModelResponse, ListModelResponse, Model},
    Client,
};

/// List and describe the various models available in the API.
/// You can refer to the [Models](https://platform.openai.com/docs/models) documentation to understand what
/// models are available and the differences between them.
pub struct Models<'c> {
    client: &'c Client,
}

impl<'c> Models<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Lists the currently available models, and provides basic information
    /// about each one such as the owner and availability.
    pub async fn list(&self) -> Result<ListModelResponse, OpenAIError> {
        self.client.get("/models").await
    }

    /// Retrieves a model instance, providing basic information about the model
    /// such as the owner and permissioning.
    pub async fn retrieve(&self, id: &str) -> Result<Model, OpenAIError> {
        self.client.get(format!("/models/{id}").as_str()).await
    }

    /// Delete a fine-tuned model. You must have the Owner role in your organization.
    pub async fn delete(&self, model: &str) -> Result<DeleteModelResponse, OpenAIError> {
        self.client
            .delete(format!("/models/{model}").as_str())
            .await
    }
}
