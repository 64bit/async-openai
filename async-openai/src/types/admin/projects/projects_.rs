use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Represents an individual project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints
    pub id: String,
    /// The object type, which is always `organization.project`
    pub object: String,
    /// The name of the project. This appears in reporting.
    pub name: Option<String>,
    /// The Unix timestamp (in seconds) of when the project was created.
    pub created_at: u64,
    /// The Unix timestamp (in seconds) of when the project was archived or `null`.
    pub archived_at: Option<u64>,
    /// `active` or `archived`
    pub status: Option<String>,
    /// The external key associated with the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
    /// The residency configuration for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residency: Option<PublicProjectResidency>,
}

/// A list of Project objects.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectListResponse {
    pub object: String,
    pub data: Vec<Project>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: String,
}

/// The project create request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder)]
#[builder(name = "ProjectCreateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectCreateRequest {
    /// The friendly name of the project, this name appears in reports.
    pub name: String,
    /// Create the project with the specified data residency region. Your organization must have access to
    /// Data residency functionality in order to use. See [data residency controls](https://platform.openai.com/docs/guides/your-data#data-residency-controls)
    /// to review the functionality and limitations of setting this field.
    /// Deprecated: use `residency` instead. Do not provide both `geography` and `residency`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use residency instead.")]
    pub geography: Option<String>,
    /// External key ID to associate with the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
    /// Create the project with the specified residency configuration. Your organization must have access to
    /// the requested residency configuration in order to use it. See [data residency
    /// controls](/docs/guides/your-data#data-residency-controls) to review the functionality and
    /// limitations of setting this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residency: Option<PublicProjectResidency>,
}

/// The project update request payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "ProjectUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ProjectUpdateRequest {
    /// The updated name of the project, this name appears in reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Geography for the project.
    /// Deprecated: use `residency` when creating a project to configure data residency. This field is
    /// retained for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use residency instead.")]
    pub geography: Option<String>,
    /// External key ID to associate with the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_key_id: Option<String>,
}

/// Details about a group's membership in a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroup {
    /// The object type, which is always `project.group`.
    pub object: String,
    /// Identifier of the project.
    pub project_id: String,
    /// Identifier of the group that has access to the project.
    pub group_id: String,
    /// Display name of the group.
    pub group_name: String,
    /// Unix timestamp (in seconds) when the group was granted project access.
    pub created_at: u64,
    /// The type of the group.
    pub group_type: String,
}

/// Paginated list of groups that have access to a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroupListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// Project group memberships returned in the current page.
    pub data: Vec<ProjectGroup>,
    /// Whether additional project group memberships are available.
    pub has_more: bool,
    /// Cursor to fetch the next page of results, or `null` when there are no more results.
    pub next: Option<String>,
}

/// Confirmation payload returned after removing a group from a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectGroupDeletedResource {
    /// The object type, which is always `project.group.deleted`.
    pub object: String,
    /// Whether the group membership in the project was removed.
    pub deleted: bool,
}

/// Request payload for granting a group access to a project.
#[derive(Debug, Serialize, Deserialize, Builder, Clone, PartialEq)]
#[builder(name = "InviteProjectGroupRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InviteProjectGroupBody {
    /// Identifier of the group to add to the project.
    pub group_id: String,
    /// Identifier of the project role to grant to the group.
    pub role: String,
}

/// Permission state for a single hosted tool on a project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HostedToolPermission {
    /// Whether the hosted tool is enabled for the project.
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HostedToolPermissionUpdate {
    /// Whether to enable the hosted tool for the project.
    pub enabled: bool,
}

/// Represents a project's data retention control setting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectDataRetention {
    /// The object type, which is always `project.data_retention`.
    pub object: String,
    /// The configured project data retention type.
    pub r#type: ProjectDataRetentionType,
}

/// The configured project data retention type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectDataRetentionType {
    #[serde(rename = "organization_default")]
    OrganizationDefault,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "zero_data_retention")]
    ZeroDataRetention,
    #[serde(rename = "modified_abuse_monitoring")]
    ModifiedAbuseMonitoring,
    #[serde(rename = "enhanced_zero_data_retention")]
    EnhancedZeroDataRetention,
    #[serde(rename = "enhanced_modified_abuse_monitoring")]
    EnhancedModifiedAbuseMonitoring,
}

/// Represents hosted tool permissions for a project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectHostedToolPermissions {
    pub file_search: HostedToolPermission,
    pub web_search: HostedToolPermission,
    pub image_generation: HostedToolPermission,
    pub mcp: HostedToolPermission,
    pub code_interpreter: HostedToolPermission,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectHostedToolPermissionsUpdateRequest {
    /// The file search permission update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<HostedToolPermissionUpdate>,
    /// The web search permission update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<HostedToolPermissionUpdate>,
    /// The image generation permission update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_generation: Option<HostedToolPermissionUpdate>,
    /// The MCP permission update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<HostedToolPermissionUpdate>,
    /// The code interpreter permission update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_interpreter: Option<HostedToolPermissionUpdate>,
}

