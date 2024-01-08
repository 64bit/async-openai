use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::InputSource;
use crate::error::OpenAIError;

#[cfg_attr(not(feature = "wasm"), derive(Default))]
#[derive(Debug, Clone, PartialEq)]
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
#[serde(rename_all = "snake_case")]
pub enum SpeechResponseFormat {
    #[default]
    Mp3,
    Opus,
    Aac,
    Flac,
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

#[cfg_attr(not(feature = "wasm"), derive(Default, Builder))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(not(feature = "wasm"), builder(name = "CreateTranscriptionRequestArgs"))]
#[cfg_attr(not(feature = "wasm"), builder(pattern = "mutable"))]
#[cfg_attr(not(feature = "wasm"), builder(setter(into, strip_option), default))]
#[cfg_attr(not(feature = "wasm"), builder(derive(Debug)))]
#[cfg_attr(not(feature = "wasm"), builder(build_fn(error = "OpenAIError")))]
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
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponse {
    pub text: String,
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

    /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,

    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>, // default: 1.0
}

#[cfg_attr(not(feature = "wasm"), derive(Default, Builder))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(not(feature = "wasm"), builder(name = "CreateTranslationRequestArgs"))]
#[cfg_attr(not(feature = "wasm"), builder(pattern = "mutable"))]
#[cfg_attr(not(feature = "wasm"), builder(setter(into, strip_option), default))]
#[cfg_attr(not(feature = "wasm"), builder(derive(Debug)))]
#[cfg_attr(not(feature = "wasm"), builder(build_fn(error = "OpenAIError")))]
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
