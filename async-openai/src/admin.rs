use crate::{
    admin_api_keys::AdminAPIKeys, audit_logs::AuditLogs, certificates::Certificates,
    config::Config, invites::Invites, projects::Projects, users::Users, Client,
};

/// Admin group for all administration APIs.
/// This groups together admin API keys, invites, users, projects, audit logs, and certificates.
pub struct Admin<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Admin<'c, C> {
    pub(crate) fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// To call [AdminAPIKeys] group related APIs using this client.
    pub fn api_keys(&self) -> AdminAPIKeys<'_, C> {
        AdminAPIKeys::new(self.client)
    }

    /// To call [Invites] group related APIs using this client.
    pub fn invites(&self) -> Invites<'_, C> {
        Invites::new(self.client)
    }

    /// To call [Users] group related APIs using this client.
    pub fn users(&self) -> Users<'_, C> {
        Users::new(self.client)
    }

    /// To call [Projects] group related APIs using this client.
    pub fn projects(&self) -> Projects<'_, C> {
        Projects::new(self.client)
    }

    /// To call [AuditLogs] group related APIs using this client.
    pub fn audit_logs(&self) -> AuditLogs<'_, C> {
        AuditLogs::new(self.client)
    }

    /// To call [Certificates] group related APIs using this client.
    pub fn certificates(&self) -> Certificates<'_, C> {
        Certificates::new(self.client)
    }
}
