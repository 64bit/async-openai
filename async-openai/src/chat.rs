use crate::{
    client::Client,
    error::OpenAIError,
    types::{ChatResponseStream, CreateChatRequest, CreateChatResponse},
};

/// Given a series of messages, the model will return one or more predicted
/// completion messages.
pub struct Chat<'c> {
    client: &'c Client,
}

impl<'c> Chat<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a completion for the provided messages and parameters
    pub async fn create(
        &self,
        request: CreateChatRequest,
    ) -> Result<CreateChatResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Chat::create_stream".into(),
            ));
        }
        self.client.post("/chat/completions", request).await
    }

    /// Creates a completion request for the provided messages and parameters
    ///
    /// Stream back partial progress. Tokens will be sent as data-only
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
    /// as they become available, with the stream terminated by a data: \[DONE\] message.
    ///
    /// [ChatResponseStream] is a parsed SSE stream until a \[DONE\] is received from server.
    pub async fn create_stream(
        &self,
        mut request: CreateChatRequest,
    ) -> Result<ChatResponseStream, OpenAIError> {
        if request.stream.is_some() && !request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Chat::create".into(),
            ));
        }

        request.stream = Some(true);

        Ok(self.client.post_stream("/chat/completions", request).await)
    }
}