/// Represents the model allowlist or denylist policy for a project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectModelPermissions {
    /// The object type, which is always `project.model_permissions`.
    pub object: String,
    /// Whether the project uses an allowlist or a denylist.
    pub mode: ProjectModelPermissionsMode,
    /// The model IDs included in the model permissions policy.
    pub model_ids: Vec<String>,
}

/// Confirmation payload returned after deleting project model permissions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectModelPermissionsDeleteResponse {
    /// The object type, which is always `project.model_permissions.deleted`.
    pub object: String,
    /// Whether the project model permissions were deleted.
    pub deleted: bool,
}

/// Whether the project uses an allowlist or a denylist.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectModelPermissionsMode {
    #[serde(rename = "allow_list")]
    AllowList,
    #[serde(rename = "deny_list")]
    DenyList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectModelPermissionsUpdateRequest {
    /// The model permissions mode to apply.
    pub mode: ProjectModelPermissionsMode,
    /// The model IDs included in this permissions policy.
    pub model_ids: Vec<String>,
}

/// Represents a spend alert configured at the project level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpendAlert {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `project.spend_alert`.
    pub object: String,
    /// The alert threshold amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount.
    pub currency: ProjectSpendAlertCurrency,
    /// The time interval for evaluating spend against the threshold.
    pub interval: ProjectSpendAlertInterval,
    pub notification_channel: crate::types::admin::projects::SpendAlertNotificationChannel,
}

/// The currency for the threshold amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectSpendAlertCurrency {
    #[serde(rename = "USD")]
    USD,
}

/// Confirmation payload returned after deleting a project spend alert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpendAlertDeletedResource {
    /// The deleted spend alert ID.
    pub id: String,
    /// Always `project.spend_alert.deleted`.
    pub object: String,
    /// Whether the spend alert was deleted.
    pub deleted: bool,
}

/// The time interval for evaluating spend against the threshold.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectSpendAlertInterval {
    #[serde(rename = "month")]
    Month,
}

/// Paginated list of project spend alerts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpendAlertListResource {
    /// Always `list`.
    pub object: String,
    /// Spend alerts returned in the current page.
    pub data: Vec<ProjectSpendAlert>,
    /// The ID of the first spend alert in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    /// The ID of the last spend alert in this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Whether more spend alerts are available when paginating.
    pub has_more: bool,
}

/// Confirmation payload returned after deleting a project hard spend limit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpendLimitDeletedResource {
    /// The object type, which is always `project.spend_limit.deleted`.
    pub object: String,
    /// Whether the hard spend limit was deleted.
    pub deleted: bool,
}

/// Represents a hard spend limit configured at the project level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpendLimitResource {
    /// The object type, which is always `project.spend_limit`.
    pub object: String,
    /// The hard spend limit amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount. Currently, only `USD` is supported.
    pub currency: crate::types::admin::projects::SpendLimitCurrency,
    /// The time interval for evaluating spend against the threshold. Currently, only `month` is supported.
    pub interval: crate::types::admin::projects::SpendLimitInterval,
    /// The current enforcement state of the hard spend limit.
    pub enforcement: crate::types::admin::projects::SpendLimitEnforcement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PublicProjectResidency {
    #[serde(rename = "GLOBAL")]
    GLOBAL,
    #[serde(rename = "US_STORAGE_PROCESSING")]
    USSTORAGEPROCESSING,
    #[serde(rename = "EU_STORAGE_PROCESSING")]
    EUSTORAGEPROCESSING,
    #[serde(rename = "JP_STORAGE")]
    JPSTORAGE,
    #[serde(rename = "KR_STORAGE")]
    KRSTORAGE,
    #[serde(rename = "CA_STORAGE")]
    CASTORAGE,
    #[serde(rename = "SG_STORAGE")]
    SGSTORAGE,
    #[serde(rename = "IN_STORAGE")]
    INSTORAGE,
    #[serde(rename = "AU_STORAGE")]
    AUSTORAGE,
    #[serde(rename = "GB_STORAGE")]
    GBSTORAGE,
    #[serde(rename = "AE_STORAGE")]
    AESTORAGE,
    #[serde(rename = "AE_STORAGE_PROCESSING")]
    AESTORAGEPROCESSING,
}

/// Parameters for updating project data retention controls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProjectDataRetentionBody {
    /// The desired project data retention type.
    pub retention_type: ProjectDataRetentionType,
}

/// Parameters for the hard spend limit you want to create or replace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProjectSpendLimitBody {
    /// The hard spend limit amount, in cents.
    pub threshold_amount: i64,
    /// The currency for the threshold amount. Currently, only `USD` is supported.
    pub currency: UpdateProjectSpendLimitBodyCurrency,
    /// The time interval for evaluating spend against the threshold. Currently, only `month` is supported.
    pub interval: UpdateProjectSpendLimitBodyInterval,
}

/// The currency for the threshold amount. Currently, only `USD` is supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateProjectSpendLimitBodyCurrency {
    #[serde(rename = "USD")]
    USD,
}

/// The time interval for evaluating spend against the threshold. Currently, only `month` is supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateProjectSpendLimitBodyInterval {
    #[serde(rename = "month")]
    Month,
}
