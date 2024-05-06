use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateVectorStoreFileRequest, DeleteVectorStoreFileResponse, ListVectorStoreFilesResponse,
        VectorStoreFileObject,
    },
    Client,
};

/// Vector store files represent files inside a vector store.
///
/// Related guide: [File Search](https://platform.openai.com/docs/assistants/tools/file-search)
pub struct VectorStoreFiles<'c, C: Config> {
    client: &'c Client<C>,
    pub vector_store_id: String,
}

impl<'c, C: Config> VectorStoreFiles<'c, C> {
    pub fn new(client: &'c Client<C>, vector_store_id: &str) -> Self {
        Self {
            client,
            vector_store_id: vector_store_id.into(),
        }
    }

    /// Create a vector store file by attaching a [File](https://platform.openai.com/docs/api-reference/files) to a [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object).
    pub async fn create(
        &self,
        request: CreateVectorStoreFileRequest,
    ) -> Result<VectorStoreFileObject, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{}/files", &self.vector_store_id),
                request,
            )
            .await
    }

    /// Retrieves a vector store file.
    pub async fn retrieve(&self, file_id: &str) -> Result<VectorStoreFileObject, OpenAIError> {
        self.client
            .get(&format!(
                "/vector_stores/{}/files/{file_id}",
                &self.vector_store_id
            ))
            .await
    }

    /// Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](https://platform.openai.com/docs/api-reference/files/delete) endpoint.
    pub async fn delete(
        &self,
        file_id: &str,
    ) -> Result<DeleteVectorStoreFileResponse, OpenAIError> {
        self.client
            .delete(&format!(
                "/vector_stores/{}/files/{file_id}",
                &self.vector_store_id
            ))
            .await
    }

    /// Returns a list of vector store files.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListVectorStoreFilesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                &format!("/vector_stores/{}/files", &self.vector_store_id),
                query,
            )
            .await
    }
}
