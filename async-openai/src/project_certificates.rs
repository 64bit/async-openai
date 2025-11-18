use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::certificates::{ListCertificatesResponse, ToggleCertificatesRequest},
    Client, RequestOptions,
};

/// Manage certificates for a given project. Supports listing, activating, and deactivating certificates.
pub struct ProjectCertificates<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectCertificates<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// List all certificates for this project.
    pub async fn list(&self) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/certificates", self.project_id).as_str(),
                &self.request_options,
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
                &self.request_options,
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
                &self.request_options,
            )
            .await
    }
}
