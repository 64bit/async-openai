use crate::{
    config::Config,
    error::OpenAIError,
    types::batches::{Batch, BatchRequest, ListBatchesResponse},
    Client, RequestOptions,
};

/// Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
///
/// Related guide: [Batch](https://platform.openai.com/docs/guides/batch)
pub struct Batches<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Batches<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates and executes a batch from an uploaded file of requests
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: BatchRequest) -> Result<Batch, OpenAIError> {
        self.client
            .post("/batches", request, &self.request_options)
            .await
    }

    /// List your organization's batches.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListBatchesResponse, OpenAIError> {
        self.client.get("/batches", &self.request_options).await
    }

    /// Retrieves a batch.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        self.client
            .get(&format!("/batches/{batch_id}"), &self.request_options)
            .await
    }

    /// Cancels an in-progress batch. The batch will be in status `cancelling` for up to 10 minutes, before changing to `cancelled`, where it will have partial results (if any) available in the output file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        self.client
            .post(
                &format!("/batches/{batch_id}/cancel"),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }
}
