use serde::{Deserialize, Serialize};

/// The event type.
#[derive(Debug, Serialize, Deserialize)]
pub enum AuditLogEventType {
    #[serde(rename = "api_key.created")]
    ApiKeyCreated,
    #[serde(rename = "api_key.updated")]
    ApiKeyUpdated,
    #[serde(rename = "api_key.deleted")]
    ApiKeyDeleted,
    #[serde(rename = "certificate.created")]
    CertificateCreated,
    #[serde(rename = "certificate.updated")]
    CertificateUpdated,
    #[serde(rename = "certificate.deleted")]
    CertificateDeleted,
    #[serde(rename = "certificates.activated")]
    CertificatesActivated,
    #[serde(rename = "certificates.deactivated")]
    CertificatesDeactivated,
    #[serde(rename = "checkpoint.permission.created")]
    CheckpointPermissionCreated,
    #[serde(rename = "checkpoint.permission.deleted")]
    CheckpointPermissionDeleted,
    #[serde(rename = "external_key.registered")]
    ExternalKeyRegistered,
    #[serde(rename = "external_key.removed")]
    ExternalKeyRemoved,
    #[serde(rename = "group.created")]
    GroupCreated,
    #[serde(rename = "group.updated")]
    GroupUpdated,
    #[serde(rename = "group.deleted")]
    GroupDeleted,
    #[serde(rename = "invite.sent")]
    InviteSent,
    #[serde(rename = "invite.accepted")]
    InviteAccepted,
    #[serde(rename = "invite.deleted")]
    InviteDeleted,
    #[serde(rename = "ip_allowlist.created")]
    IpAllowlistCreated,
    #[serde(rename = "ip_allowlist.updated")]
    IpAllowlistUpdated,
    #[serde(rename = "ip_allowlist.deleted")]
    IpAllowlistDeleted,
    #[serde(rename = "ip_allowlist.config.activated")]
    IpAllowlistConfigActivated,
    #[serde(rename = "ip_allowlist.config.deactivated")]
    IpAllowlistConfigDeactivated,
    #[serde(rename = "login.succeeded")]
    LoginSucceeded,
    #[serde(rename = "login.failed")]
    LoginFailed,
    #[serde(rename = "logout.succeeded")]
    LogoutSucceeded,
    #[serde(rename = "logout.failed")]
    LogoutFailed,
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,
    #[serde(rename = "project.created")]
    ProjectCreated,
    #[serde(rename = "project.updated")]
    ProjectUpdated,
    #[serde(rename = "project.archived")]
    ProjectArchived,
    #[serde(rename = "project.deleted")]
    ProjectDeleted,
    #[serde(rename = "rate_limit.updated")]
    RateLimitUpdated,
    #[serde(rename = "rate_limit.deleted")]
    RateLimitDeleted,
    #[serde(rename = "resource.deleted")]
    ResourceDeleted,
    #[serde(rename = "tunnel.created")]
    TunnelCreated,
    #[serde(rename = "tunnel.updated")]
    TunnelUpdated,
    #[serde(rename = "tunnel.deleted")]
    TunnelDeleted,
    #[serde(rename = "role.created")]
    RoleCreated,
    #[serde(rename = "role.updated")]
    RoleUpdated,
    #[serde(rename = "role.deleted")]
    RoleDeleted,
    #[serde(rename = "role.assignment.created")]
    RoleAssignmentCreated,
    #[serde(rename = "role.assignment.deleted")]
    RoleAssignmentDeleted,
    #[serde(rename = "scim.enabled")]
    ScimEnabled,
    #[serde(rename = "scim.disabled")]
    ScimDisabled,
    #[serde(rename = "service_account.created")]
    ServiceAccountCreated,
    #[serde(rename = "service_account.updated")]
    ServiceAccountUpdated,
    #[serde(rename = "service_account.deleted")]
    ServiceAccountDeleted,
    #[serde(rename = "user.added")]
    UserAdded,
    #[serde(rename = "user.updated")]
    UserUpdated,
    #[serde(rename = "user.deleted")]
    UserDeleted,
    #[serde(rename = "workload_identity_provider.created")]
    WorkloadIdentityProviderCreated,
    #[serde(rename = "workload_identity_provider.updated")]
    WorkloadIdentityProviderUpdated,
    #[serde(rename = "workload_identity_provider.deleted")]
    WorkloadIdentityProviderDeleted,
    #[serde(rename = "workload_identity_provider_mapping.created")]
    WorkloadIdentityProviderMappingCreated,
    #[serde(rename = "workload_identity_provider_mapping.updated")]
    WorkloadIdentityProviderMappingUpdated,
    #[serde(rename = "workload_identity_provider_mapping.deleted")]
    WorkloadIdentityProviderMappingDeleted,
    #[serde(rename = "role.bound_to_resource")]
    RoleBoundToResource,
    #[serde(rename = "role.unbound_from_resource")]
    RoleUnboundFromResource,
    #[serde(rename = "tenant.metadata.updated")]
    TenantMetadataUpdated,
    #[serde(rename = "tenant.microsoft_entra_mapping.upserted")]
    TenantMicrosoftEntraMappingUpserted,
    #[serde(rename = "tenant.microsoft_entra_mapping.deleted")]
    TenantMicrosoftEntraMappingDeleted,
    #[serde(rename = "tenant.workload_identity.provider.created")]
    TenantWorkloadIdentityProviderCreated,
    #[serde(rename = "tenant.workload_identity.provider.updated")]
    TenantWorkloadIdentityProviderUpdated,
    #[serde(rename = "tenant.workload_identity.provider.archived")]
    TenantWorkloadIdentityProviderArchived,
    #[serde(rename = "tenant.workload_identity.mapping.created")]
    TenantWorkloadIdentityMappingCreated,
    #[serde(rename = "tenant.workload_identity.mapping.updated")]
    TenantWorkloadIdentityMappingUpdated,
    #[serde(rename = "tenant.workload_identity.mapping.archived")]
    TenantWorkloadIdentityMappingArchived,
    #[serde(rename = "tenant.workload_identity.binding.created")]
    TenantWorkloadIdentityBindingCreated,
    #[serde(rename = "tenant.workload_identity.principal.provisioned")]
    TenantWorkloadIdentityPrincipalProvisioned,
    #[serde(rename = "tenant.workload_identity.access_token.issued")]
    TenantWorkloadIdentityAccessTokenIssued,
    #[serde(rename = "tenant.admin_api_key.created")]
    TenantAdminApiKeyCreated,
    #[serde(rename = "tenant.admin_api_key.updated")]
    TenantAdminApiKeyUpdated,
    #[serde(rename = "tenant.admin_api_key.deleted")]
    TenantAdminApiKeyDeleted,
    #[serde(rename = "tenant.project_api_key.created")]
    TenantProjectApiKeyCreated,
    #[serde(rename = "tenant.trusted_access.business_verification.started")]
    TenantTrustedAccessBusinessVerificationStarted,
    #[serde(rename = "tenant.trusted_access.application.submitted")]
    TenantTrustedAccessApplicationSubmitted,
    #[serde(rename = "tenant.chatgpt_access_token.revoked")]
    TenantChatgptAccessTokenRevoked,
    #[serde(rename = "tenant.migration.completed")]
    TenantMigrationCompleted,
    #[serde(rename = "tenant.sso.migrated")]
    TenantSsoMigrated,
    #[serde(rename = "tenant.domains.migrated")]
    TenantDomainsMigrated,
    #[serde(rename = "tenant.sso_connection.created")]
    TenantSsoConnectionCreated,
    #[serde(rename = "tenant.sso_connection.updated")]
    TenantSsoConnectionUpdated,
    #[serde(rename = "tenant.sso_connection.deleted")]
    TenantSsoConnectionDeleted,
    #[serde(rename = "tenant.sso_connection.setup.started")]
    TenantSsoConnectionSetupStarted,
    #[serde(rename = "tenant.policy.created")]
    TenantPolicyCreated,
    #[serde(rename = "tenant.policy.updated")]
    TenantPolicyUpdated,
    #[serde(rename = "tenant.policy.deleted")]
    TenantPolicyDeleted,
    #[serde(rename = "tenant.policy.attached")]
    TenantPolicyAttached,
    #[serde(rename = "tenant.policy.detached")]
    TenantPolicyDetached,
    #[serde(rename = "tenant.principal_authentication_policy.resolved")]
    TenantPrincipalAuthenticationPolicyResolved,
    #[serde(rename = "tenant.scim.setup.started")]
    TenantScimSetupStarted,
    #[serde(rename = "tenant.scim.deletion.requested")]
    TenantScimDeletionRequested,
    #[serde(rename = "tenant.scim.directory.created")]
    TenantScimDirectoryCreated,
    #[serde(rename = "tenant.product_access_policy.updated")]
    TenantProductAccessPolicyUpdated,
    #[serde(rename = "tenant.resource_share_grant.created")]
    TenantResourceShareGrantCreated,
    #[serde(rename = "tenant.resource_share_grant.updated")]
    TenantResourceShareGrantUpdated,
    #[serde(rename = "tenant.resource_share_grant.accepted")]
    TenantResourceShareGrantAccepted,
    #[serde(rename = "tenant.resource_share_grant.declined")]
    TenantResourceShareGrantDeclined,
    #[serde(rename = "tenant.resource_share_grant.revoked")]
    TenantResourceShareGrantRevoked,
    #[serde(rename = "tenant.resource_share_grant.deleted")]
    TenantResourceShareGrantDeleted,
    #[serde(rename = "tenant.service_account.updated")]
    TenantServiceAccountUpdated,
    #[serde(rename = "tenant.service_account.deleted")]
    TenantServiceAccountDeleted,
    #[serde(rename = "tenant.service_account.token.revoked")]
    TenantServiceAccountTokenRevoked,
    #[serde(rename = "tenant.billing.overage_limit.updated")]
    TenantBillingOverageLimitUpdated,
    #[serde(rename = "tenant.billing.alerts.updated")]
    TenantBillingAlertsUpdated,
    #[serde(rename = "tenant.billing.info.updated")]
    TenantBillingInfoUpdated,
    #[serde(rename = "tenant.usage_limit.workspace.updated")]
    TenantUsageLimitWorkspaceUpdated,
    #[serde(rename = "tenant.usage_limit.group.updated")]
    TenantUsageLimitGroupUpdated,
    #[serde(rename = "tenant.usage_limit.user.updated")]
    TenantUsageLimitUserUpdated,
    #[serde(rename = "tenant.usage_limit.increase_request.updated")]
    TenantUsageLimitIncreaseRequestUpdated,
    #[serde(rename = "tenant.usage_limit.increase_request.resolved")]
    TenantUsageLimitIncreaseRequestResolved,
    #[serde(rename = "tenant.group.created")]
    TenantGroupCreated,
    #[serde(rename = "tenant.group.updated")]
    TenantGroupUpdated,
    #[serde(rename = "tenant.group.deleted")]
    TenantGroupDeleted,
    #[serde(rename = "tenant.group.member.added")]
    TenantGroupMemberAdded,
    #[serde(rename = "tenant.group.member.removed")]
    TenantGroupMemberRemoved,
    #[serde(rename = "tenant.migration_rollout.status.updated")]
    TenantMigrationRolloutStatusUpdated,
    #[serde(rename = "tenant.migration_rollout.tier.updated")]
    TenantMigrationRolloutTierUpdated,
    #[serde(rename = "tenant.role.metadata.updated")]
    TenantRoleMetadataUpdated,
    #[serde(rename = "tenant.custom_role.created")]
    TenantCustomRoleCreated,
    #[serde(rename = "tenant.custom_role.updated")]
    TenantCustomRoleUpdated,
    #[serde(rename = "tenant.custom_role.deleted")]
    TenantCustomRoleDeleted,
    #[serde(rename = "tenant.role_assignment.created")]
    TenantRoleAssignmentCreated,
    #[serde(rename = "tenant.role_assignment.deleted")]
    TenantRoleAssignmentDeleted,
    #[serde(rename = "tenant.resource_role_assignment.created")]
    TenantResourceRoleAssignmentCreated,
    #[serde(rename = "tenant.resource_role_assignment.deleted")]
    TenantResourceRoleAssignmentDeleted,
    #[serde(rename = "tenant.resource_access.updated")]
    TenantResourceAccessUpdated,
    #[serde(rename = "tenant.resource_access.deleted")]
    TenantResourceAccessDeleted,
    #[serde(rename = "tenant.ads_account.onboarding.redemption")]
    TenantAdsAccountOnboardingRedemption,
    #[serde(rename = "tenant.session_policy.created")]
    TenantSessionPolicyCreated,
    #[serde(rename = "tenant.session_policy.updated")]
    TenantSessionPolicyUpdated,
    #[serde(rename = "tenant.session_policy.deleted")]
    TenantSessionPolicyDeleted,
    #[serde(rename = "tenant.session_revocation.started")]
    TenantSessionRevocationStarted,
    #[serde(rename = "tenant.third_party_app_policy.updated")]
    TenantThirdPartyAppPolicyUpdated,
    #[serde(rename = "tenant.user.added")]
    TenantUserAdded,
    #[serde(rename = "tenant.user.updated")]
    TenantUserUpdated,
    #[serde(rename = "tenant.user.removed")]
    TenantUserRemoved,
    #[serde(rename = "tenant.user.looked_up")]
    TenantUserLookedUp,
    #[serde(rename = "tenant.user.invited")]
    TenantUserInvited,
    #[serde(rename = "tenant.membership.revoked")]
    TenantMembershipRevoked,
    #[serde(rename = "tenant.api_organization_invite.upserted")]
    TenantApiOrganizationInviteUpserted,
    #[serde(rename = "tenant.api_organization_invite.deleted")]
    TenantApiOrganizationInviteDeleted,
    #[serde(rename = "tenant.chatgpt_workspace_invite.upserted")]
    TenantChatgptWorkspaceInviteUpserted,
    #[serde(rename = "tenant.membership.accepted")]
    TenantMembershipAccepted,
    #[serde(rename = "tenant.membership.declined")]
    TenantMembershipDeclined,
    #[serde(rename = "tenant.workspace_invite_email_settings.updated")]
    TenantWorkspaceInviteEmailSettingsUpdated,
}

