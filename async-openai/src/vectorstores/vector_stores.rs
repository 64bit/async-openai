use crate::{
    config::Config,
    error::OpenAIError,
    types::vectorstores::{
        CreateVectorStoreRequest, DeleteVectorStoreResponse, ListVectorStoresResponse,
        UpdateVectorStoreRequest, VectorStoreObject, VectorStoreSearchRequest,
        VectorStoreSearchResultsPage,
    },
    Client, RequestOptions, VectorStoreFileBatches, VectorStoreFiles,
};

pub struct VectorStores<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> VectorStores<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// [VectorStoreFiles] API group
    pub fn files(&self, vector_store_id: &str) -> VectorStoreFiles<'_, C> {
        VectorStoreFiles::new(self.client, vector_store_id)
    }

    /// [VectorStoreFileBatches] API group
    pub fn file_batches(&self, vector_store_id: &str) -> VectorStoreFileBatches<'_, C> {
        VectorStoreFileBatches::new(self.client, vector_store_id)
    }

    /// Create a vector store.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateVectorStoreRequest,
    ) -> Result<VectorStoreObject, OpenAIError> {
        self.client
            .post("/vector_stores", request, &self.request_options)
            .await
    }

    /// Retrieves a vector store.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, vector_store_id: &str) -> Result<VectorStoreObject, OpenAIError> {
        self.client
            .get(
                &format!("/vector_stores/{vector_store_id}"),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of vector stores.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListVectorStoresResponse, OpenAIError> {
        self.client
            .get("/vector_stores", &self.request_options)
            .await
    }

    /// Delete a vector store.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        vector_store_id: &str,
    ) -> Result<DeleteVectorStoreResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/vector_stores/{vector_store_id}"),
                &self.request_options,
            )
            .await
    }

    /// Modifies a vector store.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        vector_store_id: &str,
        request: UpdateVectorStoreRequest,
    ) -> Result<VectorStoreObject, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{vector_store_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    /// Searches a vector store.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn search(
        &self,
        vector_store_id: &str,
        request: VectorStoreSearchRequest,
    ) -> Result<VectorStoreSearchResultsPage, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{vector_store_id}/search"),
                request,
                &self.request_options,
            )
            .await
    }
}
