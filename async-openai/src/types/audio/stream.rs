use serde::{Deserialize, Serialize};

use crate::types::audio::{LogProbProperties, TranscriptTextUsageTokens};

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

#[cfg(feature = "_api")]
pub type TranscriptionResponseStream = std::pin::Pin<
    Box<
        dyn futures::Stream<
                Item = Result<CreateTranscriptionResponseStreamEvent, crate::error::OpenAIError>,
            > + Send,
    >,
>;

/// Stream of response events
#[cfg(feature = "_api")]
pub type SpeechResponseStream = std::pin::Pin<
    Box<
        dyn futures::Stream<
                Item = Result<CreateSpeechResponseStreamEvent, crate::error::OpenAIError>,
            > + Send,
    >,
>;

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
    SpeechAudioDeltaEvent => "speech.audio.delta",
    SpeechAudioDoneEvent => "speech.audio.done",
    TranscriptionTextSegmentEvent => "transcript.text.segment",
    TranscriptionTextDeltaEvent => "transcript.text.delta",
    TranscriptionTextDoneEvent => "transcript.text.done",
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for CreateSpeechResponseStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            CreateSpeechResponseStreamEvent::SpeechAudioDelta(event) => event.event_type(),
            CreateSpeechResponseStreamEvent::SpeechAudioDone(event) => event.event_type(),
        }
    }
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for CreateTranscriptionResponseStreamEvent {
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
