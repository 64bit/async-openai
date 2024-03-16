use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::InputSource;
use crate::error::OpenAIError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AudioInput {
    pub source: InputSource,
}

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpeechResponseFormat {
    #[default]
    Mp3,
    Opus,
    Aac,
    Flac,
    Pcm,
    Wav,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Voice {
    #[default]
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub enum SpeechModel {
    #[default]
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimestampGranularity {
    Word,
    #[default]
    Segment,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateTranscriptionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateTranscriptionRequest {
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg, mpga, m4a, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. Only `whisper-1` is currently available.
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text/prompting) should match the audio language.
    pub prompt: Option<String>,

    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: Option<AudioResponseFormat>,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0

    /// The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format will improve accuracy and latency.
    pub language: Option<String>,

    ///  The timestamp granularities to populate for this transcription. Any of these options: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponse {
    /// Transcribed text.
    pub text: String,

    /// If [`CreateTranscriptionRequestArgs::response_format`] is set to
    /// [`AudioResponseFormat::VerboseJson`], this field will be populated with
    /// the name of the language detected in the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// If [`CreateTranscriptionRequestArgs::response_format`] is set to
    /// [`AudioResponseFormat::VerboseJson`], this field will be populated with
    /// the duration of the audio in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,

    /// If [`CreateTranscriptionRequestArgs::response_format`] is set to
    /// [`AudioResponseFormat::VerboseJson`] and
    /// [`CreateTranscriptionRequestArgs::timestamp_granularities`] contains
    /// [`TimestampGranularity::Word`], this field will be populated with the
    /// word-level information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<CreateTranscriptionResponseWord>>,

    /// If [`CreateTranscriptionRequestArgs::response_format`] is set to
    /// [`AudioResponseFormat::VerboseJson`] and
    /// [`CreateTranscriptionRequestArgs::timestamp_granularities`] contains
    /// [`TimestampGranularity::Segment`], this field will be populated with the
    /// segment-level information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<CreateTranscriptionResponseSegment>>,

    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponseWord {
    /// The word.
    pub word: String,

    /// The start time of the word in seconds.
    pub start: f32,

    /// The end time of the word in seconds.
    pub end: f32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponseSegment {
    /// Unique identifier of the segment.
    pub id: i32,

    // Seek offset of the segment.
    pub seek: i32,

    /// Start time of the segment in seconds.
    pub start: f32,

    /// End time of the segment in seconds.
    pub end: f32,

    /// Transcribed text of the segment.
    pub text: String,

    /// Token IDs.
    pub tokens: Vec<i32>,

    /// Temperature parameter used for generating the segment.
    pub temperature: f32,

    /// Average log probability of the segment.
    pub avg_logprob: f32,

    /// Compression ratio of the segment.
    pub compression_ratio: f32,

    /// Probability of no speech in the segment.
    pub no_speech_prob: f32,
}

#[derive(Clone, Default, Debug, Builder, PartialEq, Serialize)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateSpeechRequest {
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,

    /// One of the available [TTS models](https://platform.openai.com/docs/models/tts): `tts-1` or `tts-1-hd`
    pub model: SpeechModel,

    /// The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech/voice-options).
    pub voice: Voice,

    /// The format to return audio in.
    /// Supported formats are `mp3`, `opus`, `aac`, `flac`, `pcm`, and `wav`.
    ///
    /// The `pcm` audio format, similar to `wav` but without a header, utilizes a 24kHz sample rate, mono channel, and 16-bit depth in signed little-endian format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,

    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>, // default: 1.0
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateTranslationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateTranslationRequest {
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg, mpga, m4a, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. Only `whisper-1` is currently available.
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text/prompting) should be in English.
    pub prompt: Option<String>,

    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: Option<AudioResponseFormat>,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateTranslationResponse {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct CreateSpeechResponse {
    pub bytes: Bytes,
}
