use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::spend_alerts::{
        CreateSpendAlertBody, ListSpendAlertsQuery, OrganizationSpendAlert,
        OrganizationSpendAlertDeletedResource, OrganizationSpendAlertListResource,
    },
    Client, RequestOptions,
};

/// Manage spend alerts for an organization.
pub struct SpendAlerts<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> SpendAlerts<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Lists organization spend alerts.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list(
        &self,
        query: ListSpendAlertsQuery,
    ) -> Result<OrganizationSpendAlertListResource, OpenAIError> {
        let mut options = self.request_options.clone();
        options.with_query(&query)?;
        self.client
            .get("/organization/spend_alerts", &options)
            .await
    }

    /// Creates an organization spend alert.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateSpendAlertBody,
    ) -> Result<OrganizationSpendAlert, OpenAIError> {
        self.client
            .post("/organization/spend_alerts", request, &self.request_options)
            .await
    }

    /// Retrieves an organization spend alert.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, alert_id: &str) -> Result<OrganizationSpendAlert, OpenAIError> {
        self.client
            .get(
                &format!("/organization/spend_alerts/{alert_id}"),
                &self.request_options,
            )
            .await
    }

    /// Updates an organization spend alert.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
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
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
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
