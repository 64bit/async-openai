use crate::{
    config::Config,
    error::OpenAIError,
    types::vectorstores::{
        CreateVectorStoreFileRequest, DeleteVectorStoreFileResponse, ListVectorStoreFilesResponse,
        UpdateVectorStoreFileAttributesRequest, VectorStoreFileContentResponse,
        VectorStoreFileObject,
    },
    Client, RequestOptions,
};

/// Vector store files represent files inside a vector store.
///
/// Related guide: [File Search](https://platform.openai.com/docs/assistants/tools/file-search)
pub struct VectorStoreFiles<'c, C: Config> {
    client: &'c Client<C>,
    pub vector_store_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> VectorStoreFiles<'c, C> {
    pub fn new(client: &'c Client<C>, vector_store_id: &str) -> Self {
        Self {
            client,
            vector_store_id: vector_store_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Create a vector store file by attaching a [File](https://platform.openai.com/docs/api-reference/files) to a [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object).
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateVectorStoreFileRequest,
    ) -> Result<VectorStoreFileObject, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{}/files", &self.vector_store_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieves a vector store file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, file_id: &str) -> Result<VectorStoreFileObject, OpenAIError> {
        self.client
            .get(
                &format!("/vector_stores/{}/files/{file_id}", &self.vector_store_id),
                &self.request_options,
            )
            .await
    }

    /// Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](https://platform.openai.com/docs/api-reference/files/delete) endpoint.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        file_id: &str,
    ) -> Result<DeleteVectorStoreFileResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/vector_stores/{}/files/{file_id}", &self.vector_store_id),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of vector store files.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListVectorStoreFilesResponse, OpenAIError> {
        self.client
            .get(
                &format!("/vector_stores/{}/files", &self.vector_store_id),
                &self.request_options,
            )
            .await
    }

    /// Update attributes on a vector store file.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        file_id: &str,
        request: UpdateVectorStoreFileAttributesRequest,
    ) -> Result<VectorStoreFileObject, OpenAIError> {
        self.client
            .post(
                &format!("/vector_stores/{}/files/{file_id}", &self.vector_store_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve the parsed contents of a vector store file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve_file_content(
        &self,
        file_id: &str,
    ) -> Result<VectorStoreFileContentResponse, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/vector_stores/{}/files/{file_id}/content",
                    &self.vector_store_id
                ),
                &self.request_options,
            )
            .await
    }
}

#[cfg(all(test, feature = "vectorstore", feature = "file"))]
mod tests {
    use crate::types::files::{CreateFileRequest, FileInput, FilePurpose};
    use crate::types::vectorstores::CreateVectorStoreRequest;
    use crate::Client;

    #[tokio::test]
    async fn vector_store_file_creation_and_deletion(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();

        // Create a file
        let openai_file = client
            .files()
            .create(CreateFileRequest {
                file: FileInput::from_vec_u8(
                    String::from("meow.txt"),
                    String::from(":3").into_bytes(),
                ),
                purpose: FilePurpose::UserData,
                expires_after: None,
            })
            .await?;

        // Create a vector store
        let vecor_store_object = client
            .vector_stores()
            .create(CreateVectorStoreRequest {
                file_ids: Some(vec![openai_file.id.clone()]),
                name: None,
                description: None,
                expires_after: None,
                chunking_strategy: None,
                metadata: None,
            })
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let vector_store_file_object = client
            .vector_stores()
            .files(&vecor_store_object.id)
            .retrieve(&openai_file.id)
            .await?;

        assert_eq!(vector_store_file_object.id, openai_file.id);
        // Delete the vector store
        client
            .vector_stores()
            .delete(&vecor_store_object.id)
            .await?;

        // Delete the file
        client.files().delete(&openai_file.id).await?;

        Ok(())
    }
}
