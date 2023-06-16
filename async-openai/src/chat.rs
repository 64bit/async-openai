use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
    },
    Client,
};

/// Given a chat conversation, the model will return a chat completion response.
pub struct Chat<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Chat<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a completion for the chat message
    pub async fn create(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Chat::create_stream".into(),
            ));
        }
        self.client.post("/chat/completions", request).await
    }

    /// Creates a completion for the chat message
    ///
    /// partial message deltas will be sent, like in ChatGPT. Tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message.
    ///
    /// [ChatCompletionResponseStream] is a parsed SSE stream until a \[DONE\] is received from server.
    pub async fn create_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        if request.stream.is_some() && !request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Chat::create".into(),
            ));
        }

        request.stream = Some(true);

        Ok(self.client.post_stream("/chat/completions", request).await)
    }
}
