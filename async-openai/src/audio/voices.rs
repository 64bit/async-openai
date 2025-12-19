use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{CreateVoiceRequest, VoiceResource},
    Client, RequestOptions,
};

/// Voice API group
pub struct Voices<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Voices<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates a custom voice.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(&self, request: CreateVoiceRequest) -> Result<VoiceResource, OpenAIError> {
        self.client
            .post_form("/audio/voices", request, &self.request_options)
            .await
    }
}
