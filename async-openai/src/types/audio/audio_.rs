use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::audio::{LogProbProperties, TranscriptTextUsageDuration, TranscriptionUsage};
use crate::types::InputSource;

// openapi spec type: VoiceIdsShared
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    #[default]
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
    Verse,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AudioInput {
    pub source: InputSource,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
    DiarizedJson,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TranslationResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
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

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpeechModel {
    #[default]
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(rename = "gpt-4o-mini-tts")]
    Gpt4oMiniTts,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
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
    /// The audio file object (not file name) to transcribe, in one of these formats:
    /// flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, `whisper-1`
    /// (which is powered by our open source Whisper V2 model), and `gpt-4o-transcribe-diarize`.
    pub model: String,

    /// The language of the input audio. Supplying the input language in
    /// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve
    /// accuracy and latency.
    pub language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio segment. The
    /// [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should match the audio
    /// language. This field is not supported when using `gpt-4o-transcribe-diarize`.
    pub prompt: Option<String>,

    /// The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, `vtt`, or
    /// `diarized_json`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is
    /// `json`. For `gpt-4o-transcribe-diarize`, the supported formats are `json`, `text`, and
    /// `diarized_json`, with `diarized_json` required to receive speaker annotations.
    pub response_format: Option<AudioResponseFormat>,

    /// TThe sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more
    /// random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the
    /// model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically
    /// increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0

    /// Additional information to include in the transcription response.

    /// `logprobs` will return the log probabilities of the tokens in the
    /// response to understand the model's confidence in the transcription.
    /// `logprobs` only works with response_format set to `json` and only with
    /// the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`. This field is not supported when
    /// using `gpt-4o-transcribe-diarize`.
    pub include: Option<Vec<TranscriptionInclude>>,

    /// The timestamp granularities to populate for this transcription. `response_format` must be set
    /// `verbose_json` to use timestamp granularities. Either or both of these options are supported:
    /// `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating
    /// word timestamps incurs additional latency. This option is not available for `gpt-4o-transcribe-diarize`.
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    /// See the [Streaming section of the Speech-to-Text guide](https://platform.openai.com/docs/guides/speech-to-text?lang=curl#streaming-transcriptions)
    /// for more information.
    /// Note: Streaming is not supported for the `whisper-1` model and will be ignored.    
    pub stream: Option<bool>,

    /// Controls how the audio is cut into chunks. When set to `"auto"`, the server first normalizes
    /// loudness and then uses voice activity detection (VAD) to choose boundaries. `server_vad` object
    /// can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as
    /// a single block. Required when using `gpt-4o-transcribe-diarize` for inputs longer than 30
    /// seconds.
    pub chunking_strategy: Option<TranscriptionChunkingStrategy>,

    /// Optional list of speaker names that correspond to the audio samples provided in
    /// `known_speaker_references[]`. Each entry should be a short identifier (for example `customer` or
    /// `agent`). Up to 4 speakers are supported.
    pub known_speaker_names: Option<Vec<String>>,

    /// Optional list of audio samples (as [data
    /// URLs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)) that contain
    /// known speaker references matching `known_speaker_names[]`. Each sample must be between 2 and 10
    /// seconds, and can use any of the same input audio formats supported by `file`.
    pub known_speaker_references: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionChunkingStrategy {
    #[default]
    Auto,
    #[serde(untagged)]
    ServerVad(VadConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum VadConfigType {
    #[default]
    ServerVad,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct VadConfig {
    pub r#type: VadConfigType,
    /// Amount of audio to include before the VAD detected speech (in milliseconds). Default: 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u32>,

    /// Duration of silence to detect speech stop (in milliseconds).
    /// With shorter values the model will respond more quickly,
    /// but may jump in on short pauses from the user. Default: 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u32>,

    /// Sensitivity threshold (0.0 to 1.0) for voice activity detection. A
    /// higher threshold will require louder audio to activate the model, and
    /// thus might perform better in noisy environments. Default: 0.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionInclude {
    Logprobs,
}

/// Represents a transcription response returned by model, based on the provided
/// input.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponseJson {
    /// The transcribed text.
    pub text: String,

    /// The log probabilities of the tokens in the transcription. Only returned with the models
    /// `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.
    pub logprobs: Option<Vec<LogProbProperties>>,

    /// Token usage statistics for the request.
    pub usage: TranscriptionUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CreateTranscriptionResponseDiarizedJsonTask {
    Transcribe,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTranscriptionResponseDiarizedJson {
    /// The type of task that was run. Always `transcribe`.
    pub task: Option<CreateTranscriptionResponseDiarizedJsonTask>,

    /// Duration of the input audio in seconds.
    pub duration: Option<f32>,

    /// The concatenated transcript text for the entire audio input.
    pub text: String,

    /// Segments of the transcript annotated with timestamps and speaker labels.
    pub segments: Vec<TranscriptionDiarizedSegment>,

    /// Token or duration usage statistics for the request.
    pub usage: TranscriptionUsage,
}

/// Represents a verbose json transcription response returned by model, based on
/// the provided input.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponseVerboseJson {
    /// The language of the input audio.
    pub language: String,

    /// The duration of the input audio.
    pub duration: f32,

    /// The transcribed text.
    pub text: String,

    /// Extracted words and their corresponding timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<TranscriptionWord>>,

    /// Segments of the transcribed text and their corresponding details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,

    /// Usage statistics for models billed by audio input duration.
    pub usage: TranscriptTextUsageDuration,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TranscriptionWord {
    /// The text content of the word.
    pub word: String,

    /// Start time of the word in seconds.
    pub start: f32,

    /// End time of the word in seconds.
    pub end: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TranscriptionDiarizedSegmentType {
    #[serde(rename = "transcript.text.segment")]
    TranscriptTextSegment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionDiarizedSegment {
    /// The type of the segment. Always `transcript.text.segment`.
    pub r#type: TranscriptionDiarizedSegmentType,

    /// Unique identifier for the segment.
    pub id: String,

    /// Start timestamp of the segment in seconds.
    pub start: f32,

    /// End timestamp of the segment in seconds.
    pub end: f32,

    /// Transcript text for this segment.
    pub text: String,

    /// Speaker label for this segment.
    /// When known speakers are provided, the label matches known_speaker_names[].
    /// Otherwise speakers are labeled sequentially using capital letters (`A`, `B`, ...).
    pub speaker: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TranscriptionSegment {
    /// Unique identifier of the segment.
    pub id: u32,

    // Seek offset of the segment.
    pub seek: u32,

    /// Start time of the segment in seconds.
    pub start: f32,

    /// End time of the segment in seconds.
    pub end: f32,

    /// Text content of the segment.
    pub text: String,

    /// Array of token IDs for the text content.
    pub tokens: Vec<u32>,

    /// Temperature parameter used for generating the segment.
    pub temperature: f32,

    /// Average logprob of the segment. If the value is lower than -1, consider
    /// the logprobs failed.
    pub avg_logprob: f32,

    /// Compression ratio of the segment. If the value is greater than 2.4,
    /// consider the compression failed.
    pub compression_ratio: f32,

    /// Probability of no speech in the segment. If the value is higher than 1.0
    /// and the `avg_logprob` is below -1, consider this segment silent.
    pub no_speech_prob: f32,
}

#[derive(Clone, Default, Debug, Builder, PartialEq, Serialize, Deserialize)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateSpeechRequest {
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,

    /// One of the available [TTS models](https://platform.openai.com/docs/models#tts): `tts-1`,
    /// `tts-1-hd` or `gpt-4o-mini-tts`.
    pub model: SpeechModel,

    /// The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer` and `verse`.

    /// The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`,
    /// `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices
    /// are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech#voice-options).
    pub voice: Voice,

    /// Control the voice of your generated audio with additional instructions.
    /// Does not work with `tts-1` or `tts-1-hd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,

    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>, // default: 1.0

    /// The format to stream the audio in. Supported formats are `sse` and `audio`. `sse` is not
    /// supported for `tts-1` or `tts-1-hd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_format: Option<StreamFormat>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StreamFormat {
    #[default]
    #[serde(rename = "sse")]
    SSE,
    #[serde(rename = "audio")]
    Audio,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateTranslationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateTranslationRequest {
    /// The audio file object (not file name) translate, in one of these
    /// formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. Only `whisper-1` (which is powered by our open
    /// source Whisper V2 model) is currently available.
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text#prompting) should be in English.
    pub prompt: Option<String>,

    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: Option<TranslationResponseFormat>,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateTranslationResponseJson {
    pub text: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranslationResponseVerboseJson {
    /// The language of the output translation (always `english`).
    pub language: String,
    /// The duration of the input audio.
    pub duration: String,
    /// The translated text.
    pub text: String,
    /// Segments of the translated text and their corresponding details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}

#[derive(Debug, Clone)]
pub struct CreateSpeechResponse {
    pub bytes: Bytes,
}
