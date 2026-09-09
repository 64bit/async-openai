use super::stream::{
    ResponseAudioDeltaEvent, ResponseAudioDoneEvent, ResponseAudioTranscriptDeltaEvent,
    ResponseAudioTranscriptDoneEvent, ResponseCodeInterpreterCallCodeDeltaEvent,
    ResponseCodeInterpreterCallCodeDoneEvent, ResponseCodeInterpreterCallCompletedEvent,
    ResponseCodeInterpreterCallInProgressEvent, ResponseCodeInterpreterCallInterpretingEvent,
    ResponseCompletedEvent, ResponseContentPartAddedEvent, ResponseContentPartDoneEvent,
    ResponseCreatedEvent, ResponseCustomToolCallInputDeltaEvent,
    ResponseCustomToolCallInputDoneEvent, ResponseFailedEvent,
    ResponseFileSearchCallCompletedEvent, ResponseFileSearchCallInProgressEvent,
    ResponseFileSearchCallSearchingEvent, ResponseFunctionCallArgumentsDeltaEvent,
    ResponseFunctionCallArgumentsDoneEvent, ResponseImageGenCallCompletedEvent,
    ResponseImageGenCallGeneratingEvent, ResponseImageGenCallInProgressEvent,
    ResponseImageGenCallPartialImageEvent, ResponseInProgressEvent, ResponseIncompleteEvent,
    ResponseMCPCallArgumentsDeltaEvent, ResponseMCPCallArgumentsDoneEvent,
    ResponseMCPCallCompletedEvent, ResponseMCPCallFailedEvent, ResponseMCPCallInProgressEvent,
    ResponseMCPListToolsCompletedEvent, ResponseMCPListToolsFailedEvent,
    ResponseMCPListToolsInProgressEvent, ResponseOutputItemAddedEvent, ResponseOutputItemDoneEvent,
    ResponseOutputTextAnnotationAddedEvent, ResponseQueuedEvent,
    ResponseReasoningSummaryPartAddedEvent, ResponseReasoningSummaryPartDoneEvent,
    ResponseReasoningSummaryTextDeltaEvent, ResponseReasoningSummaryTextDoneEvent,
    ResponseReasoningTextDeltaEvent, ResponseReasoningTextDoneEvent, ResponseRefusalDeltaEvent,
    ResponseRefusalDoneEvent, ResponseShellCallCommandAddedStreamingEvent,
    ResponseShellCallCommandDeltaStreamingEvent, ResponseShellCallCommandDoneStreamingEvent,
    ResponseShellCallOutputContentDeltaStreamingEvent,
    ResponseShellCallOutputContentDoneStreamingEvent, ResponseTextDeltaEvent,
    ResponseTextDoneEvent, ResponseWebSearchCallCompletedEvent,
    ResponseWebSearchCallInProgressEvent, ResponseWebSearchCallSearchingEvent,
};
use super::{CreateResponse, FunctionCallOutputItemParam, MisalignmentErrorDetailsResource};
use serde::{Deserialize, Serialize};

/// A Responses WebSocket client event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ResponsesClientEvent {
    #[serde(rename = "response.create")]
    Create(ResponsesClientEventResponseCreate),
    #[serde(rename = "response.steer")]
    Steer(ResponseSteerEvent),
}

/// Client event for creating a response over a persistent WebSocket connection.
///
/// This payload uses the same top-level fields as `POST /v1/responses`, plus WebSocket-only envelope metadata.
/// Notes:
/// - `stream` is implicit over WebSocket and should not be sent.
/// - `background` is not supported over WebSocket.
/// - `stream_id` is WebSocket-only and is not part of `POST /v1/responses`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesClientEventResponseCreate {
    /// The WebSocket lane for this response. Requests with the same
    /// `stream_id` are processed FIFO, and events for the response echo the
    /// same `stream_id`.
    ///
    /// `stream_id` controls routing; `previous_response_id` controls conversation lineage,
    ///  so a new lane can fork from a response created on another lane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(flatten)]
    pub response: CreateResponse,
}

