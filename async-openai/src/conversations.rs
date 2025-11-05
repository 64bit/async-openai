use crate::{
    config::Config,
    conversation_items::ConversationItems,
    error::OpenAIError,
    types::responses::{
        ConversationResource, CreateConversationRequest, DeleteConversationResponse,
        UpdateConversationRequest,
    },
    Client,
};

pub struct Conversations<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Conversations<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// [ConversationItems] API group
    pub fn items(&self, conversation_id: &str) -> ConversationItems<'_, C> {
        ConversationItems::new(self.client, conversation_id)
    }

    /// Create a conversation.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateConversationRequest,
    ) -> Result<ConversationResource, OpenAIError> {
        self.client.post("/conversations", request).await
    }

    /// Retrieves a conversation.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        conversation_id: &str,
    ) -> Result<ConversationResource, OpenAIError> {
        self.client
            .get(&format!("/conversations/{conversation_id}"))
            .await
    }

    /// Delete a conversation. Items in the conversation will not be deleted.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        conversation_id: &str,
    ) -> Result<DeleteConversationResponse, OpenAIError> {
        self.client
            .delete(&format!("/conversations/{conversation_id}"))
            .await
    }

    /// Update a conversation.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        conversation_id: &str,
        request: UpdateConversationRequest,
    ) -> Result<ConversationResource, OpenAIError> {
        self.client
            .post(&format!("/conversations/{conversation_id}"), request)
            .await
    }
}
