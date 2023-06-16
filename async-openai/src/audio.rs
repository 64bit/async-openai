use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateTranscriptionRequest, CreateTranscriptionResponse, CreateTranslationRequest,
        CreateTranslationResponse,
    },
    util::create_file_part,
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
        let audio_part = create_file_part(&request.file.path).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", audio_part)
            .text("model", request.model);

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string())
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string())
        }

        self.client.post_form("/audio/transcriptions", form).await
    }

    /// Translates audio into into English.
    pub async fn translate(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponse, OpenAIError> {
        let audio_part = create_file_part(&request.file.path).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", audio_part)
            .text("model", request.model);

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string())
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string())
        }

        self.client.post_form("/audio/translations", form).await
    }
}
