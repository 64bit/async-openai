use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{CreateSpeechRequest, CreateSpeechResponse, SpeechResponseStream},
    Client, RequestOptions,
};

pub struct Speech<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Speech<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Generates audio from the input text.
    pub async fn create(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .post_raw("/audio/speech", request, &self.request_options)
            .await?;

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
    pub async fn create_stream(
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
        Ok(self
            .client
            .post_stream("/audio/speech", request, &self.request_options)
            .await)
    }
}
