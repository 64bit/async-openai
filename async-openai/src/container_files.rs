use bytes::Bytes;
use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::containers::{
        ContainerFileListResource, ContainerFileResource, CreateContainerFileRequest,
        DeleteContainerFileResponse,
    },
    Client,
};

/// Create and manage container files for use with the Code Interpreter tool.
pub struct ContainerFiles<'c, C: Config> {
    client: &'c Client<C>,
    container_id: String,
}

impl<'c, C: Config> ContainerFiles<'c, C> {
    pub fn new(client: &'c Client<C>, container_id: &str) -> Self {
        Self {
            client,
            container_id: container_id.to_string(),
        }
    }

    /// Create a container file by uploading a raw file or by referencing an existing file ID.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateContainerFileRequest,
    ) -> Result<ContainerFileResource, OpenAIError> {
        self.client
            .post_form(&format!("/containers/{}/files", self.container_id), request)
            .await
    }

    /// List container files.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ContainerFileListResource, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(&format!("/containers/{}/files", self.container_id), &query)
            .await
    }

    /// Retrieve a container file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, file_id: &str) -> Result<ContainerFileResource, OpenAIError> {
        self.client
            .get(format!("/containers/{}/files/{file_id}", self.container_id).as_str())
            .await
    }

    /// Delete a container file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, file_id: &str) -> Result<DeleteContainerFileResponse, OpenAIError> {
        self.client
            .delete(format!("/containers/{}/files/{file_id}", self.container_id).as_str())
            .await
    }

    /// Returns the content of a container file.
    pub async fn content(&self, file_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(format!("/containers/{}/files/{file_id}/content", self.container_id).as_str())
            .await?;
        Ok(bytes)
    }
}
