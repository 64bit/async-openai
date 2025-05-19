use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateTokenizeRequest, CreateTokenizeResponse},
    Client,
};

/// Given chat or completion requests, the model will return the tokens information
/// pertaining to the request. Only useful if the underlying API server implements
/// /tokenize endpoint.
///
/// Related guide: [Tokenize](https://docs.vllm.ai/en/latest/serving/openai_compatible_server.html#tokenizer-api)
pub struct Tokenize<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Tokenize<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Create a tokenization for the given request
    ///
    /// byot: You must ensure "stream: false" in serialized `request`
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn create(
        &self,
        request: CreateTokenizeRequest,
    ) -> Result<CreateTokenizeResponse, OpenAIError> {
        self.client.post("/tokenize", request).await
    }
}
