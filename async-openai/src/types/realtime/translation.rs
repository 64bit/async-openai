use serde::{Deserialize, Serialize};

use crate::types::realtime::{NoiseReductionType, RealtimeServerEventError};

/// Optional source-language transcription configuration for a translation session.
/// When configured, the server emits `session.input_transcript.delta` events.
/// Translation itself still runs from the input audio stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationInputTranscription {
    /// The transcription model used for source transcript deltas.
    pub model: String,
}

/// Optional input noise reduction for a translation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationNoiseReduction {
    pub r#type: NoiseReductionType,
}

/// Configuration for translation input audio.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationInputAudio {
    // DONT add because it can be 'null': #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional source-language transcription. Set to `null` to disable.
    pub transcription: Option<RealtimeTranslationInputTranscription>,
    // DONT add because it can be 'null': #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional input noise reduction. Set to `null` to disable.
    pub noise_reduction: Option<RealtimeTranslationNoiseReduction>,
}

/// Configuration for translation output audio.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationOutputAudio {
    /// Target language for translated output audio and transcript deltas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Configuration for translation input and output audio.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeTranslationInputAudio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeTranslationOutputAudio>,
}

/// A Realtime translation session. Translation sessions continuously translate input
/// audio into the configured output language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationSession {
    /// Unique identifier for the session that looks like `sess_1234567890abcdef`.
    pub id: String,
    /// The session type. Always `translation` for Realtime translation sessions.
    pub r#type: String,
    /// Expiration timestamp for the session, in seconds since epoch.
    pub expires_at: u64,
    /// The Realtime translation model used for this session.
    pub model: String,
    /// Configuration for translation input and output audio.
    pub audio: RealtimeTranslationAudio,
}

/// Realtime translation session configuration. Translation sessions stream
/// source audio in and translated audio plus transcript deltas out continuously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationSessionCreateRequest {
    /// The Realtime translation model used for this session.
    pub model: String,
    /// Configuration for translation input and output audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranslationAudio>,
}

/// Realtime translation session fields that can be updated with `session.update`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationSessionUpdateRequest {
    /// Configuration for translation input and output audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranslationAudio>,
}

/// The anchor point for the translation client secret expiration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeTranslationClientSecretExpiresAnchor {
    #[default]
    CreatedAt,
}

/// Configuration for the client secret expiration. Expiration refers to
/// the time after which a client secret will no longer be valid for creating sessions.
/// The session itself may continue after that time once started. A secret can be used to
/// create multiple sessions until it expires.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientSecretExpiresAfter {
    /// The anchor point for the client secret expiration, meaning that
    /// `seconds` will be added to the `created_at` time of the client
    /// secret to produce an expiration timestamp. Only `created_at` is currently supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<RealtimeTranslationClientSecretExpiresAnchor>,
    /// The number of seconds from the anchor point to the expiration. Select a value between
    /// `10` and `7200` (2 hours). Defaults to 600 seconds (10 minutes) if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u32>,
}

/// Create a translation session and client secret for the Realtime API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientSecretCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<RealtimeTranslationClientSecretExpiresAfter>,
    pub session: RealtimeTranslationSessionCreateRequest,
}

/// Response from creating a translation session and client secret for the Realtime API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientSecretCreateResponse {
    /// The generated client secret value.
    pub value: String,
    /// Expiration timestamp for the client secret, in seconds since epoch.
    pub expires_at: u64,
    /// The translation session.
    pub session: RealtimeTranslationSession,
}

/// Send this event to update the translation session configuration.
/// Translation sessions support updates to `audio.output.language`,
/// `audio.input.transcription`, and `audio.input.noise_reduction`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientEventSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Translation session fields to update. The session `type` and `model` are set
    /// at creation and cannot be changed with `session.update`.
    pub session: RealtimeTranslationSessionUpdateRequest,
}

