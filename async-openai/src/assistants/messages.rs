use crate::{
    config::Config,
    error::OpenAIError,
    types::assistants::{
        CreateMessageRequest, DeleteMessageResponse, ListMessagesResponse, MessageObject,
        ModifyMessageRequest,
    },
    Client, RequestOptions,
};

/// Represents a message within a [thread](https://platform.openai.com/docs/api-reference/threads).
pub struct Messages<'c, C: Config> {
    ///  The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) to create a message for.
    pub thread_id: String,
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Messages<'c, C> {
    pub fn new(client: &'c Client<C>, thread_id: &str) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Create a message.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateMessageRequest,
    ) -> Result<MessageObject, OpenAIError> {
        self.client
            .post(
                &format!("/threads/{}/messages", self.thread_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve a message.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, message_id: &str) -> Result<MessageObject, OpenAIError> {
        self.client
            .get(
                &format!("/threads/{}/messages/{message_id}", self.thread_id),
                &self.request_options,
            )
            .await
    }

    /// Modifies a message.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        message_id: &str,
        request: ModifyMessageRequest,
    ) -> Result<MessageObject, OpenAIError> {
        self.client
            .post(
                &format!("/threads/{}/messages/{message_id}", self.thread_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Returns a list of messages for a given thread.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListMessagesResponse, OpenAIError> {
        self.client
            .get(
                &format!("/threads/{}/messages", self.thread_id),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, message_id: &str) -> Result<DeleteMessageResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/threads/{}/messages/{message_id}", self.thread_id),
                &self.request_options,
            )
            .await
    }
}
