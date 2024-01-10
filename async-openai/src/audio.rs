use futures::{stream::StreamExt, Stream};
use std::pin::Pin;
use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateSpeechRequest, CreateSpeechResponse, CreateTranscriptionRequest,
        CreateTranscriptionResponse, CreateTranslationRequest, CreateTranslationResponse,
        SpeechResponseStream
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

    /// Generates audio from the input text.
    pub async fn speech(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let bytes = self.client.post_raw("/audio/speech", request).await?;

        Ok(CreateSpeechResponse { bytes })
    }

    /// Generates audio from the input text.
    pub async fn speech_stream(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<SpeechResponseStream, OpenAIError> {

        let stream = Box::pin(self.client.post_raw_stream("/audio/speech", request)
            .await?
            .map(|stream_item| stream_item.map(|b| CreateSpeechResponse { bytes: b })));
        Ok(stream)
    }
}
