use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateResponseRequest, CreateResponseResponse},
    Client,
};

/// Given text input or a list of context items, the model will generate a response.
///
/// Related guide: [Responses API](https://platform.openai.com/docs/guides/responses)
pub struct Responses<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Responses<'c, C> {
    /// Constructs a new Responses client.
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a model response for the given input.
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn create(
        &self,
        request: CreateResponseRequest,
    ) -> Result<CreateResponseResponse, OpenAIError> {
        self.client.post("/responses", request).await
    }
}
