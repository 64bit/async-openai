use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::users::{User, UserDeleteResponse, UserListResponse, UserRoleUpdateRequest},
    Client, RequestOptions, UserRoles,
};

/// Manage users and their role in an organization. Users will be automatically added to the Default project.
pub struct Users<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Users<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// To call [UserRoles] group related APIs using this client.
    pub fn roles(&self, user_id: &str) -> UserRoles<'_, C> {
        UserRoles::new(self.client, user_id)
    }

    /// Lists all of the users in the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<UserListResponse, OpenAIError> {
        self.client
            .get("/organization/users", &self.request_options)
            .await
    }

    /// Modifies a user's role in the organization.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn modify(
        &self,
        user_id: &str,
        request: UserRoleUpdateRequest,
    ) -> Result<User, OpenAIError> {
        self.client
            .post(
                format!("/organization/users/{user_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve a user by their identifier
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, user_id: &str) -> Result<User, OpenAIError> {
        self.client
            .get(
                format!("/organization/users/{user_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Deletes a user from the organization.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, user_id: &str) -> Result<UserDeleteResponse, OpenAIError> {
        self.client
            .delete(
                format!("/organization/users/{user_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
