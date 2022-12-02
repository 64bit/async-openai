use crate::{
    error::OpenAIError,
    types::{ListModelResponse, Model},
    Client,
};

/// List and describe the various models available in the API.
/// You can refer to the [Models](https://beta.openai.com/docs/models) documentation to understand what
/// models are available and the differences between them.
pub struct Models;

impl Models {
    /// Lists the currently available models, and provides basic information
    /// about each one such as the owner and availability.
    pub async fn list(client: &Client) -> Result<ListModelResponse, OpenAIError> {
        client.get("/models").await
    }

    /// Retrieves a model instance, providing basic information about the model
    /// such as the owner and permissioning.
    pub async fn retrieve(client: &Client, id: &str) -> Result<Model, OpenAIError> {
        client.get(format!("/models/{id}").as_str()).await
    }
}
