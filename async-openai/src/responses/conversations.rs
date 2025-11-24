use crate::{
    config::Config,
    error::OpenAIError,
    types::responses::{
        ConversationResource, CreateConversationRequest, DeleteConversationResponse,
        UpdateConversationRequest,
    },
    Client, ConversationItems, RequestOptions,
};

pub struct Conversations<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Conversations<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
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
        self.client
            .post("/conversations", request, &self.request_options)
            .await
    }

    /// Retrieves a conversation.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        conversation_id: &str,
    ) -> Result<ConversationResource, OpenAIError> {
        self.client
            .get(
                &format!("/conversations/{conversation_id}"),
                &self.request_options,
            )
            .await
    }

    /// Delete a conversation. Items in the conversation will not be deleted.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        conversation_id: &str,
    ) -> Result<DeleteConversationResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/conversations/{conversation_id}"),
                &self.request_options,
            )
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
            .post(
                &format!("/conversations/{conversation_id}"),
                request,
                &self.request_options,
            )
            .await
    }
}
