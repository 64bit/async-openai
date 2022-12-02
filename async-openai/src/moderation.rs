use crate::{
    error::OpenAIError,
    types::{CreateModerationRequest, CreateModerationResponse},
    Client,
};

/// Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
///
/// Related guide: [Moderations](https://beta.openai.com/docs/guides/moderation/overview)
pub struct Moderation;

impl Moderation {
    /// Classifies if text violates OpenAI's Content Policy
    pub async fn create(
        client: &Client,
        request: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, OpenAIError> {
        client.post("/moderations", request).await
    }
}
