use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::roles::{
        PublicCreateOrganizationRoleBody, PublicRoleListResource, PublicUpdateOrganizationRoleBody,
        Role, RoleDeletedResource,
    },
    Client, RequestOptions,
};

/// Manage custom roles that can be assigned to groups and users at the organization or project level.
pub struct Roles<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Roles<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Lists the roles configured for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<PublicRoleListResource, OpenAIError> {
        self.client
            .get("/organization/roles", &self.request_options)
            .await
    }

    /// Creates a custom role for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: PublicCreateOrganizationRoleBody,
    ) -> Result<Role, OpenAIError> {
        self.client
            .post("/organization/roles", request, &self.request_options)
            .await
    }

    /// Updates an existing organization role.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        role_id: &str,
        request: PublicUpdateOrganizationRoleBody,
    ) -> Result<Role, OpenAIError> {
        self.client
            .post(
                format!("/organization/roles/{role_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a custom role from the organization.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, role_id: &str) -> Result<RoleDeletedResource, OpenAIError> {
        self.client
            .delete(
                format!("/organization/roles/{role_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
