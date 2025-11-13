use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

/// Represents a ChatKit session and its resolved configuration.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionResource {
    /// Identifier for the ChatKit session.
    pub id: String,
    /// Type discriminator that is always `chatkit.session`.
    #[serde(default = "default_session_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the session expires.
    pub expires_at: u64,
    /// Ephemeral client secret that authenticates session requests.
    pub client_secret: String,
    /// Workflow metadata for the session.
    pub workflow: ChatkitWorkflow,
    /// User identifier associated with the session.
    pub user: String,
    /// Resolved rate limit values.
    pub rate_limits: ChatSessionRateLimits,
    /// Convenience copy of the per-minute request limit.
    pub max_requests_per_1_minute: u32,
    /// Current lifecycle state of the session.
    pub status: ChatSessionStatus,
    /// Resolved ChatKit feature configuration for the session.
    pub chatkit_configuration: ChatSessionChatkitConfiguration,
}

fn default_session_object() -> String {
    "chatkit.session".to_string()
}

/// Workflow metadata and state returned for the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatkitWorkflow {
    /// Identifier of the workflow backing the session.
    pub id: String,
    /// Specific workflow version used for the session. Defaults to null when using the latest deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// State variable key-value pairs applied when invoking the workflow. Defaults to null when no overrides were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_variables: Option<HashMap<String, serde_json::Value>>,
    /// Tracing settings applied to the workflow.
    pub tracing: ChatkitWorkflowTracing,
}

/// Controls diagnostic tracing during the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatkitWorkflowTracing {
    /// Indicates whether tracing is enabled.
    pub enabled: bool,
}

/// Active per-minute request limit for the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionRateLimits {
    /// Maximum allowed requests per one-minute window.
    pub max_requests_per_1_minute: u32,
}

/// Current lifecycle state of the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatSessionStatus {
    Active,
    Expired,
    Cancelled,
}

/// ChatKit configuration for the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionChatkitConfiguration {
    /// Automatic thread titling preferences.
    pub automatic_thread_titling: ChatSessionAutomaticThreadTitling,
    /// Upload settings for the session.
    pub file_upload: ChatSessionFileUpload,
    /// History retention configuration.
    pub history: ChatSessionHistory,
}

/// Automatic thread title preferences for the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionAutomaticThreadTitling {
    /// Whether automatic thread titling is enabled.
    pub enabled: bool,
}

/// Upload permissions and limits applied to the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionFileUpload {
    /// Indicates if uploads are enabled for the session.
    pub enabled: bool,
    /// Maximum upload size in megabytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<u32>,
    /// Maximum number of uploads allowed during the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files: Option<u32>,
}

/// History retention preferences returned for the session.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatSessionHistory {
    /// Indicates if chat history is persisted for the session.
    pub enabled: bool,
    /// Number of prior threads surfaced in history views. Defaults to null when all history is retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_threads: Option<u32>,
}

/// Parameters for provisioning a new ChatKit session.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "CreateChatSessionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateChatSessionBody {
    /// Workflow that powers the session.
    pub workflow: WorkflowParam,
    /// A free-form string that identifies your end user; ensures this Session can access other objects that have the same `user` scope.
    pub user: String,
    /// Optional override for session expiration timing in seconds from creation. Defaults to 10 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpiresAfterParam>,
    /// Optional override for per-minute request limits. When omitted, defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<RateLimitsParam>,
    /// Optional overrides for ChatKit runtime configuration features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatkit_configuration: Option<ChatkitConfigurationParam>,
}

/// Workflow reference and overrides applied to the chat session.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "WorkflowParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct WorkflowParam {
    /// Identifier for the workflow invoked by the session.
    pub id: String,
    /// Specific workflow version to run. Defaults to the latest deployed version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// State variables forwarded to the workflow. Keys may be up to 64 characters, values must be primitive types, and the map defaults to an empty object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_variables: Option<HashMap<String, serde_json::Value>>,
    /// Optional tracing overrides for the workflow invocation. When omitted, tracing is enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<WorkflowTracingParam>,
}

/// Controls diagnostic tracing during the session.
#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "WorkflowTracingParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct WorkflowTracingParam {
    /// Whether tracing is enabled during the session. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Controls when the session expires relative to an anchor timestamp.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "ExpiresAfterParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ExpiresAfterParam {
    /// Base timestamp used to calculate expiration. Currently fixed to `created_at`.
    #[serde(default = "default_anchor")]
    #[builder(default = "default_anchor()")]
    pub anchor: String,
    /// Number of seconds after the anchor when the session expires.
    pub seconds: u32,
}

fn default_anchor() -> String {
    "created_at".to_string()
}

/// Controls request rate limits for the session.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "RateLimitsParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct RateLimitsParam {
    /// Maximum number of requests allowed per minute for the session. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_1_minute: Option<u32>,
}

/// Optional per-session configuration settings for ChatKit behavior.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "ChatkitConfigurationParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatkitConfigurationParam {
    /// Configuration for automatic thread titling. When omitted, automatic thread titling is enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_thread_titling: Option<AutomaticThreadTitlingParam>,
    /// Configuration for upload enablement and limits. When omitted, uploads are disabled by default (max_files 10, max_file_size 512 MB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload: Option<FileUploadParam>,
    /// Configuration for chat history retention. When omitted, history is enabled by default with no limit on recent_threads (null).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryParam>,
}

/// Controls whether ChatKit automatically generates thread titles.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "AutomaticThreadTitlingParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct AutomaticThreadTitlingParam {
    /// Enable automatic thread title generation. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Controls whether users can upload files.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "FileUploadParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct FileUploadParam {
    /// Enable uploads for this session. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Maximum size in megabytes for each uploaded file. Defaults to 512 MB, which is the maximum allowable size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<u32>,
    /// Maximum number of files that can be uploaded to the session. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files: Option<u32>,
}

/// Controls how much historical context is retained for the session.
#[derive(Clone, Serialize, Debug, Deserialize, Builder, PartialEq, Default)]
#[builder(name = "HistoryParamArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct HistoryParam {
    /// Enables chat users to access previous ChatKit threads. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Number of recent ChatKit threads users have access to. Defaults to unlimited when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_threads: Option<u32>,
}
