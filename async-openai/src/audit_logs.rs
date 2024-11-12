use serde::Serialize;

use crate::{config::Config, error::OpenAIError, types::ListAuditLogsResponse, Client};

pub struct AuditLogs<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> AuditLogs<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    pub async fn get<Q>(&self, query: &Q) -> Result<ListAuditLogsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/audit_logs", query)
            .await
    }
}
