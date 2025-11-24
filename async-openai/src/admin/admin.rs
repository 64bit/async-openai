use crate::{
    config::Config, AdminAPIKeys, AuditLogs, Certificates, Client, Groups, Invites, Projects,
    Roles, Usage, Users,
};

/// Admin group for all administration APIs.
/// This groups together admin API keys, invites, users, projects, audit logs, certificates, roles, and groups.
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

    /// To call [Roles] group related APIs using this client.
    pub fn roles(&self) -> Roles<'_, C> {
        Roles::new(self.client)
    }

    /// To call [Groups] group related APIs using this client.
    pub fn groups(&self) -> Groups<'_, C> {
        Groups::new(self.client)
    }

    /// To call [Usage] group related APIs using this client.
    pub fn usage(&self) -> Usage<'_, C> {
        Usage::new(self.client)
    }
}
