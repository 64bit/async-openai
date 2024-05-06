use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateModerationRequest, CreateModerationResponse},
    Client,
};

/// Given some input text, outputs if the model classifies it as potentially harmful across several categories.
///
/// Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation)
pub struct Moderations<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Moderations<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Classifies if text is potentially harmful.
    pub async fn create(
        &self,
        request: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, OpenAIError> {
        self.client.post("/moderations", request).await
    }
}
