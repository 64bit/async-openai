use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::groups::{GroupRoleAssignment, PublicAssignOrganizationGroupRoleBody},
    types::admin::roles::{DeletedRoleAssignmentResource, RoleListResource},
    Client, RequestOptions,
};

/// Manage role assignments for groups in a project.
pub struct ProjectGroupRoles<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub group_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectGroupRoles<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str, group_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            group_id: group_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all role assignments for a group in the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<RoleListResource, OpenAIError> {
        self.client
            .get(
                format!(
                    "/projects/{}/groups/{}/roles",
                    self.project_id, self.group_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }

    /// Assigns a role to a group in the project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn assign(
        &self,
        request: PublicAssignOrganizationGroupRoleBody,
    ) -> Result<GroupRoleAssignment, OpenAIError> {
        self.client
            .post(
                format!(
                    "/projects/{}/groups/{}/roles",
                    self.project_id, self.group_id
                )
                .as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Unassigns a role from a group in the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn unassign(
        &self,
        role_id: &str,
    ) -> Result<DeletedRoleAssignmentResource, OpenAIError> {
        self.client
            .delete(
                format!(
                    "/projects/{}/groups/{}/roles/{}",
                    self.project_id, self.group_id, role_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }
}
