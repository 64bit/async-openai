use crate::{
    config::Config,
    error::OpenAIError,
    types::assistants::{
        AssistantObject, CreateAssistantRequest, DeleteAssistantResponse, ListAssistantsResponse,
        ModifyAssistantRequest,
    },
    Client, RequestOptions,
};

/// Build assistants that can call models and use tools to perform tasks.
///
/// [Get started with the Assistants API](https://platform.openai.com/docs/assistants)
pub struct Assistants<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Assistants<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create an assistant with a model and instructions.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantObject, OpenAIError> {
        self.client
            .post("/assistants", request, &self.request_options)
            .await
    }

    /// Retrieves an assistant.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, assistant_id: &str) -> Result<AssistantObject, OpenAIError> {
        self.client
            .get(
                &format!("/assistants/{assistant_id}"),
                &self.request_options,
            )
            .await
    }

    /// Modifies an assistant.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        assistant_id: &str,
        request: ModifyAssistantRequest,
    ) -> Result<AssistantObject, OpenAIError> {
        self.client
            .post(
                &format!("/assistants/{assistant_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    /// Delete an assistant.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, assistant_id: &str) -> Result<DeleteAssistantResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/assistants/{assistant_id}"),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of assistants.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListAssistantsResponse, OpenAIError> {
        self.client.get("/assistants", &self.request_options).await
    }
}