/// Represents a list of audit logs.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAuditLogsResponse {
    /// The object type, which is always `list`.
    pub object: String,
    /// A list of `AuditLog` objects.
    pub data: Vec<AuditLog>,
    /// The first `audit_log_id` in the retrieved `list`.
    pub first_id: Option<String>,
    /// The last `audit_log_id` in the retrieved `list`.
    pub last_id: Option<String>,
    /// The `has_more` property is used for pagination to indicate there are additional results.
    pub has_more: bool,
}

/// The project that the action was scoped to. Absent for actions not scoped to projects.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProject {
    /// The project ID.
    pub id: String,
    /// The project title.
    pub name: String,
}

/// The actor who performed the audit logged action.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActor {
    /// The type of actor. Is either `session` or `api_key`.
    pub r#type: String,
    /// The session in which the audit logged action was performed.
    pub session: Option<AuditLogActorSession>,
    /// The API Key used to perform the audit logged action.
    pub api_key: Option<AuditLogActorApiKey>,
}

/// The session in which the audit logged action was performed.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorSession {
    /// The user who performed the audit logged action.
    pub user: AuditLogActorUser,
    /// The IP address from which the action was performed.
    pub ip_address: String,
}

/// The API Key used to perform the audit logged action.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorApiKey {
    /// The tracking id of the API key.
    pub id: String,
    /// The type of API key. Can be either `user` or `service_account`.
    pub r#type: AuditLogActorApiKeyType,
    /// The user who performed the audit logged action, if applicable.
    pub user: Option<AuditLogActorUser>,
    /// The service account that performed the audit logged action, if applicable.
    pub service_account: Option<AuditLogActorServiceAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditLogActorApiKeyType {
    User,
    ServiceAccount,
}

