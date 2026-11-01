use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub use crate::types::shared::{
    ImageDetail as ImageDetailParam, MessagePhase as MessagePhaseParam,
};
pub use crate::types::shared::{InputTokenDetails, OutputTokenDetails, ResponseUsage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionResponse {
    /// The model used.
    pub model: String,
    /// Answers in question order. Any question can return a refusal instead of an answer; other questions can still receive answers. A refusal has `type: "refusal"` and the question's `name`, or `null` if it was unnamed.
    pub answers: Vec<AnswerResource>,
    /// Token usage for the request.
    pub usage: ResponseUsage,
}

/// A completed question always includes its name, including null when unnamed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnswerResource {
    Predicate(AnswerResourcePredicate),
    Choice(AnswerResourceChoice),
    Score(AnswerResourceScore),
    Refusal(AnswerResourceRefusal),
}

/// The model declined to answer this question. Other questions in the same request can still receive answers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AnswerResourceRefusal {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AnswerResourceScore {
    pub name: Option<String>,
    pub score: f64,
    pub probabilities: Vec<ScoreProbabilityResource>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreProbabilityResource {
    pub value: i64,
    pub label: String,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnswerResourceChoice {
    pub name: Option<String>,
    pub choice: ChoiceValueResource,
    pub probabilities: Vec<ChoiceProbabilityResource>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceProbabilityResource {
    pub value: ChoiceValueResource,
    pub probability: f64,
}

/// Choice values are typed: a string and a boolean with the same text are distinct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChoiceValueResource {
    Text(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AnswerResourcePredicate {
    pub name: Option<String>,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Builder)]
#[builder(
    name = "DecisionRequestArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
// Required input has no schema default; optional builder fields store Option directly.
pub struct DecisionRequest {
    /// The model to use. Only `gpt-6-luna` is supported.
    pub model: String,
    pub input: DecisionInput,
    /// Ask 1–400 questions about the input. Each question can be a predicate, choice, or score. Answers come back in the same order.
    pub questions: Vec<QuestionParam>,
    /// Opaque caller-provided end-user identifier, scoped by the verified org. Match Responses' limit; this is never the authenticated user identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// Partition prompt cache reuse by caller-provided key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Options for prompt caching. Supported for `gpt-5.6` and later models. By default, OpenAI automatically chooses one implicit cache breakpoint. You can add explicit breakpoints to content blocks with `prompt_cache_breakpoint`. Each request can write up to four breakpoints. For cache matching, OpenAI considers up to the latest 80 breakpoints in the conversation, without a content-block lookback limit. Set `mode` to `explicit` to disable the implicit breakpoint. The `ttl` defaults to `30m`, which is currently the only supported value. See the [prompt caching guide](https://developers.openai.com/api/docs/guides/prompt-caching) for current details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<DecisionRequestPromptCacheOptions>,
}

/// Options for prompt caching. Supported for `gpt-5.6` and later models. By default, OpenAI automatically chooses one implicit cache breakpoint. You can add explicit breakpoints to content blocks with `prompt_cache_breakpoint`. Each request can write up to four breakpoints. For cache matching, OpenAI considers up to the latest 80 breakpoints in the conversation, without a content-block lookback limit. Set `mode` to `explicit` to disable the implicit breakpoint. The `ttl` defaults to `30m`, which is currently the only supported value. See the [prompt caching guide](https://developers.openai.com/api/docs/guides/prompt-caching) for current details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionRequestPromptCacheOptions {
    /// The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries for longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<DecisionRequestPromptCacheOptionsTtl>,
    /// Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`. With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request does not use prompt caching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DecisionRequestPromptCacheOptionsMode>,
}

/// Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`. With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request does not use prompt caching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionRequestPromptCacheOptionsMode {
    Implicit,
    Explicit,
}

/// The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries for longer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DecisionRequestPromptCacheOptionsTtl {
    #[serde(rename = "30m")]
    #[default]
    Value30m,
}

/// A question about the request's input, with an optional correlation name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuestionParam {
    Predicate(QuestionParamPredicate),
    Choice(QuestionParamChoice),
    Score(QuestionParamScore),
}

/// Rate the input against the supplied ordered levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionParamScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: QuestionParamInstructions,
    pub levels: Vec<ScoreLevelParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreLevelParam {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Instructions for a predicate, choice, or score question, as text or an ordered list of content parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QuestionParamInstructions {
    Text(String),
    Items(Vec<DecisionQuestionInputPart>),
}

/// A text, image, or JSON content part in a question instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionQuestionInputPart {
    InputText(DecisionQuestionInputText),
    InputImage(DecisionQuestionInputImage),
    InputJson(DecisionInputJson),
}

