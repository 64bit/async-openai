use serde::{Deserialize, Serialize};

use crate::types::realtime::{LogProbProperties, TranscriptionUsage};

use super::{
    conversation_item::RealtimeConversationItem, error::RealtimeAPIError,
    response::RealtimeResponse, session::Session,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventError {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Details of the error.
    pub error: RealtimeAPIError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The session resource.
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The updated session resource.
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
    /// The ID of the item that precedes this one, if any. This is used to maintain ordering when items are inserted.
    pub previous_item_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
    /// The ID of the item that precedes this one, if any. This is used to maintain ordering when items are inserted.
    pub previous_item_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the preceding item after which the new item will be inserted. Can be null if the item has no predecessor.
    pub previous_item_id: Option<String>,
    /// The ID of the user message item that will be created.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventInputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Milliseconds from the start of all audio written to the buffer during the session when speech was
    /// first detected. This will correspond to the beginning of audio sent to the model, and thus includes
    /// the `prefix_padding_ms` configured in the Session.
    pub audio_start_ms: u32,
    /// The ID of the user message item that will be created when speech stops.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Milliseconds since the session started when speech stopped. This will correspond to the end of
    /// audio sent to the model, and thus includes the `min_silence_duration_ms` configured in the Session.
    pub audio_end_ms: u32,
    /// The ID of the user message item that will be created.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventInputAudioBufferTimeoutTriggered {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Millisecond offset of audio written to the input audio buffer at the time the timeout was triggered.
    pub audio_end_ms: u32,
    /// Millisecond offset of audio written to the input audio buffer that was after the playback time of the last model response.
    pub audio_start_ms: u32,
    /// The ID of the item associated with this segment.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the audio that is being transcribed.
    pub item_id: String,
    /// The index of the content part containing the audio.
    pub content_index: u32,
    /// The transcribed text.
    pub transcript: String,
    /// Optional per-token log probability data.
    pub logprobs: Option<Vec<LogProbProperties>>,
    /// Usage statistics for the transcription, this is billed according to the ASR model's pricing rather than
    /// the realtime model's pricing.
    pub usage: TranscriptionUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the audio that is being transcribed.
    pub item_id: String,
    ///The index of the content part in the item's content array.
    pub content_index: u32,
    /// The text delta.
    pub delta: String,
    /// The log probabilities of the transcription. These can be enabled by configurating the session with
    /// `"include": ["item.input_audio_transcription.logprobs"]`. Each entry in the array
    /// corresponds a log probability of which token would be selected for this chunk of transcription. This
    /// can help to identify if it was possible there were multiple valid options for a given chunk of
    /// transcription.
    pub logprobs: Option<Vec<LogProbProperties>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item.
    pub item_id: String,
    /// The index of the content part containing the audio.
    pub content_index: u32,
    /// Details of the transcription error.
    pub error: RealtimeAPIError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the assistant message item that was truncated.
    pub item_id: String,
    /// The index of the content part that was truncated.
    pub content_index: u32,
    /// The duration up to which the audio was truncated, in milliseconds.
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item that was deleted.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemRetrieved {
    /// The unique ID of the server event.
    pub event_id: String,
    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionSegment {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the input audio content.
    pub item_id: String,
    /// The index of the input audio content part within the item.
    pub content_index: u32,
    /// The text for this segment.
    pub text: String,
    /// The segment identifier.
    pub id: String,
    /// The detected speaker label for this segment.
    pub speaker: String,
    /// Start time of the segment in seconds.
    pub start: f32,
    /// End time of the segment in seconds.
    pub end: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The response resource.
    pub response: RealtimeResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The response resource.
    pub response: RealtimeResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the Response to which the item belongs.
    pub response_id: String,
    /// The index of the output item in the Response.
    pub output_index: u32,
    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseOutputItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response to which the item belongs.
    pub response_id: String,
    /// The index of the output item in the Response.
    pub output_index: u32,
    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data
        audio: Option<String>,
        /// The transcript of the audio
        transcript: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item to which the content part was added.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The content part that was added.
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseContentPartDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The content part that is done.
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseTextDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The text delta.
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseTextDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The final text content.
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// The text delta.
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    ///The final transcript of the audio.
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseAudioDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
    /// Base64-encoded audio data delta.
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseAudioDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The index of the content part in the item's content array.
    pub content_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the function call.
    pub call_id: String,
    /// The arguments delta as a JSON string.
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the function call.
    pub call_id: String,
    /// The final arguments as a JSON string.
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeRateLimitName {
    Requests,
    Tokens,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeRateLimit {
    /// The name of the rate limit (requests, tokens).
    pub name: RealtimeRateLimitName,
    /// The maximum allowed value for the rate limit.
    pub limit: u32,
    /// The remaining value before the limit is reached.
    pub remaining: u32,
    /// Seconds until the rate limit resets.
    pub reset_seconds: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    pub rate_limits: Vec<RealtimeRateLimit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventMCPListToolsInProgress {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventMCPListToolsCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventMCPListToolsFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseMCPCallArgumentsDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The JSON-encoded arguments delta.
    pub delta: String,
    /// If present, indicates the delta text was obfuscated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseMCPCallArgumentsDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the response.
    pub response_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The final JSON-encoded arguments string.
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseMCPCallInProgress {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the MCP tool call item.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseMCPCallCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the MCP tool call item.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeServerEventResponseMCPCallFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The index of the output item in the response.
    pub output_index: u32,
    /// The ID of the MCP tool call item.
    pub item_id: String,
}

/// These are events emitted from the OpenAI Realtime WebSocket server to the client.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RealtimeServerEvent {
    /// Returned when an error occurs, which could be a client problem or a server problem.
    /// Most errors are recoverable and the session will stay open, we recommend to
    /// implementors to monitor and log error messages by default.
    #[serde(rename = "error")]
    Error(RealtimeServerEventError),

    /// Returned when a Session is created. Emitted automatically when a new connection is established as the first server event.
    /// This event will contain the default Session configuration.
    #[serde(rename = "session.created")]
    SessionCreated(RealtimeServerEventSessionCreated),

    /// Returned when a session is updated with a `session.update` event, unless there is an error.
    #[serde(rename = "session.updated")]
    SessionUpdated(RealtimeServerEventSessionUpdated),

    /// Sent by the server when an Item is added to the default Conversation. This can happen in several cases:
    /// - When the client sends a conversation.item.create event
    /// - When the input audio buffer is committed. In this case the item will be a user message containing the audio from the buffer.
    /// - When the model is generating a Response. In this case the `conversation.item.added` event will be sent when the model starts
    ///   generating a specific Item, and thus it will not yet have any content (and `status` will be `in_progress`).
    ///
    /// The event will include the full content of the Item (except when model is generating a Response) except for audio data,
    /// which can be retrieved separately with a `conversation.item.retrieve` event if necessary.
    #[serde(rename = "conversation.item.added")]
    ConversationItemAdded(RealtimeServerEventConversationItemAdded),

    /// Returned when a conversation item is finalized.
    ///
    /// The event will include the full content of the Item except for audio data, which can be retrieved
    /// separately with a `conversation.item.retrieve` event if needed.
    #[serde(rename = "conversation.item.done")]
    ConversationItemDone(RealtimeServerEventConversationItemDone),

    /// Returned when a conversation item is retrieved with `conversation.item.retrieve`.
    /// This is provided as a way to fetch the server's representation of an item, for example to get access
    /// to the post-processed audio data after noise cancellation and VAD.
    /// It includes the full content of the Item, including audio data.
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),

    /// This event is the output of audio transcription for user audio written to the user audio
    /// buffer. Transcription begins when the input audio buffer is committed by the client or
    /// server (when VAD is enabled). Transcription runs asynchronously with Response
    /// creation, so this event may come before or after the Response events.
    ///
    /// Realtime API models accept audio natively, and thus input transcription is a separate process
    /// run on a separate ASR (Automatic Speech Recognition) model. The transcript
    /// may diverge somewhat from the model's interpretation, and should be treated as a rough guide.
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted(
        RealtimeServerEventConversationItemInputAudioTranscriptionCompleted,
    ),

    /// Returned when the text value of an input audio transcription content part is updated with incremental transcription results.
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta(
        RealtimeServerEventConversationItemInputAudioTranscriptionDelta,
    ),

    /// Returned when an input audio transcription segment is identified for an item.
    #[serde(rename = "conversation.item.input_audio_transcription.segment")]
    ConversationItemInputAudioTranscriptionSegment(
        RealtimeServerEventConversationItemInputAudioTranscriptionSegment,
    ),

    /// Returned when input audio transcription is configured, and a transcription request for a user message failed.
    /// These events are separate from other `error` events so that the client can identify the related Item.
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed(
        RealtimeServerEventConversationItemInputAudioTranscriptionFailed,
    ),

    /// Returned when an earlier assistant audio message item is truncated by the client with a `conversation.item.truncate` event.
    /// This event is used to synchronize the server's understanding of the audio with the client's playback.
    ///
    /// This action will truncate the audio and remove the server-side text transcript to ensure there is no text in the
    /// context that hasn't been heard by the user.
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated(RealtimeServerEventConversationItemTruncated),

    /// Returned when an item in the conversation is deleted by the client with a `conversation.item.delete` event.
    /// This event is used to synchronize the server's understanding of the conversation history with the client's view.
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted(RealtimeServerEventConversationItemDeleted),

    /// Returned when an input audio buffer is committed, either by the client or automatically in server VAD mode.
    /// The `item_id` property is the ID of the user message item that will be created,
    /// thus a `conversation.item.created` event will also be sent to the client.
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),

    /// Returned when the input audio buffer is cleared by the client with a `input_audio_buffer.clear` event.
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),

    /// Sent by the server when in `server_vad` mode to indicate that speech has been detected in the audio buffer.
    /// This can happen any time audio is added to the buffer (unless speech is already detected).
    /// The client may want to use this event to interrupt audio playback or provide visual feedback to the user.
    ///
    /// The client should expect to receive a `input_audio_buffer.speech_stopped` event when speech stops.
    /// The `item_id` property is the ID of the user message item that will be created when speech stops and will
    /// also be included in the `input_audio_buffer.speech_stopped` event (unless the client manually commits the
    ///  audio buffer during VAD activation).
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),

    /// Returned in `server_vad` mode when the server detects the end of speech in the audio buffer.
    /// The server will also send a `conversation.item.created` event with the user message item that is created from the audio buffer.
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),

    /// Returned when the Server VAD timeout is triggered for the input audio buffer. This is
    /// configured with `idle_timeout_ms` in the `turn_detection` settings of the session, and
    /// it indicates that there hasn't been any speech detected for the configured duration.
    ///
    /// The `audio_start_ms` and `audio_end_ms` fields indicate the segment of audio after the
    /// last model response up to the triggering time, as an offset from the beginning of audio
    /// written to the input audio buffer. This means it demarcates the segment of audio that
    /// was silent and the difference between the start and end values will roughly match the configured timeout.
    ///
    /// The empty audio will be committed to the conversation as an `input_audio` item (there
    /// will be a `input_audio_buffer.committed` event) and a model response will be generated.
    /// There may be speech that didn't trigger VAD but is still detected by the model, so the model may respond
    /// with something relevant to the conversation or a prompt to continue speaking.
    #[serde(rename = "input_audio_buffer.timeout_triggered")]
    InputAudioBufferTimeoutTriggered(RealtimeServerEventInputAudioBufferTimeoutTriggered),

    /// *WebRTC Only*: Emitted when the server begins streaming audio to the client. This
    /// event is emitted after an audio content part has been added (`response.content_part.added`) to the response.
    /// [Learn more](https://platform.openai.com/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),

    /// *WebRTC Only*: Emitted when the output audio buffer has been completely drained on
    /// the server, and no more audio is forthcoming. This event is emitted after the full response data has been sent
    /// to the client (`response.done`). [Learn more](https://platform.openai.com/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),

    /// *WebRTC Only*: Emitted when the output audio buffer is cleared. This happens either in
    /// VAD mode when the user has interrupted (`input_audio_buffer.speech_started`), or when the client has
    /// emitted the `output_audio_buffer.clear` event to manually cut off the current audio response.
    /// [Learn more](https://platform.openai.com/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc).
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),

    /// Returned when a new Response is created. The first event of response creation,
    /// where the response is in an initial state of `in_progress`.
    #[serde(rename = "response.created")]
    ResponseCreated(RealtimeServerEventResponseCreated),

    /// Returned when a Response is done streaming. Always emitted, no matter the final state.
    /// The Response object included in the `response.done` event will include all output Items in the Response
    /// but will omit the raw audio data.
    ///
    /// Clients should check the `status` field of the Response to determine if it was successful
    /// (`completed`) or if there was another outcome: `cancelled`, `failed`, or `incomplete`.
    ///
    /// A response will contain all output items that were generated during the response, excluding any audio content.
    #[serde(rename = "response.done")]
    ResponseDone(RealtimeServerEventResponseDone),

    /// Returned when a new Item is created during Response generation.
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),

    /// Returned when an Item is done streaming. Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),

    /// Returned when a new content part is added to an assistant message item during response generation.
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),

    /// Returned when a content part is done streaming in an assistant message item.
    /// Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(RealtimeServerEventResponseContentPartDone),

    /// Returned when the text value of an "output_text" content part is updated.
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta(RealtimeServerEventResponseTextDelta),

    /// Returned when the text value of an "output_text" content part is done streaming.
    /// Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone(RealtimeServerEventResponseTextDone),

    /// Returned when the model-generated transcription of audio output is updated.
    #[serde(rename = "response.output_audio_transcript.delta")]
    ResponseOutputAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),

    /// Returned when the model-generated transcription of audio output is done streaming.
    /// Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.output_audio_transcript.done")]
    ResponseOutputAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),

    /// Returned when the model-generated audio is updated.
    #[serde(rename = "response.output_audio.delta")]
    ResponseOutputAudioDelta(RealtimeServerEventResponseAudioDelta),

    /// Returned when the model-generated audio is done.
    /// Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.output_audio.done")]
    ResponseOutputAudioDone(RealtimeServerEventResponseAudioDone),

    /// Returned when the model-generated function call arguments are updated.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),

    /// Returned when the model-generated function call arguments are done streaming.
    /// Also emitted when a Response is interrupted, incomplete, or cancelled.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),

    /// Returned when MCP tool call arguments are updated.
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMCPCallArgumentsDelta(RealtimeServerEventResponseMCPCallArgumentsDelta),

    /// Returned when MCP tool call arguments are finalized during response generation.
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMCPCallArgumentsDone(RealtimeServerEventResponseMCPCallArgumentsDone),

    /// Returned when an MCP tool call is in progress.
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMCPCallInProgress(RealtimeServerEventResponseMCPCallInProgress),

    /// Returned when an MCP tool call has completed successfully.
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMCPCallCompleted(RealtimeServerEventResponseMCPCallCompleted),

    /// Returned when an MCP tool call has failed.
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMCPCallFailed(RealtimeServerEventResponseMCPCallFailed),

    /// Returned when listing MCP tools is in progress for an item.
    #[serde(rename = "mcp_list_tools.in_progress")]
    MCPListToolsInProgress(RealtimeServerEventMCPListToolsInProgress),

    /// Returned when listing MCP tools has completed for an item.
    #[serde(rename = "mcp_list_tools.completed")]
    MCPListToolsCompleted(RealtimeServerEventMCPListToolsCompleted),

    /// Returned when listing MCP tools has failed for an item.
    #[serde(rename = "mcp_list_tools.failed")]
    MCPListToolsFailed(RealtimeServerEventMCPListToolsFailed),

    /// Emitted at the beginning of a Response to indicate the updated rate limits.
    /// When a Response is created some tokens will be "reserved" for the output tokens, the rate limits
    /// shown here reflect that reservation, which is then adjusted accordingly once the Response is completed.
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),
}

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

