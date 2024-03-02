use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{ImageFile, LastError, RunStatus};

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RunStepType {
    MessageCreation,
    ToolCalls,
}

/// Represents a step in execution of a run.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepObject {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,
    /// The object type, which is always `thread.run.step`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the run step was created.
    pub created_at: i32,

    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) associated with the run step.
    pub assistant_id: Option<String>,

    /// The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) that was run.
    pub thread_id: String,

    ///  The ID of the [run](https://platform.openai.com/docs/api-reference/runs) that this run step is a part of.
    pub run_id: String,

    /// The type of run step, which can be either `message_creation` or `tool_calls`.
    pub r#type: RunStepType,

    /// The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`.
    pub status: RunStatus,

    /// The details of the run step.
    pub step_details: StepDetails,

    /// The last error associated with this run. Will be `null` if there are no errors.
    pub last_error: Option<LastError>,

    ///The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired.
    pub expired_at: Option<i32>,

    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    pub cancelled_at: Option<i32>,

    /// The Unix timestamp (in seconds) for when the run step failed.
    pub failed_at: Option<i32>,

    /// The Unix timestamp (in seconds) for when the run step completed.
    pub completed_at: Option<i32>,

    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`.
    pub usage: Option<RunStepCompletionUsage>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepCompletionUsage {
    /// Number of completion tokens used over the course of the run step.
    pub completion_tokens: u32,
    /// Number of prompt tokens used over the course of the run step.
    pub prompt_tokens: u32,
    /// Total number of tokens used (prompt + completion).
    pub total_tokens: u32,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StepDetails {
    MessageCreation(RunStepDetailsMessageCreationObject),
    ToolCalls(RunStepDetailsToolCallsObject),
}

/// Details of the message creation by the run step.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsMessageCreationObject {
    /// Always `message_creation`.
    pub r#type: String,
    pub message_creation: MessageCreation,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct MessageCreation {
    /// The ID of the message that was created by this run step.
    pub message_id: String,
}

/// Details of the tool call.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsObject {
    /// Always `tool_calls`.
    pub r#type: String,
    ///  An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `retrieval`, or `function`.
    pub tool_calls: Vec<RunStepDetailsToolCalls>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RunStepDetailsToolCalls {
    /// Details of the Code Interpreter tool call the run step was involved in.
    Code(RunStepDetailsToolCallsCodeObject),

    Retrieval(RunStepDetailsToolCallsRetrievalObject),
    Function(RunStepDetailsToolCallsFunctionObject),
}

/// Code interpreter tool call
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsCodeObject {
    /// The ID of the tool call.
    pub id: String,
    /// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
    pub r#type: String,

    /// The Code Interpreter tool call definition.
    pub code_interpreter: CodeInterpreter,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct CodeInterpreter {
    /// The input to the Code Interpreter tool call.
    pub input: String,
    /// The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type.
    pub outputs: Vec<CodeInterpreterOutput>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CodeInterpreterOutput {
    /// Code interpreter log output
    Log(RunStepDetailsToolCallsCodeOutputLogsObject),
    /// Code interpreter image output
    Image(RunStepDetailsToolCallsCodeOutputImageObject),
}

/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    /// Always `logs`.
    pub r#type: String,
    /// The text output from the Code Interpreter tool call.
    pub logs: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    ///  Always `image`.
    pub r#type: String,
    /// The [file](https://platform.openai.com/docs/api-reference/files) ID of the image.
    pub image: ImageFile,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsRetrievalObject {
    /// The ID of the tool call object.
    pub id: String,
    /// The type of tool call. This is always going to be `retrieval` for this type of tool call.
    pub r#type: String,
    /// For now, this is always going to be an empty object.
    pub retrieval: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepDetailsToolCallsFunctionObject {
    /// The ID of the tool call object.
    pub id: String,
    /// The type of tool call. This is always going to be `function` for this type of tool call.
    pub r#type: String,
    /// he definition of the function that was called.
    pub function: RunStepFunctionObject,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct RunStepFunctionObject {
    /// The name of the function.
    pub name: String,
    /// The arguments passed to the function.
    pub arguments: String,
    /// The output of the function. This will be `null` if the outputs have not been [submitted](https://platform.openai.com/docs/api-reference/runs/submitToolOutputs) yet.
    pub output: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ListRunStepsResponse {
    pub object: String,
    pub data: Vec<RunStepObject>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}
