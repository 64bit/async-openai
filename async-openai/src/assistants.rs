use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        AssistantObject, CreateAssistantRequest, DeleteAssistantResponse, ListAssistantsResponse,
        ModifyAssistantRequest,
    },
    AssistantFiles, Client,
};

/// Build assistants that can call models and use tools to perform tasks.
///
/// [Get started with the Assistants API](https://platform.openai.com/docs/assistants)
pub struct Assistants<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Assistants<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Assistant [AssistantFiles] API group
    pub fn files(&self, assistant_id: &str) -> AssistantFiles<C> {
        AssistantFiles::new(self.client, assistant_id)
    }

    /// Create an assistant with a model and instructions.
    pub async fn create(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantObject, OpenAIError> {
        self.client.post("/assistants", request).await
    }

    /// Retrieves an assistant.
    pub async fn retrieve(&self, assistant_id: &str) -> Result<AssistantObject, OpenAIError> {
        self.client
            .get(&format!("/assistants/{assistant_id}"))
            .await
    }

    /// Modifies an assistant.
    pub async fn update(
        &self,
        assistant_id: &str,
        request: ModifyAssistantRequest,
    ) -> Result<AssistantObject, OpenAIError> {
        self.client
            .post(&format!("/assistants/{assistant_id}"), request)
            .await
    }

    /// Delete an assistant.
    pub async fn delete(&self, assistant_id: &str) -> Result<DeleteAssistantResponse, OpenAIError> {
        self.client
            .delete(&format!("/assistants/{assistant_id}"))
            .await
    }

    /// Returns a list of assistants.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListAssistantsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/assistants", query).await
    }
}
