use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual certificate uploaded to the organization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Certificate {
    /// The object type. Can be `certificate`, `organization.certificate`, or `organization.project.certificate`.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the certificate.
    pub name: Option<String>,
    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    pub created_at: u64,
    /// Details about the certificate.
    pub certificate_details: CertificateDetails,
    /// Whether the certificate is currently active at the specified scope.
    /// Not returned when getting details for a specific certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Represents an individual certificate configured at the organization level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationCertificate {
    /// The object type, which is always `organization.certificate`.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the certificate.
    pub name: Option<String>,
    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    pub created_at: u64,
    /// Details about the certificate.
    pub certificate_details: CertificateDetails,
    /// Whether the certificate is currently active at the organization level.
    pub active: bool,
}

/// Response returned after activating one or more organization certificates.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationCertificateActivationResponse {
    /// The organization certificate activation result type. Always `organization.certificate.activation`.
    pub object: String,
    pub data: Vec<OrganizationCertificate>,
}

/// Response returned after deactivating one or more organization certificates.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationCertificateDeactivationResponse {
    /// The organization certificate deactivation result type. Always `organization.certificate.deactivation`.
    pub object: String,
    pub data: Vec<OrganizationCertificate>,
}

/// Represents an individual certificate configured at the project level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationProjectCertificate {
    /// The object type, which is always `organization.project.certificate`.
    pub object: String,
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The name of the certificate.
    pub name: Option<String>,
    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    pub created_at: u64,
    /// Details about the certificate.
    pub certificate_details: CertificateDetails,
    /// Whether the certificate is currently active at the project level.
    pub active: bool,
}

/// Response returned after activating one or more project certificates.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationProjectCertificateActivationResponse {
    /// The project certificate activation result type. Always `organization.project.certificate.activation`.
    pub object: String,
    pub data: Vec<OrganizationProjectCertificate>,
}

/// Response returned after deactivating one or more project certificates.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrganizationProjectCertificateDeactivationResponse {
    /// The project certificate deactivation result type. Always `organization.project.certificate.deactivation`.
    pub object: String,
    pub data: Vec<OrganizationProjectCertificate>,
}

/// Response for listing certificates attached to a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListProjectCertificatesResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of certificates attached to the project.
    pub data: Vec<OrganizationProjectCertificate>,
    /// The ID of the first certificate in the list.
    pub first_id: Option<String>,
    /// The ID of the last certificate in the list.
    pub last_id: Option<String>,
    /// Indicates if there are more certificates available.
    pub has_more: bool,
}

/// Details about a certificate.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CertificateDetails {
    /// The Unix timestamp (in seconds) of when the certificate becomes valid.
    pub valid_at: u64,
    /// The Unix timestamp (in seconds) of when the certificate expires.
    pub expires_at: u64,
    /// The content of the certificate in PEM format.
    /// Only included when requested via the `include[]=content` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Response for listing certificates.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListCertificatesResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// The list of certificates.
    pub data: Vec<OrganizationCertificate>,
    /// The ID of the first certificate in the list.
    pub first_id: Option<String>,
    /// The ID of the last certificate in the list.
    pub last_id: Option<String>,
    /// Indicates if there are more certificates available.
    pub has_more: bool,
}

/// Request for uploading a certificate.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "UploadCertificateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UploadCertificateRequest {
    /// An optional name for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The certificate content in PEM format.
    pub certificate: String,
}

/// Request for modifying a certificate.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq, Default)]
#[builder(name = "ModifyCertificateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ModifyCertificateRequest {
    /// The updated name for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request for toggling (activating/deactivating) certificates.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "ToggleCertificatesRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ToggleCertificatesRequest {
    /// Array of certificate IDs to toggle (1-10 certificates).
    pub certificate_ids: Vec<String>,
}

/// Response for deleting a certificate.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteCertificateResponse {
    /// The object type, which is always `certificate.deleted`.
    pub object: String,
    /// The ID of the certificate that was deleted.
    pub id: String,
}