/// Send this event to append audio bytes to the translation session input audio buffer.
///
/// WebSocket translation sessions accept base64-encoded 24 kHz PCM16 mono
/// little-endian raw audio bytes. Unsupported websocket audio formats
/// return a validation error because lower-quality audio materially degrades translation
/// quality.
///
/// Translation consumes 200 ms engine frames. For best realtime behavior,
/// append audio in 200 ms chunks. If a chunk is shorter, the server buffers it
/// until it has enough audio for one frame. If a chunk is longer, the server splits
/// it into 200 ms frames and enqueues them back-to-back.
///
/// Keep appending silence while the session is active. If a client stops
/// sending audio and later resumes, model time treats the resumed audio as
/// contiguous with the previous audio rather than as a real-world pause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientEventInputAudioBufferAppend {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Base64-encoded 24 kHz PCM16 mono audio bytes.
    pub audio: String,
}

/// Gracefully close the realtime translation session. The server flushes pending
/// input audio and emits any remaining translated output before closing the session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationClientEventSessionClose {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// A Realtime translation client event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeTranslationClientEvent {
    #[serde(rename = "session.update")]
    SessionUpdate(RealtimeTranslationClientEventSessionUpdate),
    #[serde(rename = "session.input_audio_buffer.append")]
    InputAudioBufferAppend(RealtimeTranslationClientEventInputAudioBufferAppend),
    #[serde(rename = "session.close")]
    SessionClose(RealtimeTranslationClientEventSessionClose),
}

/// Audio encoding for the translated audio delta.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTranslationAudioFormat {
    PCM16,
}

/// Returned when a translation session is created. Emitted automatically when a
/// new connection is established as the first server event. This event contains
/// the default translation session configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The translation session configuration.
    pub session: RealtimeTranslationSession,
}

/// Returned when a translation session is updated with a `session.update` event,
/// unless there is an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The translation session configuration.
    pub session: RealtimeTranslationSession,
}

/// Returned when a realtime translation session is closed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionClosed {
    /// The unique ID of the server event.
    pub event_id: String,
}

/// Returned when optional source-language transcript text is available. This event
/// is emitted only when `audio.input.transcription` is configured.
///
/// Transcript deltas are append-only text fragments. Clients should not insert
/// unconditional spaces between deltas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionInputTranscriptDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Append-only source-language transcript text.
    pub delta: String,
    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. It advances in 200 ms increments, but multiple transcript
    /// deltas may share the same `elapsed_ms`. Treat it as alignment metadata,
    /// not a unique transcript-delta identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
}

/// Returned when translated transcript text is available.
///
/// Transcript deltas are append-only text fragments. Clients should not insert
/// unconditional spaces between deltas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionOutputTranscriptDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Append-only transcript text for the translated output audio.
    pub delta: String,
    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. It advances in 200 ms increments, but multiple transcript
    /// deltas may share the same `elapsed_ms`. Treat it as alignment metadata,
    /// not a unique transcript-delta identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
}

/// Returned when translated output audio is available. Output audio deltas are
/// 200 ms frames of PCM16 audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationServerEventSessionOutputAudioDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// Base64-encoded translated audio data.
    pub delta: String,
    /// Sample rate of the audio delta. Defaults to 24000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    /// Number of audio channels. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<u32>,
    /// Audio encoding for `delta`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeTranslationAudioFormat>,
    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. Treat `elapsed_ms` as alignment metadata, not a unique
    /// event identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
}

/// A Realtime translation server event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeTranslationServerEvent {
    #[serde(rename = "error")]
    Error(RealtimeServerEventError),
    #[serde(rename = "session.created")]
    SessionCreated(RealtimeTranslationServerEventSessionCreated),
    #[serde(rename = "session.updated")]
    SessionUpdated(RealtimeTranslationServerEventSessionUpdated),
    #[serde(rename = "session.closed")]
    SessionClosed(RealtimeTranslationServerEventSessionClosed),
    #[serde(rename = "session.input_transcript.delta")]
    SessionInputTranscriptDelta(RealtimeTranslationServerEventSessionInputTranscriptDelta),
    #[serde(rename = "session.output_transcript.delta")]
    SessionOutputTranscriptDelta(RealtimeTranslationServerEventSessionOutputTranscriptDelta),
    #[serde(rename = "session.output_audio.delta")]
    SessionOutputAudioDelta(RealtimeTranslationServerEventSessionOutputAudioDelta),
}