/// The user who performed the audit logged action.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorUser {
    /// The user id.
    pub id: String,
    /// The user email.
    pub email: String,
}

/// The service account that performed the audit logged action.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogActorServiceAccount {
    /// The service account id.
    pub id: String,
}

/// A log of a user action or configuration change within this organization.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    /// The ID of this log.
    pub id: String,
    /// The event type.
    pub r#type: AuditLogEventType,
    /// The Unix timestamp (in seconds) of the event.
    pub effective_at: u64,
    /// The project that the action was scoped to. Absent for actions not scoped to projects.
    pub project: Option<AuditLogProject>,
    /// The actor who performed the audit logged action.
    pub actor: Option<AuditLogActor>,
    /// The details for events with the type `api_key.created`.
    #[serde(rename = "api_key.created")]
    pub api_key_created: Option<AuditLogApiKeyCreated>,
    /// The details for events with the type `api_key.updated`.
    #[serde(rename = "api_key.updated")]
    pub api_key_updated: Option<AuditLogApiKeyUpdated>,
    /// The details for events with the type `api_key.deleted`.
    #[serde(rename = "api_key.deleted")]
    pub api_key_deleted: Option<AuditLogApiKeyDeleted>,
    /// The details for events with the type `invite.sent`.
    #[serde(rename = "invite.sent")]
    pub invite_sent: Option<AuditLogInviteSent>,
    /// The details for events with the type `invite.accepted`.
    #[serde(rename = "invite.accepted")]
    pub invite_accepted: Option<AuditLogInviteAccepted>,
    /// The details for events with the type `invite.deleted`.
    #[serde(rename = "invite.deleted")]
    pub invite_deleted: Option<AuditLogInviteDeleted>,
    /// The details for events with the type `login.failed`.
    #[serde(rename = "login.failed")]
    pub login_failed: Option<AuditLogLoginFailed>,
    /// The details for events with the type `logout.failed`.
    #[serde(rename = "logout.failed")]
    pub logout_failed: Option<AuditLogLogoutFailed>,
    /// The details for events with the type `organization.updated`.
    #[serde(rename = "organization.updated")]
    pub organization_updated: Option<AuditLogOrganizationUpdated>,
    /// The details for events with the type `project.created`.
    #[serde(rename = "project.created")]
    pub project_created: Option<AuditLogProjectCreated>,
    /// The details for events with the type `project.updated`.
    #[serde(rename = "project.updated")]
    pub project_updated: Option<AuditLogProjectUpdated>,
    /// The details for events with the type `project.archived`.
    #[serde(rename = "project.archived")]
    pub project_archived: Option<AuditLogProjectArchived>,
    /// The details for events with the type `service_account.created`.
    #[serde(rename = "service_account.created")]
    pub service_account_created: Option<AuditLogServiceAccountCreated>,
    /// The details for events with the type `service_account.updated`.
    #[serde(rename = "service_account.updated")]
    pub service_account_updated: Option<AuditLogServiceAccountUpdated>,
    /// The details for events with the type `service_account.deleted`.
    #[serde(rename = "service_account.deleted")]
    pub service_account_deleted: Option<AuditLogServiceAccountDeleted>,
    /// The details for events with the type `user.added`.
    #[serde(rename = "user.added")]
    pub user_added: Option<AuditLogUserAdded>,
    /// The details for events with the type `user.updated`.
    #[serde(rename = "user.updated")]
    pub user_updated: Option<AuditLogUserUpdated>,
    /// The details for events with the type `user.deleted`.
    #[serde(rename = "user.deleted")]
    pub user_deleted: Option<AuditLogUserDeleted>,
    /// The details for events with this `type`.
    #[serde(rename = "role.bound_to_resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_bound_to_resource: Option<AuditLogRoleBoundToResource>,

    /// The details for events with this `type`.
    #[serde(rename = "role.unbound_from_resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_unbound_from_resource: Option<AuditLogRoleUnboundFromResource>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider.created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_created: Option<AuditLogWorkloadIdentityProviderCreated>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider.updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_updated: Option<AuditLogWorkloadIdentityProviderUpdated>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider.deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_deleted: Option<AuditLogWorkloadIdentityProviderDeleted>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider_mapping.created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_mapping_created:
        Option<AuditLogWorkloadIdentityProviderMappingCreated>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider_mapping.updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_mapping_updated:
        Option<AuditLogWorkloadIdentityProviderMappingUpdated>,

    /// The details for events with this `type`.
    #[serde(rename = "workload_identity_provider_mapping.deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_mapping_deleted:
        Option<AuditLogWorkloadIdentityProviderMappingDeleted>,
}

