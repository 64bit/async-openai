use crate::{
    config::Config,
    error::OpenAIError,
    types::chat::{
        ChatCompletionDeleted, ChatCompletionList, ChatCompletionMessageList,
        ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
        UpdateChatCompletionRequest,
    },
    Client, RequestOptions,
};

/// Given a list of messages comprising a conversation, the model will return a response.
///
/// Related guide: [Chat Completions](https://platform.openai.com/docs/guides/text-generation)
pub struct Chat<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Chat<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates a model response for the given chat conversation.
    ///
    /// Returns a [chat completion](https://platform.openai.com/docs/api-reference/chat/object) object, or a streamed sequence of [chat completion chunk](https://platform.openai.com/docs/api-reference/chat/streaming) objects if the request is streamed.
    ///
    /// Learn more in the [text generation](https://platform.openai.com/docs/guides/text-generation), [vision](https://platform.openai.com/docs/guides/vision), and [audio](https://platform.openai.com/docs/guides/audio) guides.
    ///
    /// Parameter support can differ depending on the model used to generate the response, particularly for newer reasoning models. Parameters that are only supported for reasoning models are noted below. For the current state of unsupported parameters in reasoning models, [refer to the reasoning guide](https://platform.openai.com/docs/guides/reasoning).
    ///
    /// byot: You must ensure "stream: false" in serialized `request`
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn create(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if request.stream.is_some() && request.stream.unwrap() {
                return Err(OpenAIError::InvalidArgument(
                    "When stream is true, use Chat::create_stream".into(),
                ));
            }
        }
        self.client
            .post("/chat/completions", request, &self.request_options)
            .await
    }

    /// Creates a completion for the chat message.
    ///
    /// If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    ///
    /// See the [Streaming section](https://platform.openai.com/docs/api-reference/chat/streaming) for more information, along with the [streaming responses](https://platform.openai.com/docs/guides/streaming-responses) guide for more information on how to handle the streaming events.
    ///
    /// [ChatCompletionResponseStream] is a parsed SSE stream until a \[DONE\] is received from server.
    ///
    /// byot: You must ensure "stream: true" in serialized `request`
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static"
    )]
    #[allow(unused_mut)]
    pub async fn create_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletionResponseStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if request.stream.is_some() && !request.stream.unwrap() {
                return Err(OpenAIError::InvalidArgument(
                    "When stream is false, use Chat::create".into(),
                ));
            }

            request.stream = Some(true);
        }
        Ok(self
            .client
            .post_stream("/chat/completions", request, &self.request_options)
            .await)
    }

    /// List stored Chat Completions. Only Chat Completions that have been stored
    /// with the `store` parameter set to `true` will be returned.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ChatCompletionList, OpenAIError> {
        self.client
            .get("/chat/completions", &self.request_options)
            .await
    }

    /// Get a stored chat completion. Only Chat Completions that have been created
    /// with the `store` parameter set to `true` will be returned.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        completion_id: &str,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        self.client
            .get(
                &format!("/chat/completions/{completion_id}"),
                &self.request_options,
            )
            .await
    }

    /// Modify a stored chat completion. Only Chat Completions that have been
    /// created with the `store` parameter set to `true` can be modified. Currently,
    /// the only supported modification is to update the `metadata` field.
    #[crate::byot(
        T0 = std::fmt::Display,
        T1 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn update(
        &self,
        completion_id: &str,
        request: UpdateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        self.client
            .post(
                &format!("/chat/completions/{completion_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    /// Delete a stored chat completion. Only Chat Completions that have been
    /// created with the `store` parameter set to `true` can be deleted.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, completion_id: &str) -> Result<ChatCompletionDeleted, OpenAIError> {
        self.client
            .delete(
                &format!("/chat/completions/{completion_id}"),
                &self.request_options,
            )
            .await
    }

    /// Get a list of messages for the specified chat completion.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn messages(
        &self,
        completion_id: &str,
    ) -> Result<ChatCompletionMessageList, OpenAIError> {
        self.client
            .get(
                &format!("/chat/completions/{completion_id}/messages"),
                &self.request_options,
            )
            .await
    }
}
