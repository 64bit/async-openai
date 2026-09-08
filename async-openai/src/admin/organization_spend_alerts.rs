use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::organization_spend_alerts::{
        CreateSpendAlertBody, OrganizationSpendAlert, OrganizationSpendAlertDeletedResource,
        OrganizationSpendAlertListResource,
    },
    Client, RequestOptions,
};

pub struct OrganizationSpendAlerts<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> OrganizationSpendAlerts<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }
    /// Lists organization spend alerts.
    pub async fn list(&self) -> Result<OrganizationSpendAlertListResource, OpenAIError> {
        self.client
            .get("/organization/spend_alerts", &self.request_options)
            .await
    }

    /// Creates an organization spend alert.
    pub async fn create(
        &self,
        request: CreateSpendAlertBody,
    ) -> Result<OrganizationSpendAlert, OpenAIError> {
        self.client
            .post("/organization/spend_alerts", request, &self.request_options)
            .await
    }

    /// Retrieves an organization spend alert.
    pub async fn retrieve(&self, alert_id: &str) -> Result<OrganizationSpendAlert, OpenAIError> {
        self.client
            .get(
                &format!("/organization/spend_alerts/{alert_id}"),
                &self.request_options,
            )
            .await
    }

    /// Updates an organization spend alert.
    pub async fn update(
        &self,
        alert_id: &str,
        request: CreateSpendAlertBody,
    ) -> Result<OrganizationSpendAlert, OpenAIError> {
        self.client
            .post(
                &format!("/organization/spend_alerts/{alert_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes an organization spend alert.
    pub async fn delete(
        &self,
        alert_id: &str,
    ) -> Result<OrganizationSpendAlertDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!("/organization/spend_alerts/{alert_id}"),
                &self.request_options,
            )
            .await
    }
}
