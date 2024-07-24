use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
    },
    util::merge_objects,
    Client,
};

/// Given a list of messages comprising a conversation, the model will return a response.
///
/// Related guide: [Chat completions](https://platform.openai.com//docs/guides/text-generation)
pub struct Chat<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Chat<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a model response for the given chat conversation.
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

    /// Same as [`create`] but with extra arguments.
    pub async fn create_with_extras<E>(
        &self,
        request: CreateChatCompletionRequest,
        extras: E,
    ) -> Result<CreateChatCompletionResponse, OpenAIError>
    where
        E: Serialize,
    {
        if matches!(request.stream, Some(true)) {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Chat::create_stream".into(),
            ));
        }

        let request = merge_objects(request, extras)?;

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

    /// Same as [`create_stream`] but with extra arguments.
    pub async fn create_stream_with_extras<E>(
        &self,
        mut request: CreateChatCompletionRequest,
        extras: E,
    ) -> Result<ChatCompletionResponseStream, OpenAIError>
    where
        E: Serialize,
    {
        if matches!(request.stream, Some(false)) {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Chat::create".into(),
            ));
        }

        request.stream = Some(true);

        let request = merge_objects(request, extras)?;

        Ok(self.client.post_stream("/chat/completions", request).await)
    }
}
