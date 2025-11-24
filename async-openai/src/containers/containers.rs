use crate::{
    config::Config,
    error::OpenAIError,
    types::containers::{
        ContainerListResource, ContainerResource, CreateContainerRequest, DeleteContainerResponse,
    },
    Client, ContainerFiles, RequestOptions,
};

pub struct Containers<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Containers<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
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
        self.client
            .post("/containers", request, &self.request_options)
            .await
    }

    /// List containers.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ContainerListResource, OpenAIError> {
        self.client.get("/containers", &self.request_options).await
    }

    /// Retrieve a container.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, container_id: &str) -> Result<ContainerResource, OpenAIError> {
        self.client
            .get(
                format!("/containers/{container_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Delete a container.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, container_id: &str) -> Result<DeleteContainerResponse, OpenAIError> {
        self.client
            .delete(
                format!("/containers/{container_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
