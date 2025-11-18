use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::invites::{Invite, InviteDeleteResponse, InviteListResponse, InviteRequest},
    Client, RequestOptions,
};

/// Invite and manage invitations for an organization. Invited users are automatically added to the Default project.
pub struct Invites<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Invites<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Returns a list of invites in the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<InviteListResponse, OpenAIError> {
        self.client
            .get("/organization/invites", &self.request_options)
            .await
    }

    /// Retrieves an invite.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, invite_id: &str) -> Result<Invite, OpenAIError> {
        self.client
            .get(
                format!("/organization/invites/{invite_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Create an invite for a user to the organization. The invite must be accepted by the user before they have access to the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: InviteRequest) -> Result<Invite, OpenAIError> {
        self.client
            .post("/organization/invites", request, &self.request_options)
            .await
    }

    /// Delete an invite. If the invite has already been accepted, it cannot be deleted.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, invite_id: &str) -> Result<InviteDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!("/organization/invites/{invite_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
