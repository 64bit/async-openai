use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        ConversationItem, ConversationItemList, ConversationResource,
        CreateConversationItemsRequest,
    },
    Client,
};

/// Conversation items represent items within a conversation.
pub struct ConversationItems<'c, C: Config> {
    client: &'c Client<C>,
    pub conversation_id: String,
}

impl<'c, C: Config> ConversationItems<'c, C> {
    pub fn new(client: &'c Client<C>, conversation_id: &str) -> Self {
        Self {
            client,
            conversation_id: conversation_id.into(),
        }
    }

    /// Create items in a conversation.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateConversationItemsRequest,
    ) -> Result<ConversationItemList, OpenAIError> {
        self.client
            .post(
                &format!("/conversations/{}/items", &self.conversation_id),
                request,
            )
            .await
    }

    /// List all items for a conversation.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ConversationItemList, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                &format!("/conversations/{}/items", &self.conversation_id),
                &query,
            )
            .await
    }

    /// Retrieve an item from a conversation.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, item_id: &str) -> Result<ConversationItem, OpenAIError> {
        self.client
            .get(&format!(
                "/conversations/{}/items/{item_id}",
                &self.conversation_id
            ))
            .await
    }

    /// Delete an item from a conversation.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, item_id: &str) -> Result<ConversationResource, OpenAIError> {
        self.client
            .delete(&format!(
                "/conversations/{}/items/{item_id}",
                &self.conversation_id
            ))
            .await
    }
}