/// Server events received on a Responses WebSocket connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ResponsesServerEvent {
    /// Emitted when there is a partial audio response.
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta(ResponseAudioWsDelta),
    /// Emitted when the audio response is complete.
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone(ResponseAudioWsDone),
    /// Emitted when there is a partial transcript of audio.
    #[serde(rename = "response.audio.transcript.delta")]
    ResponseAudioTranscriptDelta(ResponseAudioTranscriptWsDelta),
    /// Emitted when the full audio transcript is completed.
    #[serde(rename = "response.audio.transcript.done")]
    ResponseAudioTranscriptDone(ResponseAudioTranscriptWsDone),
    /// Emitted when a partial code snippet is streamed by the code interpreter.
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeWsDelta),
    /// Emitted when the code snippet is finalized by the code interpreter.
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeWsDone),
    /// Emitted when the code interpreter call is completed.
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallWsCompleted),
    /// Emitted when a code interpreter call is in progress.
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress(ResponseCodeInterpreterCallInWsProgress),
    /// Emitted when the code interpreter is actively interpreting the code snippet.
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallWsInterpreting),
    /// Emitted when the model response is complete.
    #[serde(rename = "response.completed")]
    ResponseCompleted(ResponseWsCompleted),
    /// Emitted when a new content part is added.
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(ResponseContentPartWsAdded),
    /// Emitted when a content part is done.
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(ResponseContentPartWsDone),
    /// An event that is emitted when a response is created.
    #[serde(rename = "response.created")]
    ResponseCreated(ResponseWsCreated),
    /// Emitted when a file search call is completed (results found).
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted(ResponseFileSearchCallWsCompleted),
    /// Emitted when a file search call is initiated.
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInWsProgress),
    /// Emitted when a file search is currently searching.
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching(ResponseFileSearchCallWsSearching),
    /// Emitted when there is a partial function-call arguments delta.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsWsDelta),
    /// Emitted when function-call arguments are finalized.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsWsDone),
    /// A streaming event that indicated a shell command was added to a tool call.
    #[serde(rename = "response.shell_call_command.added")]
    ResponseShellCallCommandAdded(ResponseShellCallCommandWsAdded),
    /// A streaming event that indicated a shell command was incrementally updated.
    #[serde(rename = "response.shell_call_command.delta")]
    ResponseShellCallCommandDelta(ResponseShellCallCommandWsDelta),
    /// A streaming event that indicated a shell command was completed.
    #[serde(rename = "response.shell_call_command.done")]
    ResponseShellCallCommandDone(ResponseShellCallCommandWsDone),
    /// A streaming event that indicated shell call output was incrementally added.
    #[serde(rename = "response.shell_call_output_content.delta")]
    ResponseShellCallOutputContentDelta(ResponseShellCallOutputContentWsDelta),
    /// A streaming event that indicated shell call output was completed.
    #[serde(rename = "response.shell_call_output_content.done")]
    ResponseShellCallOutputContentDone(ResponseShellCallOutputContentWsDone),
    /// Emitted when the response is in progress.
    #[serde(rename = "response.in_progress")]
    ResponseInProgress(ResponseInWsProgress),
    /// An event that is emitted when a response fails.
    #[serde(rename = "response.failed")]
    ResponseFailed(ResponseWsFailed),
    /// An event that is emitted when a response finishes as incomplete.
    ///
    /// Over WebSocket, steering can finish a response with `response.incomplete_details.reason` set to
    /// `steered`, followed automatically by a successor `response.created` that commits the queued
    /// steering input.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete(ResponseWsIncomplete),
    /// Emitted when a new output item is added.
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(ResponseOutputItemWsAdded),
    /// Emitted when an output item is marked done.
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(ResponseOutputItemWsDone),
    /// Emitted when a new reasoning summary part is added.
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded(ResponseReasoningSummaryPartWsAdded),
    /// Emitted when a reasoning summary part is completed.
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone(ResponseReasoningSummaryPartWsDone),
    /// Emitted when a delta is added to a reasoning summary text.
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta(ResponseReasoningSummaryTextWsDelta),
    /// Emitted when a reasoning summary text is completed.
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone(ResponseReasoningSummaryTextWsDone),
    /// Emitted when a delta is added to a reasoning text.
    #[serde(rename = "response.reasoning_text.delta")]
    ResponseReasoningTextDelta(ResponseReasoningTextWsDelta),
    /// Emitted when a reasoning text is completed.
    #[serde(rename = "response.reasoning_text.done")]
    ResponseReasoningTextDone(ResponseReasoningTextWsDone),
    /// Emitted when there is a partial refusal text.
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta(ResponseRefusalWsDelta),
    /// Emitted when refusal text is finalized.
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone(ResponseRefusalWsDone),
    /// Emitted when there is an additional text delta.
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta(ResponseTextWsDelta),
    /// Emitted when text content is finalized.
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone(ResponseTextWsDone),
    /// Emitted when a web search call is completed.
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted(ResponseWebSearchCallWsCompleted),
    /// Emitted when a web search call is initiated.
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress(ResponseWebSearchCallInWsProgress),
    /// Emitted when a web search call is executing.
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching(ResponseWebSearchCallWsSearching),
    /// Emitted when an image generation tool call has completed and the final image is available.
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenerationCallCompleted(ResponseImageGenCallWsCompleted),
    /// Emitted when an image generation tool call is actively generating an image (intermediate
    /// state).
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenerationCallGenerating(ResponseImageGenCallWsGenerating),
    /// Emitted when an image generation tool call is in progress.
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenerationCallInProgress(ResponseImageGenCallInWsProgress),
    /// Emitted when a partial image is available during image generation streaming.
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenerationCallPartialImage(ResponseImageGenCallPartialWsImage),
    /// Emitted when there is a delta (partial update) to the arguments of an MCP tool call.
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMCPCallArgumentsDelta(ResponseMcpCallArgumentsWsDelta),
    /// Emitted when the arguments for an MCP tool call are finalized.
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMCPCallArgumentsDone(ResponseMcpCallArgumentsWsDone),
    /// Emitted when an MCP tool call has completed successfully.
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMCPCallCompleted(ResponseMcpCallWsCompleted),
    /// Emitted when an MCP tool call has failed.
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMCPCallFailed(ResponseMcpCallWsFailed),
    /// Emitted when an MCP tool call is in progress.
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMCPCallInProgress(ResponseMcpCallInWsProgress),
    /// Emitted when the list of available MCP tools has been successfully retrieved.
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMCPListToolsCompleted(ResponseMcpListToolsWsCompleted),
    /// Emitted when the attempt to list available MCP tools has failed.
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMCPListToolsFailed(ResponseMcpListToolsWsFailed),
    /// Emitted when the system is in the process of retrieving the list of available MCP tools.
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMCPListToolsInProgress(ResponseMcpListToolsInWsProgress),
    /// Emitted when an annotation is added to output text content.
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded(ResponseOutputTextAnnotationWsAdded),
    /// Emitted when a response is queued and waiting to be processed.
    #[serde(rename = "response.queued")]
    ResponseQueued(ResponseWsQueued),
    /// Event representing a delta (partial update) to the input of a custom tool call.
    #[serde(rename = "response.custom_tool_call_input.delta")]
    ResponseCustomToolCallInputDelta(ResponseCustomToolCallInputWsDelta),
    /// Event indicating that input for a custom tool call is complete.
    #[serde(rename = "response.custom_tool_call_input.done")]
    ResponseCustomToolCallInputDone(ResponseCustomToolCallInputWsDone),
    #[serde(rename = "error")]
    Error(ResponseWsError),
    #[serde(rename = "response.steer.accepted")]
    SteerAccepted(ResponseSteerAcceptedEvent),
    #[serde(rename = "response.steer.pending")]
    SteerPending(ResponseSteerPendingEvent),
    #[serde(rename = "response.steer.failed")]
    SteerFailed(ResponseSteerFailedEvent),
}

