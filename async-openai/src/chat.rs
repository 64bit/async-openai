use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionResponse,
    },
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

        // Добавляем недостающие поля в request
        let mut req_json = serde_json::to_value(&request).unwrap();
        let additional_fields = serde_json::json!({
            "chat_id": "53f6fe83-3087-4ea9-a328-26a40dd9543a",
            "id": "b0780fb9-8824-4ede-b233-226610a95a0d",
            "session_id": "wkIeoXONhyglgGGaAAAN",
            "stream": true,
            "model": "normative_agent_pipeline_ragas"
        });

        if let serde_json::Value::Object(ref mut map) = req_json {
            if let serde_json::Value::Object(additional_map) = additional_fields {
                map.extend(additional_map);
            }
        }

        println!("{}", req_json);

        Ok(self
            .client
            .post_stream("/api/chat/completions", req_json)
            .await)
    }
}
