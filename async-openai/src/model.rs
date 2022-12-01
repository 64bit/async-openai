use crate::{
    error::OpenAIError,
    types::{ListModelResponse, Model},
    Client,
};

pub struct Models;

impl Models {
    pub async fn list(client: &Client) -> Result<ListModelResponse, OpenAIError> {
        client.get("/models").await
    }

    pub async fn retrieve(client: &Client, id: &str) -> Result<Model, OpenAIError> {
        client.get(format!("/models/{id}").as_str()).await
    }
}
