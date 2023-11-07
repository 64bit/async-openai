use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{ListMessageFilesResponse, MessageFileObject},
    Client,
};

/// Files attached to a message.
pub struct MessageFiles<'c, C: Config> {
    client: &'c Client<C>,
    pub thread_id: String,
    pub message_id: String,
}

impl<'c, C: Config> MessageFiles<'c, C> {
    pub fn new(client: &'c Client<C>, thread_id: &str, message_id: &str) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            message_id: message_id.into(),
        }
    }

    /// Retrieves a message file.
    pub async fn retrieve(&self, file_id: &str) -> Result<MessageFileObject, OpenAIError> {
        self.client
            .get(&format!(
                "/threads/{}/messages/{}/files/{file_id}",
                self.thread_id, self.message_id
            ))
            .await
    }

    /// Returns a list of message files.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListMessageFilesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                &format!(
                    "/threads/{}/messages/{}/files",
                    self.thread_id, self.message_id
                ),
                query,
            )
            .await
    }
}
