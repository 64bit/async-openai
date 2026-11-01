use crate::{
    config::Config,
    error::OpenAIError,
    types::decisions::{DecisionRequest, DecisionResponse},
    Client, RequestOptions,
};

/// Classify or score inputs using the Decisions API.
pub struct Decisions<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> Decisions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }
    /// Answer classification or scoring questions about the input.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: DecisionRequest) -> Result<DecisionResponse, OpenAIError> {
        self.client
            .post("/decisions", request, &self.request_options)
            .await
    }
}