/// A JSON object or an array of objects rendered as text. Nested values may be any JSON value. Limited to 32 container levels, 10,000 values, and 256 KiB of compact UTF-8 JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputJson {
    pub data: DecisionInputJsonData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputJsonData {
    Item(serde_json::Value),
    Items(Vec<serde_json::Value>),
}

/// An image provided as a base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionQuestionInputImage {
    /// A base64-encoded image in a data URL or a publicly accessible HTTP(S) image URL.
    pub image_url: String,
    /// The image detail level, using the selected model's image profile. Defaults to auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetailParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionQuestionInputText {
    pub text: String,
}

/// Choose from the supplied options based on the input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionParamChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: QuestionParamInstructions,
    /// Provide between 2 and 255 choices. Each choice must be unique.
    pub choices: Vec<ChoiceOptionParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceOptionParam {
    pub value: ChoiceValueParam,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Choice values are typed: a string and a boolean with the same text are distinct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChoiceValueParam {
    Text(String),
    Boolean(bool),
}

/// Estimate how likely it is that a statement about the input is true.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionParamPredicate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: QuestionParamInstructions,
}

/// The input to evaluate for every question. Provide a text string or an ordered array of messages, function calls and outputs, or custom tool calls and outputs. Each tool output must identify a matching call by call ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInput {
    Text(String),
    Items(Vec<DecisionInputItem>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputItem {
    Message(DecisionInputMessage),
    FunctionCall(DecisionInputFunctionCall),
    FunctionCallOutput(DecisionInputFunctionCallOutput),
    CustomToolCall(DecisionInputCustomToolCall),
    CustomToolCallOutput(DecisionInputCustomToolCallOutput),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputCustomToolCallOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    pub output: DecisionInputToolOutputContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatusParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallStatusParam {
    InProgress,
    Completed,
    Incomplete,
}

/// Text evidence or an ordered list of text and image parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputToolOutputContent {
    Text(String),
    DecisionInputToolOutputContentParts(DecisionInputToolOutputContentParts),
}

pub type DecisionInputToolOutputContentParts = Vec<DecisionInputToolOutputContentPart>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputToolOutputContentPart {
    InputText(DecisionInputText),
    InputImage(DecisionInputImage),
}

/// An image provided as a base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputImage {
    /// A base64-encoded image in a data URL or a publicly accessible HTTP(S) image URL.
    pub image_url: String,
    /// The image detail level, using the selected model's image profile. Defaults to auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetailParam>,
    /// Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
}

/// Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PromptCacheBreakpoint {
    /// The breakpoint mode. Always `explicit`.
    pub mode: PromptCacheBreakpointMode,
}

/// The breakpoint mode. Always `explicit`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PromptCacheBreakpointMode {
    #[default]
    Explicit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputText {
    pub text: String,
    /// Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputCustomToolCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub name: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatusParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputFunctionCallOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    pub output: DecisionInputToolOutputContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatusParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputFunctionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub name: String,
    pub arguments: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatusParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum DecisionInputMessage {
    User(DecisionInputUserMessage),
    Developer(DecisionInputDeveloperMessage),
    System(DecisionInputSystemMessage),
    Assistant(DecisionInputAssistantMessage),
}

/// An assistant message containing output text or refusals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputAssistantMessage {
    pub content: DecisionInputAssistantContent,
    /// The phase of an assistant message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<MessagePhaseParam>,
}

/// A text string or an ordered list of output text and refusal parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputAssistantContent {
    Text(String),
    DecisionInputAssistantContentParts(DecisionInputAssistantContentParts),
}

pub type DecisionInputAssistantContentParts = Vec<DecisionInputAssistantContentPart>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputAssistantContentPart {
    OutputText(DecisionInputOutputText),
    Refusal(DecisionInputRefusal),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputRefusal {
    pub refusal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DecisionInputOutputText {
    pub text: String,
}

/// A system message containing input text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputSystemMessage {
    pub content: DecisionInputDeveloperContent,
}

/// A text string or an ordered list of input text parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputDeveloperContent {
    Text(String),
    DecisionInputDeveloperContentParts(DecisionInputDeveloperContentParts),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputDeveloperContentPart {
    InputText(DecisionInputText),
}
pub type DecisionInputDeveloperContentParts = Vec<DecisionInputDeveloperContentPart>;

/// A developer message containing input text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputDeveloperMessage {
    pub content: DecisionInputDeveloperContent,
}

/// A user message containing input text, images, or JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputUserMessage {
    pub content: DecisionInputContent,
}

/// A text string or an ordered list of text, image, and JSON parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputContent {
    Text(String),
    DecisionInputContentParts(DecisionInputContentParts),
}

pub type DecisionInputContentParts = Vec<DecisionInputContentPart>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputContentPart {
    InputText(DecisionInputText),
    InputImage(DecisionInputImage),
    InputJson(DecisionInputJson),
}

impl From<String> for DecisionInput {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<&str> for DecisionInput {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl Default for DecisionInput {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}
