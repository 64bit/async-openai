use crate::{
    client::Client,
    error::OpenAIError,
    types::{CreateCompletionRequest, CreateCompletionResponse},
};

pub struct Completion;

impl Completion {
    pub async fn create(
        client: &Client,
        request: CreateCompletionRequest,
    ) -> Result<CreateCompletionResponse, OpenAIError> {
        client.post("/completions", request).await
    }
}
