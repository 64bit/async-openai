use crate::{
    config::Config,
    error::OpenAIError,
    types::chatkit::{
        ChatSessionResource, CreateChatSessionBody, DeletedThreadResource, ThreadItemListResource,
        ThreadListResource, ThreadResource,
    },
    Client, RequestOptions,
};

/// ChatKit API for managing sessions and threads.
///
/// Related guide: [ChatKit](https://platform.openai.com/docs/api-reference/chatkit)
pub struct Chatkit<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Chatkit<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
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
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ChatkitSessions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create a ChatKit session.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateChatSessionBody,
    ) -> Result<ChatSessionResource, OpenAIError> {
        self.client
            .post("/chatkit/sessions", request, &self.request_options)
            .await
    }

    /// Cancel a ChatKit session.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, session_id: &str) -> Result<ChatSessionResource, OpenAIError> {
        self.client
            .post(
                &format!("/chatkit/sessions/{session_id}/cancel"),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }
}

/// ChatKit threads API.
pub struct ChatkitThreads<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ChatkitThreads<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// List ChatKit threads.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ThreadListResource, OpenAIError> {
        self.client
            .get("/chatkit/threads", &self.request_options)
            .await
    }

    /// Retrieve a ChatKit thread.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, thread_id: &str) -> Result<ThreadResource, OpenAIError> {
        self.client
            .get(
                &format!("/chatkit/threads/{thread_id}"),
                &self.request_options,
            )
            .await
    }

    /// Delete a ChatKit thread.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, thread_id: &str) -> Result<DeletedThreadResource, OpenAIError> {
        self.client
            .delete(
                &format!("/chatkit/threads/{thread_id}"),
                &self.request_options,
            )
            .await
    }

    /// List ChatKit thread items.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn list_items(&self, thread_id: &str) -> Result<ThreadItemListResource, OpenAIError> {
        self.client
            .get(
                &format!("/chatkit/threads/{thread_id}/items"),
                &self.request_options,
            )
            .await
    }
}
