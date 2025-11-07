use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{
        CreateSpeechRequest, CreateSpeechResponse, CreateTranscriptionRequest,
        CreateTranscriptionResponseDiarizedJson, CreateTranscriptionResponseJson,
        CreateTranscriptionResponseVerboseJson, CreateTranslationRequest,
        CreateTranslationResponseJson, CreateTranslationResponseVerboseJson, SpeechResponseStream,
    },
    Client,
};

/// Turn audio into text or text into audio.
/// Related guide: [Speech to text](https://platform.openai.com/docs/guides/speech-to-text)
pub struct Audio<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Audio<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn transcribe(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request)
            .await
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn transcribe_verbose_json(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseVerboseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request)
            .await
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn transcribe_diarized_json(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseDiarizedJson, OpenAIError> {
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

    /// Translates audio into English.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn translate(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponseJson, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }

    /// Translates audio into English.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn translate_verbose_json(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponseVerboseJson, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }

    /// Transcribes audio into the input language.
    pub async fn translate_raw(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<Bytes, OpenAIError> {
        self.client
            .post_form_raw("/audio/translations", request)
            .await
    }

    /// Generates audio from the input text.
    pub async fn speech(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let bytes = self.client.post_raw("/audio/speech", request).await?;

        Ok(CreateSpeechResponse { bytes })
    }

    /// Generates audio from the input text in SSE stream format.
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static"
    )]
    #[allow(unused_mut)]
    pub async fn speech_stream(
        &self,
        mut request: CreateSpeechRequest,
    ) -> Result<SpeechResponseStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            use crate::types::audio::StreamFormat;
            if let Some(stream_format) = request.stream_format {
                if stream_format != StreamFormat::SSE {
                    return Err(OpenAIError::InvalidArgument(
                        "When stream_format is not SSE, use Audio::speech".into(),
                    ));
                }
            }

            request.stream_format = Some(StreamFormat::SSE);
        }
        Ok(self.client.post_stream("/audio/speech", request).await)
    }
}
