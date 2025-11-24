use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::groups::{
        CreateGroupBody, GroupDeletedResource, GroupListResource, GroupResourceWithSuccess,
        GroupResponse,
    },
    Client, GroupRoles, GroupUsers, RequestOptions,
};

/// Manage reusable collections of users for organization-wide access control and maintain their membership.
pub struct Groups<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Groups<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// To call [GroupUsers] group related APIs using this client.
    pub fn users(&self, group_id: &str) -> GroupUsers<'_, C> {
        GroupUsers::new(self.client, group_id)
    }

    /// To call [GroupRoles] group related APIs using this client.
    pub fn roles(&self, group_id: &str) -> GroupRoles<'_, C> {
        GroupRoles::new(self.client, group_id)
    }

    /// Lists all groups in the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<GroupListResource, OpenAIError> {
        self.client
            .get("/organization/groups", &self.request_options)
            .await
    }

    /// Creates a new group in the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: CreateGroupBody) -> Result<GroupResponse, OpenAIError> {
        self.client
            .post("/organization/groups", request, &self.request_options)
            .await
    }

    /// Updates a group's information.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        group_id: &str,
        request: crate::types::admin::groups::UpdateGroupBody,
    ) -> Result<GroupResourceWithSuccess, OpenAIError> {
        self.client
            .post(
                format!("/organization/groups/{group_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a group from the organization.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, group_id: &str) -> Result<GroupDeletedResource, OpenAIError> {
        self.client
            .delete(
                format!("/organization/groups/{group_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
