use crate::{
    error::OpenAIError,
    types::{CreateModerationRequest, CreateModerationResponse},
    Client,
};

/// Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
///
/// Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation/overview)
pub struct Moderations<'c> {
    client: &'c Client,
}

impl<'c> Moderations<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Classifies if text violates OpenAI's Content Policy
    pub async fn create(
        &self,
        request: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, OpenAIError> {
        self.client.post("/moderations", request).await
    }
}
