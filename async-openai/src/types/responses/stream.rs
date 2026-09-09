use serde::{Deserialize, Serialize};

use crate::types::responses::{
    Annotation, FunctionShellCallOutputContent, OutputContent, OutputItem, Response,
    ResponseLogProb, SummaryPart,
};

/// Event types for streaming responses from the Responses API
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseStreamEvent {
    /// An event that is emitted when a response is created.
    #[serde(rename = "response.created")]
    ResponseCreated(ResponseCreatedEvent),
    /// Emitted when the response is in progress.
    #[serde(rename = "response.in_progress")]
    ResponseInProgress(ResponseInProgressEvent),
    /// Emitted when the model response is complete.
    #[serde(rename = "response.completed")]
    ResponseCompleted(ResponseCompletedEvent),
    /// An event that is emitted when a response fails.
    #[serde(rename = "response.failed")]
    ResponseFailed(ResponseFailedEvent),
    /// An event that is emitted when a response finishes as incomplete.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete(ResponseIncompleteEvent),
    /// Emitted when a new output item is added.
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(ResponseOutputItemAddedEvent),
    /// Emitted when an output item is marked done.
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(ResponseOutputItemDoneEvent),
    /// Emitted when a new content part is added.
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(ResponseContentPartAddedEvent),
    /// Emitted when a content part is done.
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(ResponseContentPartDoneEvent),
    /// Emitted when there is an additional text delta.
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta(ResponseTextDeltaEvent),
    /// Emitted when text content is finalized.
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone(ResponseTextDoneEvent),
    /// Emitted when there is a partial refusal text.
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta(ResponseRefusalDeltaEvent),
    #[serde(rename = "response.refusal.done")]
    /// Emitted when refusal text is finalized.
    ResponseRefusalDone(ResponseRefusalDoneEvent),
    /// Emitted when there is a partial function-call arguments delta.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDeltaEvent),
    /// Emitted when function-call arguments are finalized.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDoneEvent),
    /// Emitted when a file search call is initiated.
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInProgressEvent),
    /// Emitted when a file search is currently searching.
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching(ResponseFileSearchCallSearchingEvent),
    /// Emitted when a file search call is completed (results found).
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted(ResponseFileSearchCallCompletedEvent),
    /// Emitted when a web search call is initiated.
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress(ResponseWebSearchCallInProgressEvent),
    /// Emitted when a web search call is executing.
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching(ResponseWebSearchCallSearchingEvent),
    /// Emitted when a web search call is completed.
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted(ResponseWebSearchCallCompletedEvent),
    /// Emitted when a new reasoning summary part is added.
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded(ResponseReasoningSummaryPartAddedEvent),
    /// Emitted when a reasoning summary part is completed.
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone(ResponseReasoningSummaryPartDoneEvent),
    /// Emitted when a delta is added to a reasoning summary text.
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta(ResponseReasoningSummaryTextDeltaEvent),
    /// Emitted when a reasoning summary text is completed.
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone(ResponseReasoningSummaryTextDoneEvent),
    /// Emitted when a delta is added to a reasoning text.
    #[serde(rename = "response.reasoning_text.delta")]
    ResponseReasoningTextDelta(ResponseReasoningTextDeltaEvent),
    /// Emitted when a reasoning text is completed.
    #[serde(rename = "response.reasoning_text.done")]
    ResponseReasoningTextDone(ResponseReasoningTextDoneEvent),
    /// Emitted when an image generation tool call has completed and the final image is available.
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenerationCallCompleted(ResponseImageGenCallCompletedEvent),
    /// Emitted when an image generation tool call is actively generating an image (intermediate state).
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenerationCallGenerating(ResponseImageGenCallGeneratingEvent),
    /// Emitted when an image generation tool call is in progress.
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenerationCallInProgress(ResponseImageGenCallInProgressEvent),
    /// Emitted when a partial image is available during image generation streaming.
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenerationCallPartialImage(ResponseImageGenCallPartialImageEvent),
    /// Emitted when there is a delta (partial update) to the arguments of an MCP tool call.
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMCPCallArgumentsDelta(ResponseMCPCallArgumentsDeltaEvent),
    /// Emitted when the arguments for an MCP tool call are finalized.
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMCPCallArgumentsDone(ResponseMCPCallArgumentsDoneEvent),
    /// Emitted when an MCP tool call has completed successfully.
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMCPCallCompleted(ResponseMCPCallCompletedEvent),
    /// Emitted when an MCP tool call has failed.
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMCPCallFailed(ResponseMCPCallFailedEvent),
    /// Emitted when an MCP tool call is in progress.
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMCPCallInProgress(ResponseMCPCallInProgressEvent),
    /// Emitted when the list of available MCP tools has been successfully retrieved.
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMCPListToolsCompleted(ResponseMCPListToolsCompletedEvent),
    /// Emitted when the attempt to list available MCP tools has failed.
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMCPListToolsFailed(ResponseMCPListToolsFailedEvent),
    /// Emitted when the system is in the process of retrieving the list of available MCP tools.
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMCPListToolsInProgress(ResponseMCPListToolsInProgressEvent),
    /// Emitted when a code interpreter call is in progress.
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress(ResponseCodeInterpreterCallInProgressEvent),
    /// Emitted when the code interpreter is actively interpreting the code snippet.
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallInterpretingEvent),
    /// Emitted when the code interpreter call is completed.
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallCompletedEvent),
    /// Emitted when a partial code snippet is streamed by the code interpreter.
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeDeltaEvent),
    /// Emitted when the code snippet is finalized by the code interpreter.
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeDoneEvent),
    /// Emitted when an annotation is added to output text content.
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded(ResponseOutputTextAnnotationAddedEvent),
    /// Emitted when a response is queued and waiting to be processed.
    #[serde(rename = "response.queued")]
    ResponseQueued(ResponseQueuedEvent),
    /// Event representing a delta (partial update) to the input of a custom tool call.
    #[serde(rename = "response.custom_tool_call_input.delta")]
    ResponseCustomToolCallInputDelta(ResponseCustomToolCallInputDeltaEvent),
    /// Event indicating that input for a custom tool call is complete.
    #[serde(rename = "response.custom_tool_call_input.done")]
    ResponseCustomToolCallInputDone(ResponseCustomToolCallInputDoneEvent),
    /// Emitted when an error occurs.
    #[serde(rename = "error")]
    ResponseError(ResponseErrorEvent),
    #[serde(rename = "response.shell_call_command.added")]
    ResponseShellCallCommandAdded(ResponseShellCallCommandAddedStreamingEvent),
    #[serde(rename = "response.shell_call_command.delta")]
    ResponseShellCallCommandDelta(ResponseShellCallCommandDeltaStreamingEvent),
    #[serde(rename = "response.shell_call_command.done")]
    ResponseShellCallCommandDone(ResponseShellCallCommandDoneStreamingEvent),
    #[serde(rename = "response.shell_call_output_content.delta")]
    ResponseShellCallOutputContentDelta(ResponseShellCallOutputContentDeltaStreamingEvent),
    #[serde(rename = "response.shell_call_output_content.done")]
    ResponseShellCallOutputContentDone(ResponseShellCallOutputContentDoneStreamingEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCreatedEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseInProgressEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCompletedEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFailedEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseIncompleteEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseOutputItemAddedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    /// The output item that was added. For reasoning items, `encrypted_content`
    /// may be incomplete while the item is in progress. Use the reasoning item
    /// from the corresponding `response.output_item.done` event when passing it
    /// as input to a subsequent request.
    pub item: OutputItem,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseOutputItemDoneEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item: OutputItem,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseContentPartAddedEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: OutputContent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseContentPartDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: OutputContent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseTextDeltaEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<ResponseLogProb>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseTextDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub text: String,
    pub logprobs: Option<Vec<ResponseLogProb>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseRefusalDeltaEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseRefusalDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub refusal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    /// <https://github.com/64bit/async-openai/issues/472>
    pub name: Option<String>,
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFileSearchCallInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFileSearchCallSearchingEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseFileSearchCallCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseWebSearchCallInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseWebSearchCallSearchingEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseWebSearchCallCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningSummaryPartAddedEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub part: SummaryPart,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningSummaryPartDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub part: SummaryPart,
    /// The completion status of the summary part. Omitted when the part completed
    /// normally and set to `incomplete` when generation was interrupted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningSummaryTextDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningTextDeltaEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseReasoningTextDoneEvent {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseImageGenCallCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseImageGenCallGeneratingEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseImageGenCallInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseImageGenCallPartialImageEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub partial_image_index: u32,
    pub partial_image_b64: String,
    /// The image size that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    /// The image quality that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,

    /// The background setting that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    /// The output format that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPCallArgumentsDeltaEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPCallArgumentsDoneEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPCallCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPCallFailedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPCallInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPListToolsCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPListToolsFailedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseMCPListToolsInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseOutputTextAnnotationAddedEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub content_index: u32,
    pub annotation_index: u32,
    pub item_id: String,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseQueuedEvent {
    pub sequence_number: u64,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCustomToolCallInputDeltaEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseCustomToolCallInputDoneEvent {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseErrorEvent {
    pub sequence_number: u64,
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
}

/// A streaming event that indicated a shell command was added to a tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandAddedStreamingEvent {
    /// The sequence number of the event that was emitted.
    pub sequence_number: u64,
    /// The index of the output item that was updated.
    pub output_index: u32,
    /// The index of the shell command that was added.
    pub command_index: u32,
    /// The shell command that was added.
    pub command: String,
}

/// A streaming event that indicated a shell command was incrementally updated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandDeltaStreamingEvent {
    /// The sequence number of the event that was emitted.
    pub sequence_number: u64,
    /// The index of the output item that was updated.
    pub output_index: u32,
    /// The index of the shell command that was updated.
    pub command_index: u32,
    /// The shell command delta that was appended.
    pub delta: String,
    /// An obfuscation string that was added to pad the event payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// A streaming event that indicated a shell command was completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandDoneStreamingEvent {
    /// The sequence number of the event that was emitted.
    pub sequence_number: u64,
    /// The index of the output item that was updated.
    pub output_index: u32,
    /// The index of the shell command that was completed.
    pub command_index: u32,
    /// The final shell command that was emitted.
    pub command: String,
}

/// A streaming event that indicated shell call output was incrementally added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallOutputContentDeltaStreamingEvent {
    /// The sequence number of the event that was emitted.
    pub sequence_number: u64,
    /// The ID of the output item that was updated.
    pub item_id: String,
    /// The index of the output item that was updated.
    pub output_index: u32,
    /// The index of the shell command that produced output.
    pub command_index: u32,
    /// The stdout/stderr delta that was emitted.
    pub delta: ShellCallOutputDelta,
}

