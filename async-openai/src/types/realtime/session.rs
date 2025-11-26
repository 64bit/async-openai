use serde::{Deserialize, Serialize};

use crate::types::{
    mcp::MCPTool,
    responses::{Prompt, ToolChoiceFunction, ToolChoiceMCP, ToolChoiceOptions},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AudioTranscription {
    /// The language of the input audio. Supplying the input language in
    /// [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The model to use for transcription. Current options are `whisper-1`,
    /// `gpt-4o-mini-transcribe`, `gpt-4o-transcribe`, and `gpt-4o-transcribe-diarize`.
    /// Use `gpt-4o-transcribe-diarize` when you need diarization with speaker labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// An optional text to guide the model's style or continue a previous audio segment.
    /// For `whisper-1`, the [prompt is a list of keywords](https://platform.openai.com/docs/guides/speech-to-text#prompting). For `gpt-4o-transcribe` models
    /// (excluding gpt-4o-transcribe-diarize), the prompt is a free text string, for example
    /// "expect words related to technology".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RealtimeTurnDetection {
    /// Server-side voice activity detection (VAD) which flips on when user speech is detected
    /// and off after a period of silence.
    #[serde(rename = "server_vad")]
    ServerVAD {
        /// Whether or not to automatically generate a response when a VAD stop event occurs.
        #[serde(skip_serializing_if = "Option::is_none")]
        create_response: Option<bool>,

        /// Optional timeout after which a model response will be triggered automatically.
        /// This is useful for situations in which a long pause from the user is unexpected,
        /// such as a phone call. The model will effectively prompt the user to continue the
        /// conversation based on the current context.
        ///
        /// The timeout value will be applied after the last model response's audio has finished
        /// playing, i.e. it's set to the response.done time plus audio playback duration.
        ///
        /// An input_audio_buffer.timeout_triggered event (plus events associated with the Response)
        ///  will be emitted when the timeout is reached. Idle timeout is currently only supported
        /// for server_vad mode.
        #[serde(skip_serializing_if = "Option::is_none")]
        idle_timeout_ms: Option<u32>,

        /// Whether or not to automatically interrupt any ongoing response with output to
        /// the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
        #[serde(skip_serializing_if = "Option::is_none")]
        interrupt_response: Option<bool>,

        /// Used only for server_vad mode. Amount of audio to include before the VAD detected speech
        /// (in milliseconds). Defaults to 300ms.
        prefix_padding_ms: u32,
        /// Used only for server_vad mode. Duration of silence to detect speech stop
        /// (in milliseconds). Defaults to 500ms. With shorter values the model will respond
        ///  more quickly, but may jump in on short pauses from the user.
        silence_duration_ms: u32,

        /// Used only for server_vad mode. Activation threshold for VAD (0.0 to 1.0),
        /// this defaults to 0.5. A higher threshold will require louder audio to activate
        /// the model, and thus might perform better in noisy environments.
        threshold: f32,
    },

    /// Server-side semantic turn detection which uses a model to determine when the user has
    ///  finished speaking.
    #[serde(rename = "semantic_vad")]
    SemanticVAD {
        /// Whether or not to automatically generate a response when a VAD stop event occurs.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        create_response: Option<bool>,

        /// Used only for `semantic_vad` mode. The eagerness of the model to respond.
        /// `low` will wait longer for the user to continue speaking, `high` will respond more
        /// quickly. `auto` is the default and is equivalent to `medium`. `low`, `medium`, and `high`
        /// have max timeouts of 8s, 4s, and 2s respectively.
        eagerness: String,

        /// Whether or not to automatically interrupt any ongoing response with output to
        /// the default conversation (i.e. `conversation` of `auto`) when a VAD start event occurs.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        interrupt_response: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MaxOutputTokens {
    #[serde(rename = "inf")]
    Inf,
    #[serde(untagged)]
    Num(u16),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeFunctionTool {
    /// The name of the function.
    pub name: String,
    /// The description of the function, including guidance on when and how to call it,
    /// and guidance about what to tell the user when calling (if anything).
    pub description: String,
    /// Parameters of the function in JSON Schema.
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RealtimeTool {
    #[serde(rename = "function")]
    Function(RealtimeFunctionTool),
    /// Give the model access to additional tools via remote Model Context Protocol (MCP) servers.
    /// [Learn more about MCP](https://platform.openai.com/docs/guides/tools-remote-mcp).
    #[serde(rename = "mcp")]
    MCP(MCPTool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    /// Use this option to force the model to call a specific function.
    Function(ToolChoiceFunction),
    /// Use this option to force the model to call a specific tool on a remote MCP server.
    Mcp(ToolChoiceMCP),

    #[serde(untagged)]
    Mode(ToolChoiceOptions),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Sage,
    Shimmer,
    Verse,
    Marin,
    Cedar,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RealtimeAudioFormats {
    /// The PCM audio format. Only a 24kHz sample rate is supported.
    #[serde(rename = "audio/pcm")]
    PCMAudioFormat {
        /// The sample rate of the audio. Always 24000.
        rate: u32,
    },
    /// The G.711 μ-law format.
    #[serde(rename = "audio/pcmu")]
    PCMUAudioFormat,
    /// The G.711 A-law format.
    #[serde(rename = "audio/pcma")]
    PCMAAudioFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct G711ULAWAudioFormat {
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioInput {
    /// The format of the input audio.
    pub format: RealtimeAudioFormats,
    /// Configuration for input audio noise reduction. This can be set to null to turn off.
    /// Noise reduction filters audio added to the input audio buffer before it is sent to VAD
    /// and the model. Filtering the audio can improve VAD and turn detection accuracy
    /// (reducing false positives) and model performance by improving perception of the
    /// input audio.
    pub noise_reduction: Option<NoiseReductionType>,
    /// Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on.
    /// Input audio transcription is not native to the model, since the model consumes audio directly.
    /// Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription)
    /// and should be treated as guidance of input audio content rather than precisely what the model
    /// heard. The client can optionally set the language and prompt for transcription,
    /// these offer additional guidance to the transcription service.
    pub transcription: Option<AudioTranscription>,

    /// Configuration for turn detection, ether Server VAD or Semantic VAD. This can
    /// be set to null to turn off, in which case the client must manually trigger model response.
    ///
    ///  Server VAD means that the model will detect the start and end of speech
    /// based on audio volume and respond at the end of user speech.
    ///
    /// Semantic VAD is more advanced and uses a turn detection model (in conjunction with VAD)
    /// to semantically estimate whether the user has finished speaking, then dynamically sets
    /// a timeout based on this probability. For example, if user audio trails off with "uhhm",
    /// the model will score a low probability of turn end and wait longer for the user to
    /// continue speaking. This can be useful for more natural conversations, but may have a
    /// higher latency.    
    pub turn_detection: RealtimeTurnDetection,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioOutput {
    /// The format of the output audio.
    pub format: RealtimeAudioFormats,
    /// The speed of the model's spoken response as a multiple of the original speed.
    /// 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed.
    /// This value can only be changed in between model turns, not while a response
    /// is in progress.
    ///
    /// This parameter is a post-processing adjustment to the audio after it is generated,
    /// it's also possible to prompt the model to speak faster or slower.
    pub speed: f32,
    /// The voice the model uses to respond. Voice cannot be changed during the session once
    /// the model has responded with audio at least once. Current voice options are
    /// `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`, `marin`, and `cedar`.
    /// We recommend `marin` and `cedar` for best quality.
    pub voice: RealtimeVoice,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audio {
    pub input: AudioInput,
    pub output: AudioOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Tracing {
    /// Enables tracing and sets default values for tracing configuration options. Always `auto`.
    Auto,

    #[serde(untagged)]
    Configuration(TracingConfiguration),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TracingConfiguration {
    /// The group id to attach to this trace to enable filtering and grouping in the Traces Dashboard.
    pub group_id: String,
    /// The arbitrary metadata to attach to this trace to enable filtering in the Traces Dashboard.
    pub metadata: serde_json::Value,
    /// The name of the workflow to attach to this trace. This is used to name the trace in the Traces Dashboard.
    pub workflow_name: String,
}

/// The truncation strategy to use for the session.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeTruncation {
    /// `auto` is the default truncation strategy.
    Auto,
    /// `disabled` will disable truncation and emit errors when the conversation exceeds the input
    /// token limit.
    Disabled,

    /// Retain a fraction of the conversation tokens when the conversation exceeds the input token
    /// limit. This allows you to amortize truncations across multiple turns, which can help improve
    /// cached token usage.
    #[serde(untagged)]
    RetentionRatio(RetentionRatioTruncation),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetentionRatioTruncation {
    /// Fraction of post-instruction conversation tokens to retain (0.0 - 1.0) when the conversation
    ///  exceeds the input token limit. Setting this to 0.8 means that messages will be dropped
    /// until 80% of the maximum allowed tokens are used. This helps reduce the frequency of
    /// truncations and improve cache rates.
    pub retention_ratio: f32,

    /// Use retention ratio truncation.
    pub r#type: String,

    /// Optional custom token limits for this truncation strategy. If not provided, the model's
    ///  default token limits will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_limits: Option<TokenLimits>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenLimits {
    /// Maximum tokens allowed in the conversation after instructions (which including tool
    /// definitions). For example, setting this to 5,000 would mean that truncation would occur
    /// when the conversation exceeds 5,000 tokens after instructions. This cannot be higher
    /// than the model's context window size minus the maximum output tokens.
    pub post_instructions: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Session {
    // Boxed as per clippy suggestion:
    // https://rust-lang.github.io/rust -clippy/rust-1.91.0/index.html#large_enum_variant
    // the largest variant contains at least 600 bytes, the second-largest variant contains at least 144 bytes
    /// The type of session to create. Always `realtime` for the Realtime API.
    #[serde(rename = "realtime")]
    RealtimeSession(Box<RealtimeSession>),
    /// The type of session to create. Always `transcription` for transcription sessions.
    #[serde(rename = "transcription")]
    RealtimeTranscriptionSession(RealtimeTranscriptionSession),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RealtimeSessionConfiguration {
    Realtime(RealtimeSession),
}

impl Default for RealtimeSessionConfiguration {
    fn default() -> Self {
        Self::Realtime(RealtimeSession::default())
    }
}

/// Realtime session object configuration.
/// openapi spec type: RealtimeSessionCreateRequestGA
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Additional fields to include in server outputs.
    ///
    /// `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    /// The default system instructions (i.e. system message) prepended to model calls.
    /// This field allows the client to guide the model on desired responses.
    /// The model can be instructed on response content and format,
    /// (e.g. "be extremely succinct", "act friendly", "here are examples of good responses")
    /// and on audio behavior (e.g. "talk quickly", "inject emotion into your voice",
    /// "laugh frequently"). The instructions are not guaranteed to be followed by the model, but
    /// they provide guidance to the model on the desired behavior.
    ///
    /// Note that the server sets default instructions which will be used if this field is not set
    /// and are visible in the `session.created` event at the start of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls. Provide an integer between 1 and 4096 to limit output tokens,
    /// or `inf` for the maximum available tokens for a given model. Defaults to `inf`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,

    /// The Realtime model used for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The set of modalities the model can respond with. It defaults to
    /// `["audio"]`, indicating that the model will respond with audio plus a transcript. `["text"]`
    /// can be used to make the model respond with text only. It is not possible to request both
    /// `text` and `audio` at the same time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,

    /// Reference to a prompt template and its variables.
    /// [Learn more](https://platform.openai.com/docs/guides/text?api-mode=responses#reusable-prompts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,

    /// How the model chooses tools. Provide one of the string modes or force a specific
    /// function/MCP tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Tools available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeTool>>,

    /// Realtime API can write session traces to the [Traces Dashboard](https://platform.openai.com/logs?api=traces).
    /// Set to null to disable tracing. Once tracing is enabled for a session, the configuration cannot be modified.
    ///
    /// `auto` will create a trace for the session with default values for the workflow name,
    ///  group id, and metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<Tracing>,

    /// When the number of tokens in a conversation exceeds the model's input token limit,
    /// the conversation be truncated, meaning messages (starting from the oldest) will not be
    /// included in the model's context. A 32k context model with 4,096 max output tokens can
    /// only include 28,224 tokens in the context before truncation occurs. Clients can configure
    /// truncation behavior to truncate with a lower max token limit, which is an effective way to
    /// control token usage and cost. Truncation will reduce the number of cached tokens on the next
    ///  turn (busting the cache), since messages are dropped from the beginning of the context.
    /// However, clients can also configure truncation to retain messages up to a fraction of the
    /// maximum context size, which will reduce the need for future truncations and thus improve
    /// the cache rate. Truncation can be disabled entirely, which means the server will never
    /// truncate but would instead return an error if the conversation exceeds the model's input
    /// token limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<RealtimeTruncation>,
}

/// Type of noise reduction. `near_field` is for close-talking microphones such as
/// headphones, `far_field` is for far-field microphones such as laptop or conference
/// room microphones.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NoiseReductionType {
    NearField,
    FarField,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionAudio {
    pub input: AudioInput,
}

/// Realtime transcription session object configuration.
/// openapi spec type: RealtimeTranscriptionSessionCreateRequestGA
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeTranscriptionSession {
    /// Configuration for input and output audio.
    pub audio: TranscriptionAudio,

    /// Additional fields to include in server outputs.
    ///
    /// `item.input_audio_transcription.logprobs`: Include logprobs for input audio transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}
