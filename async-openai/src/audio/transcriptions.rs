use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{
        CreateTranscriptionRequest, CreateTranscriptionResponseDiarizedJson,
        CreateTranscriptionResponseJson, CreateTranscriptionResponseVerboseJson,
        TranscriptionResponseStream,
    },
    Client, RequestOptions,
};

pub struct Transcriptions<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Transcriptions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request, &self.request_options)
            .await
    }

    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static, reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>"
    )]
    #[allow(unused_mut)]
    pub async fn create_stream(
        &self,
        mut request: CreateTranscriptionRequest,
    ) -> Result<TranscriptionResponseStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if let Some(stream) = request.stream {
                if !stream {
                    return Err(OpenAIError::InvalidArgument(
                        "When stream is not true, use Audio::transcribe".into(),
                    ));
                }
            }
            request.stream = Some(true);
        }

        self.client
            .post_form_stream("/audio/transcriptions", request, &self.request_options)
            .await
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create_verbose_json(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseVerboseJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request, &self.request_options)
            .await
    }

    /// Transcribes audio into the input language.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create_diarized_json(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<CreateTranscriptionResponseDiarizedJson, OpenAIError> {
        self.client
            .post_form("/audio/transcriptions", request, &self.request_options)
            .await
    }

    /// Transcribes audio into the input language.
    pub async fn create_raw(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .post_form_raw("/audio/transcriptions", request, &self.request_options)
            .await?;
        Ok(bytes)
    }
}
