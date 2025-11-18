use crate::{
    config::Config,
    error::OpenAIError,
    types::vectorstores::{
        CreateVectorStoreFileBatchRequest, ListVectorStoreFilesResponse, VectorStoreFileBatchObject,
    },
    Client, RequestOptions,
};

/// Vector store file batches represent operations to add multiple files to a vector store.
///
/// Related guide: [File Search](https://platform.openai.com/docs/assistants/tools/file-search)
pub struct VectorStoreFileBatches<'c, C: Config> {
    client: &'c Client<C>,
    pub vector_store_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> VectorStoreFileBatches<'c, C> {
    pub fn new(client: &'c Client<C>, vector_store_id: &str) -> Self {
        Self {
            client,
            vector_store_id: vector_store_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Create vector store file batch
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateVectorStoreFileBatchRequest,
    ) -> Result<VectorStoreFileBatchObject, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{}/file_batches", &self.vector_store_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieves a vector store file batch.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        batch_id: &str,
    ) -> Result<VectorStoreFileBatchObject, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/vector_stores/{}/file_batches/{batch_id}",
                    &self.vector_store_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Cancel a vector store file batch. This attempts to cancel the processing of files in this batch as soon as possible.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, batch_id: &str) -> Result<VectorStoreFileBatchObject, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/vector_stores/{}/file_batches/{batch_id}/cancel",
                    &self.vector_store_id
                ),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of vector store files in a batch.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn list_files(
        &self,
        batch_id: &str,
    ) -> Result<ListVectorStoreFilesResponse, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/vector_stores/{}/file_batches/{batch_id}/files",
                    &self.vector_store_id
                ),
                &self.request_options,
            )
            .await
    }
}