/// The details for events with the type `api_key.created`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyCreated {
    /// The tracking ID of the API key.
    pub id: String,
    /// The payload used to create the API key.
    pub data: Option<AuditLogApiKeyCreatedData>,
}

/// The payload used to create the API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyCreatedData {
    /// A list of scopes allowed for the API key, e.g. `["api.model.request"]`.
    pub scopes: Option<Vec<String>>,
}

/// The details for events with the type `api_key.updated`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyUpdated {
    /// The tracking ID of the API key.
    pub id: String,
    /// The payload used to update the API key.
    pub changes_requested: Option<AuditLogApiKeyUpdatedChangesRequested>,
}

/// The payload used to update the API key.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyUpdatedChangesRequested {
    /// A list of scopes allowed for the API key, e.g. `["api.model.request"]`.
    pub scopes: Option<Vec<String>>,
}

/// The details for events with the type `api_key.deleted`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyDeleted {
    /// The tracking ID of the API key.
    pub id: String,
}

/// The details for events with the type `invite.sent`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogInviteSent {
    /// The ID of the invite.
    pub id: String,
    /// The payload used to create the invite.
    pub data: Option<AuditLogInviteSentData>,
}

/// The payload used to create the invite.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogInviteSentData {
    /// The email invited to the organization.
    pub email: String,
    /// The role the email was invited to be. Is either `owner` or `member`.
    pub role: String,
}

