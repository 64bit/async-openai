use serde::Serialize;

use crate::{
    config::Config,
    container_files::ContainerFiles,
    error::OpenAIError,
    types::containers::{
        ContainerListResource, ContainerResource, CreateContainerRequest, DeleteContainerResponse,
    },
    Client,
};

pub struct Containers<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Containers<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// [ContainerFiles] API group
    pub fn files(&self, container_id: &str) -> ContainerFiles<'_, C> {
        ContainerFiles::new(self.client, container_id)
    }

    /// Create a container.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateContainerRequest,
    ) -> Result<ContainerResource, OpenAIError> {
        self.client.post("/containers", request).await
    }

    /// List containers.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ContainerListResource, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/containers", &query).await
    }

    /// Retrieve a container.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, container_id: &str) -> Result<ContainerResource, OpenAIError> {
        self.client
            .get(format!("/containers/{container_id}").as_str())
            .await
    }

    /// Delete a container.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, container_id: &str) -> Result<DeleteContainerResponse, OpenAIError> {
        self.client
            .delete(format!("/containers/{container_id}").as_str())
            .await
    }
}
