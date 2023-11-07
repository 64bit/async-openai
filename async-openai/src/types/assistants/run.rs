use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::FunctionCall;

use super::AssistantTools;

/// Represents an execution run on a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `assistant.run`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run was created.
    pub created_at: i32,
    ///The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) that was executed on as a part of this run.
    pub thread_id: String,

    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for execution of this run.
    pub assistant_id: Option<String>,

    /// The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, or `expired`.
    pub status: RunStatus,

    /// Details on the action required to continue the run. Will be `null` if no action is required.
    pub required_action: Option<RequiredAction>,

    /// The last error associated with this run. Will be `null` if there are no errors.
    pub last_error: Option<LastError>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub expires_at: i32,
    ///  The Unix timestamp (in seconds) for when the run was started.
    pub started_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub cancelled_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the run failed.
    pub failed_at: Option<i32>,
    ///The Unix timestamp (in seconds) for when the run was completed.
    pub completed_at: Option<i32>,

    /// The model that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub model: String,

    /// The instructions that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub instructions: String,

    /// The list of tools that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub tools: Vec<AssistantTools>,
    /// The list of [File](https://platform.openai.com/docs/api-reference/files) IDs the [assistant](/docs/api-reference/assistants) used for this run.
    pub file_ids: Vec<String>,

    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
    Expired,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RequiredAction {
    /// For now, this is always `submit_tool_outputs`.
    pub r#type: String,

    pub submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct SubmitToolOutputs {
    pub tool_calls: Vec<RunToolCallObject>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunToolCallObject {
    /// The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](/docs/api-reference/runs/submitToolOutputs) endpoint.
    id: String,
    /// The type of tool call the output is required for. For now, this is always `function`.
    r#type: String,
    /// The function definition.
    function: FunctionCall,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct LastError {
    /// One of `server_error` or `rate_limit_exceeded`.
    code: LastErrorCode,
    /// A human-readable description of the error.
    message: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LastErrorCode {
    ServerError,
    RateLimitExceeded,
}
