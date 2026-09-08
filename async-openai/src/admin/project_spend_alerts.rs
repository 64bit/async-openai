use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        CreateSpendAlertBody, ProjectSpendAlert, ProjectSpendAlertDeletedResource,
        ProjectSpendAlertListResource,
    },
    Client, RequestOptions,
};

pub struct ProjectSpendAlerts<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
    pub project_id: String,
}

impl<'c, C: Config> ProjectSpendAlerts<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
            project_id: project_id.into(),
        }
    }

    /// Lists project spend alerts.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectSpendAlertListResource, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/organization/projects/{project_id}/spend_alerts",
                    project_id = self.project_id
                ),
                &self.request_options,
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
                &format!(
                    "/organization/projects/{project_id}/spend_alerts",
                    project_id = self.project_id
                ),
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
                    "/organization/projects/{project_id}/spend_alerts/{alert_id}",
                    project_id = self.project_id
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
                    "/organization/projects/{project_id}/spend_alerts/{alert_id}",
                    project_id = self.project_id
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
                    "/organization/projects/{project_id}/spend_alerts/{alert_id}",
                    project_id = self.project_id
                ),
                &self.request_options,
            )
            .await
    }
}