/// Emitted when there is a partial audio response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioWsDelta {
    #[serde(flatten)]
    pub event: ResponseAudioDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the audio response is complete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioWsDone {
    #[serde(flatten)]
    pub event: ResponseAudioDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when there is a partial transcript of audio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioTranscriptWsDelta {
    #[serde(flatten)]
    pub event: ResponseAudioTranscriptDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the full audio transcript is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioTranscriptWsDone {
    #[serde(flatten)]
    pub event: ResponseAudioTranscriptDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a partial code snippet is streamed by the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeWsDelta {
    #[serde(flatten)]
    pub event: ResponseCodeInterpreterCallCodeDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the code snippet is finalized by the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeWsDone {
    #[serde(flatten)]
    pub event: ResponseCodeInterpreterCallCodeDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the code interpreter call is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallWsCompleted {
    #[serde(flatten)]
    pub event: ResponseCodeInterpreterCallCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a code interpreter call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallInWsProgress {
    #[serde(flatten)]
    pub event: ResponseCodeInterpreterCallInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the code interpreter is actively interpreting the code snippet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallWsInterpreting {
    #[serde(flatten)]
    pub event: ResponseCodeInterpreterCallInterpretingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the model response is complete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsCompleted {
    #[serde(flatten)]
    pub event: ResponseCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a new content part is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseContentPartWsAdded {
    #[serde(flatten)]
    pub event: ResponseContentPartAddedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a content part is done.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseContentPartWsDone {
    #[serde(flatten)]
    pub event: ResponseContentPartDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// An event that is emitted when a response is created.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsCreated {
    #[serde(flatten)]
    pub event: ResponseCreatedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a file search call is completed (results found).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallWsCompleted {
    #[serde(flatten)]
    pub event: ResponseFileSearchCallCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a file search call is initiated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallInWsProgress {
    #[serde(flatten)]
    pub event: ResponseFileSearchCallInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a file search is currently searching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallWsSearching {
    #[serde(flatten)]
    pub event: ResponseFileSearchCallSearchingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when there is a partial function-call arguments delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionCallArgumentsWsDelta {
    #[serde(flatten)]
    pub event: ResponseFunctionCallArgumentsDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when function-call arguments are finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionCallArgumentsWsDone {
    #[serde(flatten)]
    pub event: ResponseFunctionCallArgumentsDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A streaming event that indicated a shell command was added to a tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandWsAdded {
    #[serde(flatten)]
    pub event: ResponseShellCallCommandAddedStreamingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A streaming event that indicated a shell command was incrementally updated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandWsDelta {
    #[serde(flatten)]
    pub event: ResponseShellCallCommandDeltaStreamingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A streaming event that indicated a shell command was completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallCommandWsDone {
    #[serde(flatten)]
    pub event: ResponseShellCallCommandDoneStreamingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A streaming event that indicated shell call output was incrementally added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallOutputContentWsDelta {
    #[serde(flatten)]
    pub event: ResponseShellCallOutputContentDeltaStreamingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A streaming event that indicated shell call output was completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseShellCallOutputContentWsDone {
    #[serde(flatten)]
    pub event: ResponseShellCallOutputContentDoneStreamingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the response is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInWsProgress {
    #[serde(flatten)]
    pub event: ResponseInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// An event that is emitted when a response fails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsFailed {
    #[serde(flatten)]
    pub event: ResponseFailedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// An event that is emitted when a response finishes as incomplete.
///
/// Over WebSocket, steering can finish a response with `response.incomplete_details.reason` set to
/// `steered`, followed automatically by a successor `response.created` that commits the queued
/// steering input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsIncomplete {
    #[serde(flatten)]
    pub event: ResponseIncompleteEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a new output item is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputItemWsAdded {
    #[serde(flatten)]
    pub event: ResponseOutputItemAddedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an output item is marked done.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputItemWsDone {
    #[serde(flatten)]
    pub event: ResponseOutputItemDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a new reasoning summary part is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartWsAdded {
    #[serde(flatten)]
    pub event: ResponseReasoningSummaryPartAddedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a reasoning summary part is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartWsDone {
    #[serde(flatten)]
    pub event: ResponseReasoningSummaryPartDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a delta is added to a reasoning summary text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryTextWsDelta {
    #[serde(flatten)]
    pub event: ResponseReasoningSummaryTextDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a reasoning summary text is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryTextWsDone {
    #[serde(flatten)]
    pub event: ResponseReasoningSummaryTextDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a delta is added to a reasoning text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningTextWsDelta {
    #[serde(flatten)]
    pub event: ResponseReasoningTextDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a reasoning text is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningTextWsDone {
    #[serde(flatten)]
    pub event: ResponseReasoningTextDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when there is a partial refusal text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseRefusalWsDelta {
    #[serde(flatten)]
    pub event: ResponseRefusalDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when refusal text is finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseRefusalWsDone {
    #[serde(flatten)]
    pub event: ResponseRefusalDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when there is an additional text delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseTextWsDelta {
    #[serde(flatten)]
    pub event: ResponseTextDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when text content is finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseTextWsDone {
    #[serde(flatten)]
    pub event: ResponseTextDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a web search call is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallWsCompleted {
    #[serde(flatten)]
    pub event: ResponseWebSearchCallCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a web search call is initiated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallInWsProgress {
    #[serde(flatten)]
    pub event: ResponseWebSearchCallInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a web search call is executing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallWsSearching {
    #[serde(flatten)]
    pub event: ResponseWebSearchCallSearchingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an image generation tool call has completed and the final image is available.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallWsCompleted {
    #[serde(flatten)]
    pub event: ResponseImageGenCallCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an image generation tool call is actively generating an image (intermediate
/// state).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallWsGenerating {
    #[serde(flatten)]
    pub event: ResponseImageGenCallGeneratingEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an image generation tool call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallInWsProgress {
    #[serde(flatten)]
    pub event: ResponseImageGenCallInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a partial image is available during image generation streaming.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallPartialWsImage {
    #[serde(flatten)]
    pub event: ResponseImageGenCallPartialImageEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when there is a delta (partial update) to the arguments of an MCP tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallArgumentsWsDelta {
    #[serde(flatten)]
    pub event: ResponseMCPCallArgumentsDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the arguments for an MCP tool call are finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallArgumentsWsDone {
    #[serde(flatten)]
    pub event: ResponseMCPCallArgumentsDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an MCP tool call has completed successfully.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallWsCompleted {
    #[serde(flatten)]
    pub event: ResponseMCPCallCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an MCP tool call has failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallWsFailed {
    #[serde(flatten)]
    pub event: ResponseMCPCallFailedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an MCP tool call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallInWsProgress {
    #[serde(flatten)]
    pub event: ResponseMCPCallInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the list of available MCP tools has been successfully retrieved.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsWsCompleted {
    #[serde(flatten)]
    pub event: ResponseMCPListToolsCompletedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the attempt to list available MCP tools has failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsWsFailed {
    #[serde(flatten)]
    pub event: ResponseMCPListToolsFailedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when the system is in the process of retrieving the list of available MCP tools.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsInWsProgress {
    #[serde(flatten)]
    pub event: ResponseMCPListToolsInProgressEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when an annotation is added to output text content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextAnnotationWsAdded {
    #[serde(flatten)]
    pub event: ResponseOutputTextAnnotationAddedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Emitted when a response is queued and waiting to be processed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsQueued {
    #[serde(flatten)]
    pub event: ResponseQueuedEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Event representing a delta (partial update) to the input of a custom tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCustomToolCallInputWsDelta {
    #[serde(flatten)]
    pub event: ResponseCustomToolCallInputDeltaEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Event indicating that input for a custom tool call is complete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCustomToolCallInputWsDone {
    #[serde(flatten)]
    pub event: ResponseCustomToolCallInputDoneEvent,
    /// The WebSocket lane that emitted this event. This field is present when the originating
    /// `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// An error payload that was emitted for a streaming error event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorPayload {
    /// The error type that was emitted.
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The human-readable error message that was emitted.
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// The response headers that were emitted with the error, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misalignment: Option<MisalignmentErrorDetailsResource>,
}

/// Emitted when steering input has been validated and queued. Acceptance means
/// the server owns the input, not that it has been applied. The successor's
/// `response.created` event is the commit point. If accepted input cannot be
/// committed, `response.steer.failed` returns it with the same steering ID.
///
/// When the response stops for client-owned tool output or approval, the input
/// remains queued and `response.steer.pending` is emitted after
/// `response.completed`. Fill the pending event's `required_input` stubs with
/// saved results and send one matching explicit `response.create` per parent.
/// Do not resend accepted input while it is still queued.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerAcceptedEvent {
    /// The sequence number for this event.
    pub sequence_number: u64,
    /// The accepted steering submission.
    pub steer: ResponseSteerAcceptedEventSteer,
    /// The WebSocket lane that emitted this event. This field is present when
    /// the target response's `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// The accepted steering submission.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerAcceptedEventSteer {
    /// The ID assigned to the steering submission.
    pub id: String,
    /// The ID of the response being steered.
    pub previous_response_id: String,
}

/// A machine-readable steering error code. Clients should handle unknown
/// values because additional codes may be introduced. Known values include:
/// - `response_not_found`: The target response is not available on this connection.
/// - `invalid_input`: The event or input failed validation.
/// - `steering_not_supported`: The model or response execution mode does not support steering.
/// - `too_many_pending_steers`: Too much steering input is pending for the response.
/// - `response_already_completed`: The response completed and is no longer accepting steering input.
/// - `response_not_active`: The response is no longer accepting steering input.
/// - `successor_creation_failed`: The successor response could not be created.
pub type ResponseSteerErrorCode = String;

/// Queues user input to steer a response on this WebSocket connection. Input
/// can contain text, images, and files. Steering is supported only for
/// single-agent responses on models and execution modes that support steering.
/// Responses bound to a conversation or using automatic compaction do not
/// support steering.
///
/// A `response.steer.accepted` event acknowledges that the server owns the
/// queued input, not that it has been applied. The successor's `response.created`
/// event is the commit point. Input that cannot be committed is returned in
/// `response.steer.failed`.
///
/// Steering may cause the active response to finish at a safe output boundary
/// with `response.incomplete` and `incomplete_details.reason` set to `steered`,
/// followed automatically by a successor `response.created`. Normal completion
/// can also be followed by an automatic successor. Automatic successors inherit
/// the previous response's settings and continue from it with the queued input.
///
/// If the response stops for client-owned tool output or approval, accepted
/// steering input remains queued and `response.steer.pending` is emitted after
/// `response.completed`. Fill the `required_input` stubs from that event with
/// saved tool results or approval decisions, and send one explicit
/// `response.create` per parent with the same `previous_response_id` and
/// WebSocket lane. Do not rerun tools or resend accepted steering input. The
/// queued input is prepended in submission order to that request's input, and
/// the explicit request retains its own settings.
///
/// This event accepts only `type`, `previous_response_id`, and `input`. Do not
/// send `stream_id`; the target response determines the WebSocket lane.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerEvent {
    /// The ID of the response to steer on this WebSocket connection.
    pub previous_response_id: String,
    pub input: ResponseSteerInput,
}

/// Emitted when steering input is rejected or cannot be committed to a
/// successor response. Returns the original, uncommitted input so the client
/// can carry it into `response.create` when appropriate. Invalid input must
/// be corrected before retrying.
///
/// Failures after acceptance include the same steering ID. Failures before an
/// ID is allocated omit `steer.id`. A lost connection or missing acknowledgement
/// leaves the outcome unknown; it is not proof that the input was rejected.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerFailedEvent {
    /// The sequence number for this event.
    pub sequence_number: i64,
    /// The steering submission that could not be committed.
    pub steer: ResponseSteerFailedEventSteer,
    /// Information about why the input could not be committed.
    pub error: ResponseSteerFailedEventError,
    /// The WebSocket lane that emitted this event, when the target response is
    /// available and its `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// Information about why the input could not be committed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerFailedEventError {
    /// The error type. Always `invalid_request_error`.
    pub r#type: ResponseSteerFailedEventErrorType,
    pub code: ResponseSteerErrorCode,
    /// A human-readable description of the error.
    pub message: String,
}

/// The error type. Always `invalid_request_error`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseSteerFailedEventErrorType {
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
}

/// The steering submission that could not be committed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerFailedEventSteer {
    /// The ID assigned to the steering submission, if one was allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the response that was targeted for steering.
    pub previous_response_id: String,
    pub input: ResponseSteerInput,
}

/// Input to queue for a continuation of the response. Uses the same string or
/// input-item shape as `response.create.input`, with a non-empty array when
/// supplying input items.
///
/// Steering accepts only messages with the `user` role. Each message may
/// contain only `type`, `role`, and `content`, with `content` as a string or an
/// array of `input_text`, `input_image`, and `input_file` parts. The optional
/// `type` must be `message`. Other roles, tool outputs, and item types are not
/// supported for steering.
/// Input to queue for a continuation of the response. Uses the same string or
/// input-item shape as `response.create.input`, with a non-empty array when
/// supplying input items.
///
/// Steering accepts only messages with the `user` role. Each message may
/// contain only `type`, `role`, and `content`, with `content` as a string or an
/// array of `input_text`, `input_image`, and `input_file` parts. The optional
/// `type` must be `message`. Other roles, tool outputs, and item types are not
/// supported for steering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseSteerInput {
    Text(String),
    List(Vec<ResponseSteerInputItem>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseSteerInputItem {
    Message(UserMessageItemParam),
    FunctionCallOutput(FunctionCallOutputItemParam),
}

/// Emitted when accepted steering input remains queued after the target
/// response completes. The server still owns the input. Do not resend it.
/// The successor's `response.created` event is the commit point.
///
/// When `reason` is `waiting_for_required_input`, this event follows
/// `response.completed` while the response waits for the tool results or
/// approval decisions identified by `required_input`. Copy those stubs, fill
/// their result fields using the ordinary `response.create` input schemas,
/// and submit one continuation per parent with the same `previous_response_id`
/// and WebSocket lane. Use saved results without rerunning tools. The queued
/// steering input is prepended in submission order to the continuation's
/// input. That explicit request retains its own settings.
///
/// This notification is emitted at most once per steering submission. Multiple
/// submissions for the same parent can report the same required inputs; they
/// do not each require a separate continuation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerPendingEvent {
    /// The sequence number for this event.
    pub sequence_number: u64,
    /// The steering submission that remains queued.
    pub steer: ResponseSteerPendingEventSteer,
    pub reason: ResponseSteerPendingReason,
    /// Input stubs identifying outstanding client-owned tool results or
    /// approval decisions. Each stub contains identifying fields only; the
    /// client supplies the result before including it in `response.create`.
    pub required_input: Vec<ResponseSteerRequiredInput>,
    /// The WebSocket lane that emitted this event. This field is present when
    /// the target response's `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// The steering submission that remains queued.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerPendingEventSteer {
    /// The ID assigned to the steering submission.
    pub id: String,
    /// The ID of the response being steered.
    pub previous_response_id: String,
}

/// An extensible enum describing why accepted steering input is still queued.
/// Clients should handle unknown values because additional reasons may be
/// introduced. Known values include:
/// - `waiting_for_required_input`: The response is waiting for the tool results or approval decisions
///   identified by `required_input`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSteerPendingReason {
    WaitingForRequiredInput,
    #[serde(untagged)]
    Other(String),
}

/// An input stub identifying an outstanding client-owned tool result or
/// approval decision. Copy the stub and fill the result fields using the
/// corresponding `response.create` input schema. Use saved results without
/// rerunning the tool. The server does not supply results, approval decisions,
/// or safety acknowledgements in these stubs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseSteerRequiredInput {
    /// Supply `output` using the function tool call output input schema.
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(ResponseSteerRequiredInputFunctionCallOutput),
    /// Supply `output` using the custom tool call output input schema. The
    /// original custom tool call supplies the tool's name.
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput(ResponseSteerRequiredInputCustomToolCallOutput),
    /// Supply `output` using the computer tool call output input schema,
    /// including any required `acknowledged_safety_checks`.
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput(ResponseSteerRequiredInputComputerCallOutput),
    /// Supply `output` using the shell tool call output input schema. Each
    /// output entry includes `stdout`, `stderr`, and `outcome`.
    #[serde(rename = "shell_call_output")]
    ShellCallOutput(ResponseSteerRequiredInputShellCallOutput),
    /// Supply `status` and optional `output` using the apply patch tool call
    /// output input schema.
    #[serde(rename = "apply_patch_call_output")]
    ApplyPatchCallOutput(ResponseSteerRequiredInputApplyPatchCallOutput),
    /// Supply `tools` using the tool search output input schema, retaining
    /// `execution: "client"`.
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput(ResponseSteerRequiredInputToolSearchOutput),
    /// Supply `approve` using the MCP approval response input schema. An
    /// optional `reason` can be supplied when denying the request. The original
    /// approval request identifies the tool and server.
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse(ResponseSteerRequiredInputMcpApprovalResponse),
}

/// Supply `output` using the function tool call output input schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputFunctionCallOutput {
    pub call_id: String,
    pub name: String,
}

/// Supply `output` using the custom tool call output input schema. The
/// original custom tool call supplies the tool's name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputCustomToolCallOutput {
    pub call_id: String,
}

/// Supply `output` using the computer tool call output input schema,
/// including any required `acknowledged_safety_checks`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputComputerCallOutput {
    pub call_id: String,
}

/// Supply `output` using the shell tool call output input schema. Each
/// output entry includes `stdout`, `stderr`, and `outcome`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputShellCallOutput {
    pub call_id: String,
}

/// Supply `status` and optional `output` using the apply patch tool call
/// output input schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputApplyPatchCallOutput {
    pub call_id: String,
}

/// Supply `tools` using the tool search output input schema, retaining
/// `execution: "client"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputToolSearchOutput {
    pub call_id: String,
    pub execution: ResponseSteerRequiredInputToolSearchOutputExecution,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseSteerRequiredInputToolSearchOutputExecution {
    #[serde(rename = "client")]
    Client,
}

/// Supply `approve` using the MCP approval response input schema. An
/// optional `reason` can be supplied when denying the request. The original
/// approval request identifies the tool and server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseSteerRequiredInputMcpApprovalResponse {
    pub approval_request_id: String,
}

/// Emitted when an error occurs while processing a Responses WebSocket request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWsError {
    /// The HTTP status code associated with a WebSocket protocol error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u32>,
    /// The sequence number of an error emitted by the response stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
    /// Details about the error.
    pub error: ErrorPayload,
    /// The WebSocket lane that emitted this event. This field is present when the
    /// originating `response.create` event supplied a `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesWebSocketStreamEvent {
    /// The WebSocket lane that emitted this event. This field is present
    /// when the originating `response.create` event supplied a
    /// `stream_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserMessageItemParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The message role. Always `user`.
    pub role: UserMessageItemParamRole,
    /// The message content, as an array of content parts.
    pub content: crate::types::responses::EasyInputContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// The message role. Always `user`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserMessageItemParamRole {
    User,
}
