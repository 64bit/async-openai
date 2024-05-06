use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{Batch, BatchRequest, ListBatchesResponse},
    Client,
};

/// Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
///
/// Related guide: [Batch](https://platform.openai.com/docs/guides/batch)
pub struct Batches<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Batches<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates and executes a batch from an uploaded file of requests
    pub async fn create(&self, request: BatchRequest) -> Result<Batch, OpenAIError> {
        self.client.post("/batches", request).await
    }

    /// List your organization's batches.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListBatchesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/batches", query).await
    }

    /// Retrieves a batch.
    pub async fn retrieve(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        self.client.get(&format!("/batches/{batch_id}")).await
    }

    /// Cancels an in-progress batch.
    pub async fn cancel(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        self.client
            .post(
                &format!("/batches/{batch_id}/cancel"),
                serde_json::json!({}),
            )
            .await
    }
}
