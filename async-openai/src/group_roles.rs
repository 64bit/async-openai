use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::groups::{GroupRoleAssignment, PublicAssignOrganizationGroupRoleBody},
    types::admin::roles::{DeletedRoleAssignmentResource, RoleListResource},
    Client, RequestOptions,
};

/// Manage role assignments for groups in the organization.
pub struct GroupRoles<'c, C: Config> {
    client: &'c Client<C>,
    pub group_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> GroupRoles<'c, C> {
    pub fn new(client: &'c Client<C>, group_id: &str) -> Self {
        Self {
            client,
            group_id: group_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all role assignments for a group.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<RoleListResource, OpenAIError> {
        self.client
            .get(
                format!("/organization/groups/{}/roles", self.group_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Assigns a role to a group.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn assign(
        &self,
        request: PublicAssignOrganizationGroupRoleBody,
    ) -> Result<GroupRoleAssignment, OpenAIError> {
        self.client
            .post(
                format!("/organization/groups/{}/roles", self.group_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Unassigns a role from a group.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn unassign(
        &self,
        role_id: &str,
    ) -> Result<DeletedRoleAssignmentResource, OpenAIError> {
        self.client
            .delete(
                format!("/organization/groups/{}/roles/{}", self.group_id, role_id).as_str(),
                &self.request_options,
            )
            .await
    }
}
