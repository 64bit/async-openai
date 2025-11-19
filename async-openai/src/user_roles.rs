use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::roles::{DeletedRoleAssignmentResource, RoleListResource},
    types::admin::users::{PublicAssignOrganizationUserRoleBody, UserRoleAssignment},
    Client, RequestOptions,
};

/// Manage role assignments for users in the organization.
pub struct UserRoles<'c, C: Config> {
    client: &'c Client<C>,
    pub user_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> UserRoles<'c, C> {
    pub fn new(client: &'c Client<C>, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all role assignments for a user.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<RoleListResource, OpenAIError> {
        self.client
            .get(
                format!("/organization/users/{}/roles", self.user_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Assigns a role to a user.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn assign(
        &self,
        request: PublicAssignOrganizationUserRoleBody,
    ) -> Result<UserRoleAssignment, OpenAIError> {
        self.client
            .post(
                format!("/organization/users/{}/roles", self.user_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Unassigns a role from a user.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn unassign(
        &self,
        role_id: &str,
    ) -> Result<DeletedRoleAssignmentResource, OpenAIError> {
        self.client
            .delete(
                format!("/organization/users/{}/roles/{}", self.user_id, role_id).as_str(),
                &self.request_options,
            )
            .await
    }
}
