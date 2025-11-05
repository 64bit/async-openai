use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{
    realtime::{
        MaxOutputTokens, RealtimeAudioFormats, RealtimeConversationItem, RealtimeTool,
        RealtimeVoice, ToolChoice,
    },
    responses::Prompt,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeResponseUsage {
    /// Details about the input tokens used in the Response. Cached tokens are tokens from previous
    /// turns in the conversation that are included as context for the current response. Cached tokens
    /// here are counted as a subset of input tokens, meaning input tokens will include cached and
    /// uncached tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<InputTokenDetails>,

    /// The number of input tokens used in the Response, including text and audio tokens.
    pub input_tokens: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_token_details: Option<OutputTokenDetails>,

    /// The number of output tokens sent in the Response, including text and audio tokens.
    pub output_tokens: u32,

    /// The total number of tokens in the Response including input and output text and audio tokens.
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputTokenDetails {
    /// The number of audio tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
    /// The number of cached tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,

    /// Details about the cached tokens used as input for the Response.
    pub cached_token_details: Option<CachedTokenDetails>,

    /// The number of image tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<u32>,

    /// The number of text tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedTokenDetails {
    /// The number of cached audio tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,

    /// The number of cached image tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<u32>,

    /// The number of cached text tokens used as input for the Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputTokenDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub code: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeResponseStatusDetailType {
    Completed,
    Cancelled,
    Incomplete,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeResponseStatusDetailReason {
    TurnDetected,
    ClientCancelled,
    MaxOutputTokens,
    ContentFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeResponseStatusDetail {
    /// A description of the error that caused the response to fail, populated when the status is failed.
    pub error: Option<Error>,
    /// The reason the Response did not complete. For a `cancelled` Response, one of `turn_detected`
    /// (the server VAD detected a new start of speech) or `client_cancelled` (the client sent a cancel
    /// event). For an incomplete Response, one of `max_output_tokens` or `content_filter` (the
    ///  server-side safety filter activated and cut off the response).
    pub reason: Option<RealtimeResponseStatusDetailReason>,
    /// The type of error that caused the response to fail, corresponding with the `status`
    /// field (`completed`, `cancelled`, `incomplete`, `failed`).
    pub r#type: RealtimeResponseStatusDetailType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseAudioOutput {
    /// The format of the output audio.
    pub format: RealtimeAudioFormats,

    /// The voice the model uses to respond. Voice cannot be changed during the session once
    /// the model has responded with audio at least once. Current voice options are
    /// `alloy`, `ash`, `ballad`, `coral`, `echo`, `sage`, `shimmer`, `verse`, `marin`, and `cedar`.
    /// We recommend `marin` and `cedar` for best quality.
    pub voice: RealtimeVoice,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseAudio {
    /// Configuration for audio output.
    pub output: ResponseAudioOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Conversation {
    #[default]
    Auto,
    None,
}

/// The response resource.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeResponseCreateParams {
    /// Configuration for audio input and output.
    pub audio: ResponseAudio,

    /// Controls which conversation the response is added to. Currently supports auto and none,
    /// with auto as the default value. The auto value means that the contents of the response
    /// will be added to the default conversation. Set this to none to create an out-of-band
    /// response which will not add items to default conversation.
    pub conversation: Conversation,

    /// Input items to include in the prompt for the model. Using this field creates a new context
    /// for this Response instead of using the default conversation. An empty array `[]` will clear
    /// the context for this Response. Note that this can include references to items that
    /// previously appeared in the session using their id.
    pub input: Vec<RealtimeConversationItem>,

    /// The default system instructions (i.e. system message) prepended to model calls.
    /// This field allows the client to guide the model on desired responses.
    /// The model can be instructed on response content and format, (e.g. "be extremely succinct",
    /// "act friendly", "here are examples of good responses") and on audio behavior
    /// (e.g. "talk quickly", "inject emotion into your voice", "laugh frequently").
    /// The instructions are not guaranteed to be followed by the model, but they provide
    /// guidance to the model on the desired behavior. Note that the server sets default
    /// instructions which will be used if this field is not set and are visible in
    /// the `session.created` event at the start of the session.
    pub instructions: String,

    /// Maximum number of output tokens for a single assistant response, inclusive of tool calls.
    /// Provide an integer between 1 and 4096 to limit output tokens, or inf for the maximum
    /// available tokens for a given model. Defaults to `inf`.
    pub max_output_tokens: MaxOutputTokens,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for
    /// storing additional information about the object in a structured format, and querying
    /// for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are strings with a
    /// maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// The set of modalities the model used to respond, currently the only possible values
    /// are [\"audio\"], [\"text\"]. Audio output always include a text transcript.
    /// Setting the output to mode `text` will disable audio output from the model.
    pub output_modalities: Vec<String>,

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeResponse {
    /// Configuration for audio output.
    pub audio: Option<ResponseAudio>,

    /// Which conversation the response is added to, determined by the `conversation` field in the
    /// `response.create` event. If `auto`, the response will be added to the default conversation
    /// and the value of `conversation_id` will be an id like `conv_1234`. If `none`, the response
    /// will not be added to any conversation and the value of `conversation_id` will be `null`.
    /// If responses are being triggered automatically by VAD the response will be added to the
    /// default conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    /// The unique ID of the response, will look like `resp_1234`.
    pub id: String,

    /// Maximum number of output tokens for a single assistant response, inclusive of tool calls,
    /// that was used in this response.
    pub max_output_tokens: MaxOutputTokens,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for
    /// storing additional information about the object in a structured format, and querying
    /// for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are strings with a
    /// maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// The object type, must be "realtime.response".
    pub object: String,

    /// The list of output items generated by the response.
    pub output: Vec<RealtimeConversationItem>,

    /// The set of modalities the model used to respond, currently the only possible values
    /// are [\"audio\"], [\"text\"]. Audio output always include a text transcript.
    /// Setting the output to mode `text` will disable audio output from the model.
    pub output_modalities: Vec<String>,

    /// The final status of the response (`completed`, `cancelled`, `failed`, or `incomplete`, `in_progress`).
    pub status: RealtimeResponseStatus,

    /// Additional details about the status.
    pub status_details: Option<RealtimeResponseStatusDetail>,

    /// Usage statistics for the Response, this will correspond to billing. A Realtime API session
    /// will maintain a conversation context and append new Items to the Conversation, thus output
    /// from previous turns (text and audio tokens) will become the input for later turns.
    pub usage: Option<RealtimeResponseUsage>,
}