/// The details for events with the type `invite.accepted`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogInviteAccepted {
    /// The ID of the invite.
    pub id: String,
}

/// The details for events with the type `invite.deleted`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogInviteDeleted {
    /// The ID of the invite.
    pub id: String,
}

/// The details for events with the type `login.failed`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogLoginFailed {
    /// The error code of the failure.
    pub error_code: String,
    /// The error message of the failure.
    pub error_message: String,
}

/// The details for events with the type `logout.failed`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogLogoutFailed {
    /// The error code of the failure.
    pub error_code: String,
    /// The error message of the failure.
    pub error_message: String,
}

/// The details for events with the type `organization.updated`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogOrganizationUpdated {
    /// The organization ID.
    pub id: String,
    /// The payload used to update the organization settings.
    pub changes_requested: Option<AuditLogOrganizationUpdatedChangesRequested>,
}

/// The payload used to update the organization settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogOrganizationUpdatedChangesRequested {
    /// The organization title.
    pub title: Option<String>,
    /// The organization description.
    pub description: Option<String>,
    /// The organization name.
    pub name: Option<String>,
    /// The organization settings.
    pub settings: Option<AuditLogOrganizationUpdatedChangesRequestedSettings>,
}

/// The organization settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogOrganizationUpdatedChangesRequestedSettings {
    /// Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`.
    pub threads_ui_visibility: Option<String>,
    /// Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`.
    pub usage_dashboard_visibility: Option<String>,
}

