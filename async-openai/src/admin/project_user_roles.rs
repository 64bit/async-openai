use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::groups::PublicAssignOrganizationGroupRoleBody,
    types::admin::roles::{DeletedRoleAssignmentResource, RoleListResource},
    types::admin::users::UserRoleAssignment,
    Client, RequestOptions,
};

/// Manage role assignments for users in a project.
pub struct ProjectUserRoles<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub user_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectUserRoles<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str, user_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            user_id: user_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all role assignments for a user in the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<RoleListResource, OpenAIError> {
        self.client
            .get(
                format!("/projects/{}/users/{}/roles", self.project_id, self.user_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Assigns a role to a user in the project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn assign(
        &self,
        request: PublicAssignOrganizationGroupRoleBody,
    ) -> Result<UserRoleAssignment, OpenAIError> {
        self.client
            .post(
                format!("/projects/{}/users/{}/roles", self.project_id, self.user_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Unassigns a role from a user in the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn unassign(
        &self,
        role_id: &str,
    ) -> Result<DeletedRoleAssignmentResource, OpenAIError> {
        self.client
            .delete(
                format!(
                    "/projects/{}/users/{}/roles/{}",
                    self.project_id, self.user_id, role_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }
}
