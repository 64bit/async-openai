use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::organization_spend_limit::{
        OrganizationSpendLimitDeletedResource, OrganizationSpendLimitResource,
        UpdateOrganizationSpendLimitBody,
    },
    Client, RequestOptions,
};

pub struct OrganizationSpendLimit<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> OrganizationSpendLimit<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }
    /// Get the organization's hard spend limit.
    pub async fn retrieve(&self) -> Result<OrganizationSpendLimitResource, OpenAIError> {
        self.client
            .get("/organization/spend_limit", &self.request_options)
            .await
    }

    /// Create or replace the organization's hard spend limit.
    pub async fn update(
        &self,
        request: UpdateOrganizationSpendLimitBody,
    ) -> Result<OrganizationSpendLimitResource, OpenAIError> {
        self.client
            .post("/organization/spend_limit", request, &self.request_options)
            .await
    }

    /// Delete the organization's hard spend limit.
    pub async fn delete(&self) -> Result<OrganizationSpendLimitDeletedResource, OpenAIError> {
        self.client
            .delete("/organization/spend_limit", &self.request_options)
            .await
    }
}