/// The details for events with the type `project.created`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectCreated {
    /// The project ID.
    pub id: String,
    /// The payload used to create the project.
    pub data: Option<AuditLogProjectCreatedData>,
}

/// The payload used to create the project.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectCreatedData {
    /// The project name.
    pub name: String,
    /// The title of the project as seen on the dashboard.
    pub title: Option<String>,
}

/// The details for events with the type `project.updated`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectUpdated {
    /// The project ID.
    pub id: String,
    /// The payload used to update the project.
    pub changes_requested: Option<AuditLogProjectUpdatedChangesRequested>,
}

/// The payload used to update the project.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectUpdatedChangesRequested {
    /// The title of the project as seen on the dashboard.
    pub title: Option<String>,
}

/// The details for events with the type `project.archived`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogProjectArchived {
    /// The project ID.
    pub id: String,
}

/// The details for events with the type `service_account.created`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountCreated {
    /// The service account ID.
    pub id: String,
    /// The payload used to create the service account.
    pub data: Option<AuditLogServiceAccountCreatedData>,
}

/// The payload used to create the service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountCreatedData {
    /// The role of the service account. Is either `owner` or `member`.
    pub role: String,
}

/// The details for events with the type `service_account.updated`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountUpdated {
    /// The service account ID.
    pub id: String,
    /// The payload used to updated the service account.
    pub changes_requested: Option<AuditLogServiceAccountUpdatedChangesRequested>,
}

