use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The moderation policy to use for request input or generated output.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModerationMode {
    Score,
    Block,
}

/// Configuration for one side of a moderated request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModerationConfigParam {
    pub mode: ModerationMode,
}

/// Policies applied independently to request input and generated output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ModerationPolicyParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ModerationConfigParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ModerationConfigParam>,
}

/// Configuration for moderated model responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModerationParam {
    /// The moderation model to use, for example `omni-moderation-latest`.
    pub model: String,
    /// Policies to apply to request input and generated output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ModerationPolicyParam>,
}

/// Input modality reflected in a moderation category score.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModerationInputType {
    Text,
    Image,
}

/// Successful moderation result object type.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModerationResultType {
    ModerationResult,
}

/// A successful moderation result for response input or output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModerationResultBody {
    #[serde(rename = "type")]
    pub type_: ModerationResultType,
    pub model: String,
    pub flagged: bool,
    pub categories: HashMap<String, bool>,
    pub category_scores: HashMap<String, f64>,
    pub category_applied_input_types: HashMap<String, Vec<ModerationInputType>>,
}

/// Moderation failure object type.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModerationErrorType {
    Error,
}

/// An error produced while attempting moderation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModerationErrorBody {
    #[serde(rename = "type")]
    pub type_: ModerationErrorType,
    pub code: String,
    pub message: String,
}

/// A successful moderation result or moderation error.
#[cfg(feature = "response-types")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationOutcome {
    Result(ModerationResultBody),
    Error(ModerationErrorBody),
}

/// Moderation results for the input and output of a Responses API request.
#[cfg(feature = "response-types")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Moderation {
    pub input: ModerationOutcome,
    pub output: ModerationOutcome,
}

/// Successful Chat Completions moderation result object type.
#[cfg(feature = "chat-completion-types")]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionModerationResultsType {
    ModerationResults,
}

/// Successful moderation results for Chat Completions input or output.
#[cfg(feature = "chat-completion-types")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionModerationResults {
    #[serde(rename = "type")]
    pub type_: ChatCompletionModerationResultsType,
    pub model: String,
    pub results: Vec<ModerationResultBody>,
}

/// An error produced while moderating a Chat Completions request.
#[cfg(feature = "chat-completion-types")]
pub type ChatCompletionModerationError = ModerationErrorBody;

/// Successful Chat Completions moderation results or a moderation error.
#[cfg(feature = "chat-completion-types")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionModerationOutcome {
    Results(ChatCompletionModerationResults),
    Error(ChatCompletionModerationError),
}

/// Moderation results for the input and output of a Chat Completions request.
#[cfg(feature = "chat-completion-types")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionModeration {
    pub input: ChatCompletionModerationOutcome,
    pub output: ChatCompletionModerationOutcome,
}
