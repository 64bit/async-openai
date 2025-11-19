use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::roles::{
        PublicCreateOrganizationRoleBody, PublicRoleListResource, PublicUpdateOrganizationRoleBody,
        Role, RoleDeletedResource,
    },
    Client, RequestOptions,
};

/// Manage custom roles that can be assigned to groups and users at the project level.
pub struct ProjectRoles<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectRoles<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists the roles configured for the project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<PublicRoleListResource, OpenAIError> {
        self.client
            .get(
                format!("/projects/{}/roles", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Creates a custom role for the project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: PublicCreateOrganizationRoleBody,
    ) -> Result<Role, OpenAIError> {
        self.client
            .post(
                format!("/projects/{}/roles", self.project_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Updates an existing project role.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        role_id: &str,
        request: PublicUpdateOrganizationRoleBody,
    ) -> Result<Role, OpenAIError> {
        self.client
            .post(
                format!("/projects/{}/roles/{}", self.project_id, role_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a custom role from the project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, role_id: &str) -> Result<RoleDeletedResource, OpenAIError> {
        self.client
            .delete(
                format!("/projects/{}/roles/{}", self.project_id, role_id).as_str(),
                &self.request_options,
            )
            .await
    }
}
