use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateThreadRequest, DeleteThreadResponse, ModifyThreadRequest, ThreadObject},
    Client, Messages,
};

/// Create threads that assistants can interact with.
///
/// Related guide: [Assistants](https://platform.openai.com/docs/assistants/overview)
pub struct Threads<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Threads<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Call [Messages] group API to manage message in [thread_id] thread.
    pub fn messages(&self, thread_id: &str) -> Messages<C> {
        Messages::new(&self.client, thread_id)
    }

    /// Create a thread.
    pub async fn create(&self, request: CreateThreadRequest) -> Result<ThreadObject, OpenAIError> {
        self.client.post("/threads", request).await
    }

    /// Retrieves a thread.
    pub async fn retrieve(&self, thread_id: &str) -> Result<ThreadObject, OpenAIError> {
        self.client.get(&format!("/threads/{thread_id}")).await
    }

    /// Modifies a thread.
    pub async fn update(
        &self,
        thread_id: &str,
        request: ModifyThreadRequest,
    ) -> Result<ThreadObject, OpenAIError> {
        self.client
            .post(&format!("/threads/{thread_id}"), request)
            .await
    }

    /// Delete a thread.
    pub async fn delete(&self, thread_id: &str) -> Result<DeleteThreadResponse, OpenAIError> {
        self.client.delete(&format!("/threads/{thread_id}")).await
    }
}
