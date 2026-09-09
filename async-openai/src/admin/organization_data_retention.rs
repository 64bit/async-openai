use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::organization_data_retention::{
        OrganizationDataRetention, UpdateOrganizationDataRetentionBody,
    },
    Client, RequestOptions,
};

pub struct OrganizationDataRetentions<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> OrganizationDataRetentions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }
    /// Retrieves organization data retention controls.
    pub async fn retrieve(&self) -> Result<OrganizationDataRetention, OpenAIError> {
        self.client
            .get("/organization/data_retention", &self.request_options)
            .await
    }

    /// Updates organization data retention controls.
    pub async fn update(
        &self,
        request: UpdateOrganizationDataRetentionBody,
    ) -> Result<OrganizationDataRetention, OpenAIError> {
        self.client
            .post(
                "/organization/data_retention",
                request,
                &self.request_options,
            )
            .await
    }
}
