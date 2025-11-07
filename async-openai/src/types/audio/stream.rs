use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::{error::OpenAIError, traits::EventType};

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