/// The payload used to updated the service account.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
    /// The role of the service account. Is either `owner` or `member`.
    pub role: String,
}

/// The details for events with the type `service_account.deleted`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogServiceAccountDeleted {
    /// The service account ID.
    pub id: String,
}

/// The details for events with the type `user.added`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserAdded {
    /// The user ID.
    pub id: String,
    /// The payload used to add the user to the project.
    pub data: Option<AuditLogUserAddedData>,
}

/// The payload used to add the user to the project.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserAddedData {
    /// The role of the user. Is either `owner` or `member`.
    pub role: String,
}

/// The details for events with the type `user.updated`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserUpdated {
    /// The project ID.
    pub id: String,
    /// The payload used to update the user.
    pub changes_requested: Option<AuditLogUserUpdatedChangesRequested>,
}

/// The payload used to update the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserUpdatedChangesRequested {
    /// The role of the user. Is either `owner` or `member`.
    pub role: String,
}

/// The details for events with the type `user.deleted`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogUserDeleted {
    /// The user ID.
    pub id: String,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogRoleBoundToResource {
    /// The ID of the resource the role was bound to. ChatGPT workspace connector resources use
    /// `<workspace_id>__<connector_id>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the role that was bound to the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// The ID of the resource the role was bound to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The type of resource the role was bound to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The permissions granted to the role for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// The workspace ID for a ChatGPT workspace connector resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The connector ID for a ChatGPT workspace connector resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// The connector display name for a ChatGPT workspace connector resource, or the connector ID when the
    /// display name could not be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    /// Whether the connector is enabled for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The connector role mutation path that produced the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AuditLogRoleBoundToResourceSource>,
}

/// The connector role mutation path that produced the event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditLogRoleBoundToResourceSource {
    #[serde(rename = "role_toggle")]
    RoleToggle,
    #[serde(rename = "role_connector_update")]
    RoleConnectorUpdate,
    #[serde(rename = "role_delete")]
    RoleDelete,
    #[serde(rename = "workspace_permissions")]
    WorkspacePermissions,
    #[serde(rename = "connector_publish")]
    ConnectorPublish,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogRoleUnboundFromResource {
    /// The ID of the resource the role was unbound from. ChatGPT workspace connector resources use
    /// `<workspace_id>__<connector_id>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the role that was unbound from the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// The ID of the resource the role was unbound from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The type of resource the role was unbound from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The permissions remaining for the role after the change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// The workspace ID for a ChatGPT workspace connector resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The connector ID for a ChatGPT workspace connector resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// The connector display name for a ChatGPT workspace connector resource, or the connector ID when the
    /// display name could not be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    /// Whether the connector is enabled for the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The connector role mutation path that produced the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AuditLogRoleUnboundFromResourceSource>,
}

/// The connector role mutation path that produced the event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditLogRoleUnboundFromResourceSource {
    #[serde(rename = "role_toggle")]
    RoleToggle,
    #[serde(rename = "role_connector_update")]
    RoleConnectorUpdate,
    #[serde(rename = "role_delete")]
    RoleDelete,
    #[serde(rename = "workspace_permissions")]
    WorkspacePermissions,
    #[serde(rename = "connector_publish")]
    ConnectorPublish,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderCreated {
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The payload used to create the workload identity provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderDeleted {
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The workload identity provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingCreated {
    /// The workload identity provider mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<String>,
    /// The payload used to create the workload identity provider mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingDeleted {
    /// The workload identity provider mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<String>,
    /// The project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The mapped service account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingUpdated {
    /// The workload identity provider mapping ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<String>,
    /// The payload used to update the workload identity provider mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes_requested: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// The details for events with this `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderUpdated {
    /// The workload identity provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The payload used to update the workload identity provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes_requested: Option<std::collections::HashMap<String, serde_json::Value>>,
}
