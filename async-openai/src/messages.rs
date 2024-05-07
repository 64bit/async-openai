use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateMessageRequest, DeleteMessageResponse, ListMessagesResponse, MessageObject,
        ModifyMessageRequest,
    },
    Client, MessageFiles,
};

/// Represents a message within a [thread](https://platform.openai.com/docs/api-reference/threads).
pub struct Messages<'c, C: Config> {
    ///  The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) to create a message for.
    pub thread_id: String,
    client: &'c Client<C>,
}

impl<'c, C: Config> Messages<'c, C> {
    pub fn new(client: &'c Client<C>, thread_id: &str) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }

    /// Call [MessageFiles] API group
    pub fn files(&self, message_id: &str) -> MessageFiles<C> {
        MessageFiles::new(self.client, &self.thread_id, message_id)
    }

    /// Create a message.
    pub async fn create(
        &self,
        request: CreateMessageRequest,
    ) -> Result<MessageObject, OpenAIError> {
        self.client
            .post(&format!("/threads/{}/messages", self.thread_id), request)
            .await
    }

    /// Retrieve a message.
    pub async fn retrieve(&self, message_id: &str) -> Result<MessageObject, OpenAIError> {
        self.client
            .get(&format!(
                "/threads/{}/messages/{message_id}",
                self.thread_id
            ))
            .await
    }

    /// Modifies a message.
    pub async fn update(
        &self,
        message_id: &str,
        request: ModifyMessageRequest,
    ) -> Result<MessageObject, OpenAIError> {
        self.client
            .post(
                &format!("/threads/{}/messages/{message_id}", self.thread_id),
                request,
            )
            .await
    }

    /// Returns a list of messages for a given thread.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListMessagesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(&format!("/threads/{}/messages", self.thread_id), query)
            .await
    }

    pub async fn delete(&self, message_id: &str) -> Result<DeleteMessageResponse, OpenAIError> {
        self.client
            .delete(&format!(
                "/threads/{}/messages/{message_id}",
                self.thread_id
            ))
            .await
    }
}
