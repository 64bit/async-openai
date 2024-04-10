use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateSpeechRequest, CreateSpeechResponse, CreateTranscriptionRequest,
        CreateTranscriptionResponseJson, CreateTranscriptionResponseVerboseJson,
        CreateTranslationRequest, CreateTranslationResponse,
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
    ) -> Result<CreateTranscriptionResponseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request)
            .await
    }

    /// Transcribes audio into the input language.
    pub async fn transcribe_verbose_json(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseVerboseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request)
            .await
    }

    /// Transcribes audio into the input language.
    pub async fn transcribe_raw(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<Bytes, OpenAIError> {
        self.client
            .post_form_raw("/audio/transcriptions", request)
            .await
    }

    /// Translates audio into into English.
    pub async fn translate(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponse, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }

    /// Generates audio from the input text.
    pub async fn speech(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let bytes = self.client.post_raw("/audio/speech", request).await?;

        Ok(CreateSpeechResponse { bytes })
    }
}
