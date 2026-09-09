use crate::{config::Config, error::OpenAIError, Client, RequestOptions};

/// Safety APIs for the authenticated project.
pub struct Safety<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Safety<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Access project safety alerts.
    pub fn alerts(&self) -> SafetyAlerts<'_, C> {
        SafetyAlerts::new(self.client)
    }
}

/// Retrieve safety alerts for the authenticated project.
pub struct SafetyAlerts<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}
impl<'c, C: Config> SafetyAlerts<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }
    /// Get a safety alert belonging to the authenticated API project.
    pub async fn retrieve(
        &self,
        id: &str,
    ) -> Result<crate::types::safety::SafetyAlertResource, OpenAIError> {
        self.client
            .get(&format!("/safety/alerts/{id}"), &self.request_options)
            .await
    }
}
