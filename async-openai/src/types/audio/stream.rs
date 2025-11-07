use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::{
    error::OpenAIError,
    traits::EventType,
    types::{audio::TranscriptTextUsageTokens, LogProbProperties},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateSpeechResponseStreamEvent {
    /// Emitted for each chunk of audio data generated during speech synthesis.
    #[serde(rename = "speech.audio.delta")]
    SpeechAudioDelta(SpeechAudioDeltaEvent),
    /// Emitted when the speech synthesis is complete and all audio has been streamed.
    #[serde(rename = "speech.audio.done")]
    SpeechAudioDone(SpeechAudioDoneEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpeechAudioDeltaEvent {
    /// A chunk of Base64-encoded audio data.
    pub audio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpeechUsage {
    /// Number of input tokens in the prompt.
    pub input_tokens: u32,
    /// Number of output tokens generated.
    pub output_tokens: u32,
    /// Total number of tokens used (input + output).
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpeechAudioDoneEvent {
    /// Token usage statistics for the request.
    pub usage: SpeechUsage,
}

/// Stream of response events
pub type SpeechResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateSpeechResponseStreamEvent, OpenAIError>> + Send>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TranscriptionTextSegmentEvent {
    /// Unique identifier for the segment.
    pub id: String,
    /// Start timestamp of the segment in seconds.
    pub start: f32,
    /// End timestamp of the segment in seconds.
    pub end: f32,
    /// Transcript text for this segment.
    pub text: String,
    /// Speaker label for this segment.
    pub speaker: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionTextDeltaEvent {
    /// The text delta that was additionally transcribed.
    pub delta: String,
    /// The log probabilities of the individual tokens in the transcription.
    /// Only included if you [create a transcription](https://platform.openai.com/docs/api-reference/audio/create-transcription) with
    /// the `include[]` parameter set to `logprobs`.
    pub logprobs: Option<Vec<LogProbProperties>>,
    /// Identifier of the diarized segment that this delta belongs to. Only present when using
    /// `gpt-4o-transcribe-diarize`.
    pub segment_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionTextDoneEvent {
    /// The text that was transcribed.
    pub text: String,
    /// The log probabilities of the individual tokens in the transcription.
    /// Only included if you [create a transcription](https://platform.openai.com/docs/api-reference/audio/create-transcription) with
    /// the `include[]` parameter set to `logprobs`.
    pub logprobs: Option<Vec<LogProbProperties>>,
    /// Usage statistics for models billed by token usage.
    pub usage: TranscriptTextUsageTokens,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum CreateTranscriptionResponseStreamEvent {
    /// Emitted when a diarized transcription returns a completed segment with speaker information. Only
    /// emitted when you [create a transcription](https://platform.openai.com/docs/api-reference/audio/create-transcription) with
    /// `stream` set to `true` and `response_format` set to `diarized_json`.
    #[serde(rename = "transcript.text.segment")]
    TranscriptTextSegment(TranscriptionTextSegmentEvent),
    #[serde(rename = "transcript.text.delta")]
    TranscriptTextDelta(TranscriptionTextDeltaEvent),
    /// Emitted when the transcription is complete. Contains the complete transcription text. Only emitted
    /// when you [create a transcription](https://platform.openai.com/docs/api-reference/audio/create-transcription) with the
    /// `Stream` parameter set to `true`.
    #[serde(rename = "transcript.text.done")]
    TranscriptTextDone(TranscriptionTextDoneEvent),
}

pub type TranscriptionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateTranscriptionResponseStreamEvent, OpenAIError>> + Send>>;

impl EventType for SpeechAudioDeltaEvent {
    fn event_type(&self) -> &'static str {
        "speech.audio.delta"
    }
}

impl EventType for SpeechAudioDoneEvent {
    fn event_type(&self) -> &'static str {
        "speech.audio.done"
    }
}

impl EventType for CreateSpeechResponseStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            CreateSpeechResponseStreamEvent::SpeechAudioDelta(event) => event.event_type(),
            CreateSpeechResponseStreamEvent::SpeechAudioDone(event) => event.event_type(),
        }
    }
}

impl EventType for TranscriptionTextSegmentEvent {
    fn event_type(&self) -> &'static str {
        "transcript.text.segment"
    }
}

impl EventType for TranscriptionTextDeltaEvent {
    fn event_type(&self) -> &'static str {
        "transcript.text.delta"
    }
}

impl EventType for TranscriptionTextDoneEvent {
    fn event_type(&self) -> &'static str {
        "transcript.text.done"
    }
}

impl EventType for CreateTranscriptionResponseStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            CreateTranscriptionResponseStreamEvent::TranscriptTextSegment(event) => {
                event.event_type()
            }
            CreateTranscriptionResponseStreamEvent::TranscriptTextDelta(event) => {
                event.event_type()
            }
            CreateTranscriptionResponseStreamEvent::TranscriptTextDone(event) => event.event_type(),
        }
    }
}
