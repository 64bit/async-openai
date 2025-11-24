use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::groups::{
        CreateGroupUserBody, GroupUserAssignment, GroupUserDeletedResource, UserListResource,
    },
    Client, RequestOptions,
};

/// Manage users within a group, including adding and removing users.
pub struct GroupUsers<'c, C: Config> {
    client: &'c Client<C>,
    pub group_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> GroupUsers<'c, C> {
    pub fn new(client: &'c Client<C>, group_id: &str) -> Self {
        Self {
            client,
            group_id: group_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all users in a group.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<UserListResource, OpenAIError> {
        self.client
            .get(
                format!("/organization/groups/{}/users", self.group_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Adds a user to a group.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn add(
        &self,
        request: CreateGroupUserBody,
    ) -> Result<GroupUserAssignment, OpenAIError> {
        self.client
            .post(
                format!("/organization/groups/{}/users", self.group_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Removes a user from a group.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn remove(&self, user_id: &str) -> Result<GroupUserDeletedResource, OpenAIError> {
        self.client
            .delete(
                format!("/organization/groups/{}/users/{user_id}", self.group_id).as_str(),
                &self.request_options,
            )
            .await
    }
}
