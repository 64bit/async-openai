use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AudioFormat {
    #[serde(rename = "pcm16")]
    PCM16,
    #[serde(rename = "g711_ulaw")]
    G711ULAW,
    #[serde(rename = "g711_alaw")]
    G711ALAW,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NoiseReductionType {
    NearField,
    FarField,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputAudioNoiseReduction {
    /// Type of noise reduction. `near_field` is for close-talking microphones such as
    /// headphones, `far_field` is for far-field microphones such as laptop or
    /// conference room microphones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<NoiseReductionType>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AudioTranscription {
    /// The language of the input audio. Supplying the input language in ISO-639-1 (e.g. en) format will improve accuracy and latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The model to use for transcription, current options are gpt-4o-transcribe, gpt-4o-mini-transcribe, and whisper-1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// An optional text to guide the model's style or continue a previous audio segment.
    /// For whisper-1, the prompt is a list of keywords. For gpt-4o-transcribe models,
    /// the prompt is a free text string, for example "expect words related to technology".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TurnDetection {
    /// Type of turn detection, only "server_vad" is currently supported.
    #[serde(rename = "server_vad")]
    ServerVAD {
        /// Activation threshold for VAD (0.0 to 1.0).
        threshold: f32,
        /// Amount of audio to include before speech starts (in milliseconds).
        prefix_padding_ms: u32,
        /// Duration of silence to detect speech stop (in milliseconds).
        silence_duration_ms: u32,

        /// Whether or not to automatically generate a response when a VAD stop event occurs.
        #[serde(skip_serializing_if = "Option::is_none")]
        create_response: Option<bool>,

        /// Whether or not to automatically interrupt any ongoing response with output to
        /// the default conversation (i.e. conversation of auto) when a VAD start event occurs.
        #[serde(skip_serializing_if = "Option::is_none")]
        interrupt_response: Option<bool>,
    },

    #[serde(rename = "semantic_vad")]
    SemanticVAD {
        /// The eagerness of the model to respond.
        /// `low` will wait longer for the user to continue speaking,
        /// `high`` will respond more quickly. `auto`` is the default and is equivalent to `medium`
        eagerness: String,

        /// Whether or not to automatically generate a response when a VAD stop event occurs.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        create_response: Option<bool>,

        /// Whether or not to automatically interrupt any ongoing response with output to
        /// the default conversation (i.e. conversation of auto) when a VAD start event occurs.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        interrupt_response: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MaxResponseOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Num(u16),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TracingConfiguration {
    /// The group id to attach to this trace to enable filtering and grouping in the traces dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// The arbitrary metadata to attach to this trace to enable filtering in the traces dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// The name of the workflow to attach to this trace. This is used to name the trace in the traces dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TracingOption {
    /// Auto tracing with default values
    #[serde(rename = "auto")]
    Auto,
    /// Granular tracing configuration
    #[serde(rename = "config")]
    Config(TracingConfiguration),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ToolDefinition {
    #[serde(rename = "function")]
    Function {
        /// The name of the function.
        name: String,
        /// The description of the function.
        description: String,
        /// Parameters of the function in JSON Schema.
        parameters: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    #[serde(untagged)]
    Function {
        r#type: FunctionType,
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RealtimeModel {
    #[serde(rename = "gpt-4o-realtime-preview")]
    GPT4ORealtimePreview,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    GPT4ORealtimePreview20241001,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    GPT4ORealtimePreview20241217,
    #[serde(rename = "gpt-4o-realtime-preview-2025-06-03")]
    GPT4ORealtimePreview20250603,
    #[serde(rename = "gpt-4o-mini-realtime-preview")]
    GPT4OMiniRealtimePreview,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    GPT4OMiniRealtimePreview20241217,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionResource {
    /// The default model used for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<RealtimeModel>,

    /// The set of modalities the model can respond with. To disable audio, set this to ["text"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,

    //// The default system instructions prepended to model calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The voice the model uses to respond. Cannot be changed once the model has responded with audio at least once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,

    /// The speed of the model's spoken response. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed.
    /// This value can only be changed in between model turns, not while a response is in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,

    /// The format of input audio. Options are "pcm16", "g711_ulaw", or "g711_alaw".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<AudioFormat>,

    /// Configuration for input audio noise reduction. This can be set to `null` to turn off.
    /// Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
    /// Filtering the audio can improve VAD and turn detection accuracy (reducing false positives)
    /// and model performance by improving perception of the input audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<InputAudioNoiseReduction>,

    /// The format of output audio. Options are "pcm16", "g711_ulaw", or "g711_alaw".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<AudioFormat>,

    /// Configuration for input audio transcription. Can be set to null to turn off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,

    /// Configuration for turn detection. Can be set to null to turn off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<TurnDetection>,

    /// Tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// How the model chooses tools.
    pub tool_choice: Option<ToolChoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sampling temperature for the model.
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingOption>,

    /// Maximum number of output tokens for a single assistant response, inclusive of tool calls.
    /// Provide an integer between 1 and 4096 to limit output tokens, or "inf" for the maximum available tokens for a given model.
    /// Defaults to "inf".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<MaxResponseOutputTokens>,
}