#[cfg(feature = "_api")]
impl_event_type! {
    RealtimeServerEventError => "error",
    RealtimeServerEventSessionCreated => "session.created",
    RealtimeServerEventSessionUpdated => "session.updated",
    RealtimeServerEventConversationItemAdded => "conversation.item.added",
    RealtimeServerEventConversationItemDone => "conversation.item.done",
    RealtimeServerEventInputAudioBufferCommitted => "input_audio_buffer.committed",
    RealtimeServerEventInputAudioBufferCleared => "input_audio_buffer.cleared",
    RealtimeServerEventInputAudioBufferSpeechStarted => "input_audio_buffer.speech_started",
    RealtimeServerEventInputAudioBufferSpeechStopped => "input_audio_buffer.speech_stopped",
    RealtimeServerEventInputAudioBufferTimeoutTriggered => "input_audio_buffer.timeout_triggered",
    RealtimeServerEventOutputAudioBufferStarted => "output_audio_buffer.started",
    RealtimeServerEventOutputAudioBufferStopped => "output_audio_buffer.stopped",
    RealtimeServerEventOutputAudioBufferCleared => "output_audio_buffer.cleared",
    RealtimeServerEventConversationItemInputAudioTranscriptionCompleted => "conversation.item.input_audio_transcription.completed",
    RealtimeServerEventConversationItemInputAudioTranscriptionDelta => "conversation.item.input_audio_transcription.delta",
    RealtimeServerEventConversationItemInputAudioTranscriptionFailed => "conversation.item.input_audio_transcription.failed",
    RealtimeServerEventConversationItemTruncated => "conversation.item.truncated",
    RealtimeServerEventConversationItemDeleted => "conversation.item.deleted",
    RealtimeServerEventConversationItemRetrieved => "conversation.item.retrieved",
    RealtimeServerEventConversationItemInputAudioTranscriptionSegment => "conversation.item.input_audio_transcription.segment",
    RealtimeServerEventResponseCreated => "response.created",
    RealtimeServerEventResponseDone => "response.done",
    RealtimeServerEventResponseOutputItemAdded => "response.output_item.added",
    RealtimeServerEventResponseOutputItemDone => "response.output_item.done",
    RealtimeServerEventResponseContentPartAdded => "response.content_part.added",
    RealtimeServerEventResponseContentPartDone => "response.content_part.done",
    RealtimeServerEventResponseTextDelta => "response.output_text.delta",
    RealtimeServerEventResponseTextDone => "response.output_text.done",
    RealtimeServerEventResponseAudioTranscriptDelta => "response.output_audio_transcript.delta",
    RealtimeServerEventResponseAudioTranscriptDone => "response.output_audio_transcript.done",
    RealtimeServerEventResponseAudioDelta => "response.output_audio.delta",
    RealtimeServerEventResponseAudioDone => "response.output_audio.done",
    RealtimeServerEventResponseFunctionCallArgumentsDelta => "response.function_call_arguments.delta",
    RealtimeServerEventResponseFunctionCallArgumentsDone => "response.function_call_arguments.done",
    RealtimeServerEventResponseMCPCallArgumentsDelta => "response.mcp_call_arguments.delta",
    RealtimeServerEventResponseMCPCallArgumentsDone => "response.mcp_call_arguments.done",
    RealtimeServerEventResponseMCPCallInProgress => "response.mcp_call.in_progress",
    RealtimeServerEventResponseMCPCallCompleted => "response.mcp_call.completed",
    RealtimeServerEventResponseMCPCallFailed => "response.mcp_call.failed",
    RealtimeServerEventMCPListToolsInProgress => "mcp_list_tools.in_progress",
    RealtimeServerEventMCPListToolsCompleted => "mcp_list_tools.completed",
    RealtimeServerEventMCPListToolsFailed => "mcp_list_tools.failed",
    RealtimeServerEventRateLimitsUpdated => "rate_limits.updated",
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for RealtimeServerEvent {
    fn event_type(&self) -> &'static str {
        match self {
            RealtimeServerEvent::Error(e) => e.event_type(),
            RealtimeServerEvent::SessionCreated(e) => e.event_type(),
            RealtimeServerEvent::SessionUpdated(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemAdded(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemDone(e) => e.event_type(),
            RealtimeServerEvent::InputAudioBufferCommitted(e) => e.event_type(),
            RealtimeServerEvent::InputAudioBufferCleared(e) => e.event_type(),
            RealtimeServerEvent::InputAudioBufferSpeechStarted(e) => e.event_type(),
            RealtimeServerEvent::InputAudioBufferSpeechStopped(e) => e.event_type(),
            RealtimeServerEvent::InputAudioBufferTimeoutTriggered(e) => e.event_type(),
            RealtimeServerEvent::OutputAudioBufferStarted(e) => e.event_type(),
            RealtimeServerEvent::OutputAudioBufferStopped(e) => e.event_type(),
            RealtimeServerEvent::OutputAudioBufferCleared(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemInputAudioTranscriptionCompleted(e) => {
                e.event_type()
            }
            RealtimeServerEvent::ConversationItemInputAudioTranscriptionDelta(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemInputAudioTranscriptionFailed(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemTruncated(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemDeleted(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemRetrieved(e) => e.event_type(),
            RealtimeServerEvent::ConversationItemInputAudioTranscriptionSegment(e) => {
                e.event_type()
            }
            RealtimeServerEvent::ResponseCreated(e) => e.event_type(),
            RealtimeServerEvent::ResponseDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputItemAdded(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputItemDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseContentPartAdded(e) => e.event_type(),
            RealtimeServerEvent::ResponseContentPartDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputTextDelta(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputTextDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputAudioTranscriptDelta(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputAudioTranscriptDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputAudioDelta(e) => e.event_type(),
            RealtimeServerEvent::ResponseOutputAudioDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseFunctionCallArgumentsDelta(e) => e.event_type(),
            RealtimeServerEvent::ResponseFunctionCallArgumentsDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseMCPCallArgumentsDelta(e) => e.event_type(),
            RealtimeServerEvent::ResponseMCPCallArgumentsDone(e) => e.event_type(),
            RealtimeServerEvent::ResponseMCPCallInProgress(e) => e.event_type(),
            RealtimeServerEvent::ResponseMCPCallCompleted(e) => e.event_type(),
            RealtimeServerEvent::ResponseMCPCallFailed(e) => e.event_type(),
            RealtimeServerEvent::MCPListToolsInProgress(e) => e.event_type(),
            RealtimeServerEvent::MCPListToolsCompleted(e) => e.event_type(),
            RealtimeServerEvent::MCPListToolsFailed(e) => e.event_type(),
            RealtimeServerEvent::RateLimitsUpdated(e) => e.event_type(),
        }
    }
}
