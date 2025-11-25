use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::certificates::{
        Certificate, DeleteCertificateResponse, ListCertificatesResponse, ModifyCertificateRequest,
        ToggleCertificatesRequest, UploadCertificateRequest,
    },
    Client, RequestOptions,
};

/// Certificates enable Mutual TLS (mTLS) authentication for your organization.
/// Manage certificates at the organization level.
pub struct Certificates<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Certificates<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    // Organization-level certificate operations

    /// List all certificates for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list_organization(&self) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .get("/organization/certificates", &self.request_options)
            .await
    }

    /// Upload a certificate to the organization.
    /// This does not automatically activate the certificate.
    pub async fn upload_organization(
        &self,
        request: UploadCertificateRequest,
    ) -> Result<Certificate, OpenAIError> {
        self.client
            .post("/organization/certificates", request, &self.request_options)
            .await
    }

    /// Activate certificates for the organization.
    /// You can atomically and idempotently activate up to 10 certificates at a time.
    pub async fn activate_organization(
        &self,
        request: ToggleCertificatesRequest,
    ) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .post(
                "/organization/certificates/activate",
                request,
                &self.request_options,
            )
            .await
    }

    /// Deactivate certificates for the organization.
    /// You can atomically and idempotently deactivate up to 10 certificates at a time.
    pub async fn deactivate_organization(
        &self,
        request: ToggleCertificatesRequest,
    ) -> Result<ListCertificatesResponse, OpenAIError> {
        self.client
            .post(
                "/organization/certificates/deactivate",
                request,
                &self.request_options,
            )
            .await
    }

    /// Retrieve a single certificate.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, certificate_id: &str) -> Result<Certificate, OpenAIError> {
        self.client
            .get(
                format!("/organization/certificates/{certificate_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Modify a certificate. Note that only the name can be modified.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn modify(
        &self,
        certificate_id: &str,
        request: ModifyCertificateRequest,
    ) -> Result<Certificate, OpenAIError> {
        self.client
            .post(
                format!("/organization/certificates/{certificate_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Delete a certificate from the organization.
    /// The certificate must be inactive for the organization and all projects.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        certificate_id: &str,
    ) -> Result<DeleteCertificateResponse, OpenAIError> {
        self.client
            .delete(
                format!("/organization/certificates/{certificate_id}").as_str(),
                &self.request_options,
            )
            .await
    }
}
