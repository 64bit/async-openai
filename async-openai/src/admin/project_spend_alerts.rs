use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::spend_alerts::{
        CreateSpendAlertBody, ListSpendAlertsQuery, ProjectSpendAlert,
        ProjectSpendAlertDeletedResource, ProjectSpendAlertListResource,
    },
    Client, RequestOptions,
};

/// Manage spend alerts for one project.
pub struct ProjectSpendAlerts<'c, C: Config> {
    client: &'c Client<C>,
    project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectSpendAlerts<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists project spend alerts.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list(
        &self,
        query: ListSpendAlertsQuery,
    ) -> Result<ProjectSpendAlertListResource, OpenAIError> {
        let mut options = self.request_options.clone();
        options.with_query(&query)?;
        self.client
            .get(
                &format!("/organization/projects/{}/spend_alerts", self.project_id),
                &options,
            )
            .await
    }

    /// Creates a project spend alert.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateSpendAlertBody,
    ) -> Result<ProjectSpendAlert, OpenAIError> {
        self.client
            .post(
                &format!("/organization/projects/{}/spend_alerts", self.project_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieves a project spend alert.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, alert_id: &str) -> Result<ProjectSpendAlert, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{}/spend_alerts/{alert_id}",
                    self.project_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Updates a project spend alert.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        alert_id: &str,
        request: CreateSpendAlertBody,
    ) -> Result<ProjectSpendAlert, OpenAIError> {
        self.client
            .post(
                &format!(
                    "/organization/projects/{}/spend_alerts/{alert_id}",
                    self.project_id
                ),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a project spend alert.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        alert_id: &str,
    ) -> Result<ProjectSpendAlertDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!(
                    "/organization/projects/{}/spend_alerts/{alert_id}",
                    self.project_id
                ),
                &self.request_options,
            )
            .await
    }
}
