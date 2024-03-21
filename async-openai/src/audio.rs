use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateSpeechRequest, CreateSpeechResponse, CreateTranscriptionRequest,
        CreateTranscriptionResponse, CreateTranslationRequest, CreateTranslationResponse,
    },
    Client,
};
use crate::types::AudioResponseFormat;

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
        let request_format = request.response_format.unwrap_or(AudioResponseFormat::Json);
        let mut is_json = false;
        match request_format {
            AudioResponseFormat::Json => {
                is_json = true;
            }
            AudioResponseFormat::Text => {}
            AudioResponseFormat::Srt => {}
            AudioResponseFormat::VerboseJson => {
                is_json = true;
            }
            AudioResponseFormat::Vtt => {}
        }

        if !is_json {
            let bytes = self.client.post_form_return_bytes("/audio/transcriptions", request).await?;
            let text = String::from_utf8_lossy(&bytes[..]);
            let response = CreateTranscriptionResponse {
                text: text.to_string(),
            };
            return Ok(response);
        }

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
}
