use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::certificates::{ListCertificatesResponse, ToggleCertificatesRequest},
    Client,
};

/// Manage certificates for a given project. Supports listing, activating, and deactivating certificates.
pub struct ProjectCertificates<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
}

impl<'c, C: Config> ProjectCertificates<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }

    /// List all certificates for this project.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListCertificatesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/organization/projects/{}/certificates", self.project_id).as_str(),
                query,
            )
            .await
    }

    /// Activate certificates for this project.
    /// You can atomically and idempotently activate up to 10 certificates at a time.
    pub async fn activate(
        &self,
        request: ToggleCertificatesRequest,
    ) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .post(
                format!(
                    "/organization/projects/{}/certificates/activate",
                    self.project_id
                )
                .as_str(),
                request,
            )
            .await
    }

    /// Deactivate certificates for this project.
    /// You can atomically and idempotently deactivate up to 10 certificates at a time.
    pub async fn deactivate(
        &self,
        request: ToggleCertificatesRequest,
    ) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .post(
                format!(
                    "/organization/projects/{}/certificates/deactivate",
                    self.project_id
                )
                .as_str(),
                request,
            )
            .await
    }
}
