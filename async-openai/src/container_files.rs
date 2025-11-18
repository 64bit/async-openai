use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::containers::{
        ContainerFileListResource, ContainerFileResource, CreateContainerFileRequest,
        DeleteContainerFileResponse,
    },
    Client, RequestOptions,
};

/// Create and manage container files for use with the Code Interpreter tool.
pub struct ContainerFiles<'c, C: Config> {
    client: &'c Client<C>,
    container_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ContainerFiles<'c, C> {
    pub fn new(client: &'c Client<C>, container_id: &str) -> Self {
        Self {
            client,
            container_id: container_id.to_string(),
            request_options: RequestOptions::new(),
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
            .post_form(
                &format!("/containers/{}/files", self.container_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// List container files.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ContainerFileListResource, OpenAIError> {
        self.client
            .get(
                &format!("/containers/{}/files", self.container_id),
                &self.request_options,
            )
            .await
    }

    /// Retrieve a container file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, file_id: &str) -> Result<ContainerFileResource, OpenAIError> {
        self.client
            .get(
                format!("/containers/{}/files/{file_id}", self.container_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Delete a container file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, file_id: &str) -> Result<DeleteContainerFileResponse, OpenAIError> {
        self.client
            .delete(
                format!("/containers/{}/files/{file_id}", self.container_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Returns the content of a container file.
    pub async fn content(&self, file_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                format!("/containers/{}/files/{file_id}/content", self.container_id).as_str(),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}
