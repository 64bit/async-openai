use crate::{
    client::Client,
    error::OpenAIError,
    types::{CreateCompletionRequest, CreateCompletionResponse},
};

/// Given a prompt, the model will return one or more predicted
/// completions, and can also return the probabilities of alternative
/// tokens at each position.
pub struct Completion;

impl Completion {
    /// Creates a completion for the provided prompt and parameters
    pub async fn create(
        client: &Client,
        request: CreateCompletionRequest,
    ) -> Result<CreateCompletionResponse, OpenAIError> {
        client.post("/completions", request).await
    }
}
