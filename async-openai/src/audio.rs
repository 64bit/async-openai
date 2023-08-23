use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateTranscriptionRequest, CreateTranscriptionResponse, CreateTranslationRequest,
        CreateTranslationResponse,
    },
    Client,
};

/// Turn audio into text
/// Related guide: [Speech to text](https://platform.openai.com/docs/guides/speech-to-text)
pub struct Audio<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Audio<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Transcribes audio into the input language.
    pub async fn transcribe(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponse, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request)
            .await
    }

    /// Translates audio into into English.
    pub async fn translate(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponse, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }
}
