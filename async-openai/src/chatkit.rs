use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::chatkit::{
        ChatSessionResource, CreateChatSessionBody, DeletedThreadResource, ThreadItemListResource,
        ThreadListResource, ThreadResource,
    },
    Client,
};

/// ChatKit API for managing sessions and threads.
///
/// Related guide: [ChatKit](https://platform.openai.com/docs/api-reference/chatkit)
pub struct Chatkit<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Chatkit<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Access sessions API.
    pub fn sessions(&self) -> ChatkitSessions<'_, C> {
        ChatkitSessions::new(self.client)
    }

    /// Access threads API.
    pub fn threads(&self) -> ChatkitThreads<'_, C> {
        ChatkitThreads::new(self.client)
    }
}

/// ChatKit sessions API.
pub struct ChatkitSessions<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> ChatkitSessions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Create a ChatKit session.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateChatSessionBody,
    ) -> Result<ChatSessionResource, OpenAIError> {
        self.client.post("/chatkit/sessions", request).await
    }

    /// Cancel a ChatKit session.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, session_id: &str) -> Result<ChatSessionResource, OpenAIError> {
        self.client
            .post(
                &format!("/chatkit/sessions/{session_id}/cancel"),
                serde_json::json!({}),
            )
            .await
    }
}

/// ChatKit threads API.
pub struct ChatkitThreads<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> ChatkitThreads<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// List ChatKit threads.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ThreadListResource, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/chatkit/threads", &query).await
    }

    /// Retrieve a ChatKit thread.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, thread_id: &str) -> Result<ThreadResource, OpenAIError> {
        self.client
            .get(&format!("/chatkit/threads/{thread_id}"))
            .await
    }

    /// Delete a ChatKit thread.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, thread_id: &str) -> Result<DeletedThreadResource, OpenAIError> {
        self.client
            .delete(&format!("/chatkit/threads/{thread_id}"))
            .await
    }

    /// List ChatKit thread items.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list_items<Q>(
        &self,
        thread_id: &str,
        query: &Q,
    ) -> Result<ThreadItemListResource, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(&format!("/chatkit/threads/{thread_id}/items"), &query)
            .await
    }
}