/// A streaming event that indicated shell call output was completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallOutputContentDoneStreamingEvent {
    /// The sequence number of the event that was emitted.
    pub sequence_number: u64,
    /// The ID of the output item that was updated.
    pub item_id: String,
    /// The index of the output item that was updated.
    pub output_index: u32,
    /// The index of the shell command that produced output.
    pub command_index: u32,
    /// The output contents emitted for the shell command.
    pub output: Vec<FunctionShellCallOutputContent>,
}

/// A delta of stdout/stderr emitted while a shell call was running.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShellCallOutputDelta {
    /// The stdout delta that was emitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    /// The stderr delta that was emitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
}

/// Stream of response events
#[cfg(feature = "_api")]
pub type ResponseStream = crate::types::stream::StreamResponse<ResponseStreamEvent>;

// Implement EventType trait for all event types in this file
#[cfg(feature = "_api")]
macro_rules! impl_event_type {
    ($($ty:ty => $event_type:expr),* $(,)?) => {
        $(
            impl crate::traits::EventType for $ty {
                fn event_type(&self) -> &'static str {
                    $event_type
                }
            }
        )*
    };
}

// Apply macro for each event struct type in this file.
#[cfg(feature = "_api")]
impl_event_type! {
    ResponseShellCallCommandAddedStreamingEvent => "response.shell_call_command.added",
    ResponseShellCallCommandDeltaStreamingEvent => "response.shell_call_command.delta",
    ResponseShellCallCommandDoneStreamingEvent => "response.shell_call_command.done",
    ResponseShellCallOutputContentDeltaStreamingEvent => "response.shell_call_output_content.delta",
    ResponseShellCallOutputContentDoneStreamingEvent => "response.shell_call_output_content.done",
    ResponseCreatedEvent => "response.created",
    ResponseInProgressEvent => "response.in_progress",
    ResponseCompletedEvent => "response.completed",
    ResponseFailedEvent => "response.failed",
    ResponseIncompleteEvent => "response.incomplete",
    ResponseOutputItemAddedEvent => "response.output_item.added",
    ResponseOutputItemDoneEvent => "response.output_item.done",
    ResponseContentPartAddedEvent => "response.content_part.added",
    ResponseContentPartDoneEvent => "response.content_part.done",
    ResponseTextDeltaEvent => "response.output_text.delta",
    ResponseTextDoneEvent => "response.output_text.done",
    ResponseRefusalDeltaEvent => "response.refusal.delta",
    ResponseRefusalDoneEvent => "response.refusal.done",
    ResponseFunctionCallArgumentsDeltaEvent => "response.function_call_arguments.delta",
    ResponseFunctionCallArgumentsDoneEvent => "response.function_call_arguments.done",
    ResponseFileSearchCallInProgressEvent => "response.file_search_call.in_progress",
    ResponseFileSearchCallSearchingEvent => "response.file_search_call.searching",
    ResponseFileSearchCallCompletedEvent => "response.file_search_call.completed",
    ResponseWebSearchCallInProgressEvent => "response.web_search_call.in_progress",
    ResponseWebSearchCallSearchingEvent => "response.web_search_call.searching",
    ResponseWebSearchCallCompletedEvent => "response.web_search_call.completed",
    ResponseReasoningSummaryPartAddedEvent => "response.reasoning_summary_part.added",
    ResponseReasoningSummaryPartDoneEvent => "response.reasoning_summary_part.done",
    ResponseReasoningSummaryTextDeltaEvent => "response.reasoning_summary_text.delta",
    ResponseReasoningSummaryTextDoneEvent => "response.reasoning_summary_text.done",
    ResponseReasoningTextDeltaEvent => "response.reasoning_text.delta",
    ResponseReasoningTextDoneEvent => "response.reasoning_text.done",
    ResponseImageGenCallCompletedEvent => "response.image_generation_call.completed",
    ResponseImageGenCallGeneratingEvent => "response.image_generation_call.generating",
    ResponseImageGenCallInProgressEvent => "response.image_generation_call.in_progress",
    ResponseImageGenCallPartialImageEvent => "response.image_generation_call.partial_image",
    ResponseMCPCallArgumentsDeltaEvent => "response.mcp_call_arguments.delta",
    ResponseMCPCallArgumentsDoneEvent => "response.mcp_call_arguments.done",
    ResponseMCPCallCompletedEvent => "response.mcp_call.completed",
    ResponseMCPCallFailedEvent => "response.mcp_call.failed",
    ResponseMCPCallInProgressEvent => "response.mcp_call.in_progress",
    ResponseMCPListToolsCompletedEvent => "response.mcp_list_tools.completed",
    ResponseMCPListToolsFailedEvent => "response.mcp_list_tools.failed",
    ResponseMCPListToolsInProgressEvent => "response.mcp_list_tools.in_progress",
    ResponseCodeInterpreterCallInProgressEvent => "response.code_interpreter_call.in_progress",
    ResponseCodeInterpreterCallInterpretingEvent => "response.code_interpreter_call.interpreting",
    ResponseCodeInterpreterCallCompletedEvent => "response.code_interpreter_call.completed",
    ResponseCodeInterpreterCallCodeDeltaEvent => "response.code_interpreter_call_code.delta",
    ResponseCodeInterpreterCallCodeDoneEvent => "response.code_interpreter_call_code.done",
    ResponseOutputTextAnnotationAddedEvent => "response.output_text.annotation.added",
    ResponseQueuedEvent => "response.queued",
    ResponseCustomToolCallInputDeltaEvent => "response.custom_tool_call_input.delta",
    ResponseCustomToolCallInputDoneEvent => "response.custom_tool_call_input.done",
    ResponseErrorEvent => "error",
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for ResponseStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::ResponseShellCallCommandAdded(event) => event.event_type(),
            Self::ResponseShellCallCommandDelta(event) => event.event_type(),
            Self::ResponseShellCallCommandDone(event) => event.event_type(),
            Self::ResponseShellCallOutputContentDelta(event) => event.event_type(),
            Self::ResponseShellCallOutputContentDone(event) => event.event_type(),
            Self::ResponseCreated(event) => event.event_type(),
            Self::ResponseInProgress(event) => event.event_type(),
            Self::ResponseCompleted(event) => event.event_type(),
            Self::ResponseFailed(event) => event.event_type(),
            Self::ResponseIncomplete(event) => event.event_type(),
            Self::ResponseOutputItemAdded(event) => event.event_type(),
            Self::ResponseOutputItemDone(event) => event.event_type(),
            Self::ResponseContentPartAdded(event) => event.event_type(),
            Self::ResponseContentPartDone(event) => event.event_type(),
            Self::ResponseOutputTextDelta(event) => event.event_type(),
            Self::ResponseOutputTextDone(event) => event.event_type(),
            Self::ResponseRefusalDelta(event) => event.event_type(),
            Self::ResponseRefusalDone(event) => event.event_type(),
            Self::ResponseFunctionCallArgumentsDelta(event) => event.event_type(),
            Self::ResponseFunctionCallArgumentsDone(event) => event.event_type(),
            Self::ResponseFileSearchCallInProgress(event) => event.event_type(),
            Self::ResponseFileSearchCallSearching(event) => event.event_type(),
            Self::ResponseFileSearchCallCompleted(event) => event.event_type(),
            Self::ResponseWebSearchCallInProgress(event) => event.event_type(),
            Self::ResponseWebSearchCallSearching(event) => event.event_type(),
            Self::ResponseWebSearchCallCompleted(event) => event.event_type(),
            Self::ResponseReasoningSummaryPartAdded(event) => event.event_type(),
            Self::ResponseReasoningSummaryPartDone(event) => event.event_type(),
            Self::ResponseReasoningSummaryTextDelta(event) => event.event_type(),
            Self::ResponseReasoningSummaryTextDone(event) => event.event_type(),
            Self::ResponseReasoningTextDelta(event) => event.event_type(),
            Self::ResponseReasoningTextDone(event) => event.event_type(),
            Self::ResponseImageGenerationCallCompleted(event) => event.event_type(),
            Self::ResponseImageGenerationCallGenerating(event) => event.event_type(),
            Self::ResponseImageGenerationCallInProgress(event) => event.event_type(),
            Self::ResponseImageGenerationCallPartialImage(event) => event.event_type(),
            Self::ResponseMCPCallArgumentsDelta(event) => event.event_type(),
            Self::ResponseMCPCallArgumentsDone(event) => event.event_type(),
            Self::ResponseMCPCallCompleted(event) => event.event_type(),
            Self::ResponseMCPCallFailed(event) => event.event_type(),
            Self::ResponseMCPCallInProgress(event) => event.event_type(),
            Self::ResponseMCPListToolsCompleted(event) => event.event_type(),
            Self::ResponseMCPListToolsFailed(event) => event.event_type(),
            Self::ResponseMCPListToolsInProgress(event) => event.event_type(),
            Self::ResponseCodeInterpreterCallInProgress(event) => event.event_type(),
            Self::ResponseCodeInterpreterCallInterpreting(event) => event.event_type(),
            Self::ResponseCodeInterpreterCallCompleted(event) => event.event_type(),
            Self::ResponseCodeInterpreterCallCodeDelta(event) => event.event_type(),
            Self::ResponseCodeInterpreterCallCodeDone(event) => event.event_type(),
            Self::ResponseOutputTextAnnotationAdded(event) => event.event_type(),
            Self::ResponseQueued(event) => event.event_type(),
            Self::ResponseCustomToolCallInputDelta(event) => event.event_type(),
            Self::ResponseCustomToolCallInputDone(event) => event.event_type(),
            Self::ResponseError(event) => event.event_type(),
        }
    }
}
