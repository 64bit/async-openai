use crate::{
    config::Config, error::OpenAIError, types::admin::audit_logs::ListAuditLogsResponse, Client,
    RequestOptions,
};

/// Logs of user actions and configuration changes within this organization.
/// To log events, you must activate logging in the [Organization Settings](https://platform.openai.com/settings/organization/general).
/// Once activated, for security reasons, logging cannot be deactivated.
pub struct AuditLogs<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> AuditLogs<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// List user actions and configuration changes within this organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn get(&self) -> Result<ListAuditLogsResponse, OpenAIError> {
        self.client
            .get("/organization/audit_logs", &self.request_options)
            .await
    }
}
