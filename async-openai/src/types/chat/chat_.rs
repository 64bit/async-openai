use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    error::OpenAIError,
    types::{
        chat::{
            CompletionTokensDetails, CustomGrammarFormatParam, FunctionCall, FunctionName,
            FunctionObject, ImageUrl, PromptTokensDetails, ReasoningEffort, ResponseFormat,
        },
        Metadata,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Prompt {
    String(String),
    StringArray(Vec<String>),
    // Minimum value is 0, maximum value is 4_294_967_295 (inclusive).
    IntegerArray(Vec<u32>),
    ArrayOfIntegerArray(Vec<Vec<u32>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StopConfiguration {
    String(String),           // nullable: true
    StringArray(Vec<String>), // minItems: 1; maxItems: 4
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Logprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<Option<f32>>, // Option is to account for null value in the list
    pub top_logprobs: Vec<serde_json::Value>,
    pub text_offset: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompletionFinishReason {
    Stop,
    Length,
    ContentFilter,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Logprobs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<CompletionFinishReason>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatCompletionFunctionCall {
    /// The model does not call a function, and responds to the end-user.
    #[serde(rename = "none")]
    None,
    /// The model can pick between an end-user or calling a function.
    #[serde(rename = "auto")]
    Auto,

    // In spec this is ChatCompletionFunctionCallOption
    // based on feedback from @m1guelpf in https://github.com/64bit/async-openai/pull/118
    // it is diverged from the spec
    /// Forces the model to call the specified function.
    #[serde(untagged)]
    Function { name: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
    Tool,
    Function,
}

/// Usage statistics for the completion request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct CompletionUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion.
    pub completion_tokens: u32,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u32,
    /// Breakdown of tokens used in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    /// Breakdown of tokens used in a completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestDeveloperMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestDeveloperMessage {
    /// The contents of the developer message.
    pub content: ChatCompletionRequestDeveloperMessageContent,

    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    Text(String),
    Array(Vec<ChatCompletionRequestDeveloperMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestDeveloperMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestSystemMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestSystemMessage {
    /// The contents of the system message.
    pub content: ChatCompletionRequestSystemMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartTextArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartText {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    /// The refusal message generated by the model.
    pub refusal: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartImageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartImage {
    pub image_url: ImageUrl,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputAudioFormat {
    Wav,
    #[default]
    Mp3,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct InputAudio {
    /// Base64 encoded audio data.
    pub data: String,
    /// The format of the encoded audio data. Currently supports "wav" and "mp3".
    pub format: InputAudioFormat,
}

/// Learn about [audio inputs](https://platform.openai.com/docs/guides/audio).
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartAudioArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartAudio {
    pub input_audio: InputAudio,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct FileObject {
    /// The base64 encoded file data, used when passing the file to the model
    /// as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_data: Option<String>,
    /// The ID of an uploaded file to use as input.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<String>,
    /// The name of the file, used when passing the file to the model as a
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFile {
    pub file: FileObject,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestUserMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    ImageUrl(ChatCompletionRequestMessageContentPartImage),
    InputAudio(ChatCompletionRequestMessageContentPartAudio),
    File(ChatCompletionRequestMessageContentPartFile),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestSystemMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Refusal(ChatCompletionRequestMessageContentPartRefusal),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRequestToolMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    /// The text contents of the system message.
    Text(String),
    /// An array of content parts with a defined type. For system messages, only type `text` is supported.
    Array(Vec<ChatCompletionRequestSystemMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type. Supported options differ based on the [model](https://platform.openai.com/docs/models) being used to generate the response. Can contain text, image, or audio inputs.
    Array(Vec<ChatCompletionRequestUserMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    /// The text contents of the message.
    Text(String),
    /// An array of content parts with a defined type. Can be one or more of type `text`, or exactly one of type `refusal`.
    Array(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    /// The text contents of the tool message.
    Text(String),
    /// An array of content parts with a defined type. For tool messages, only type `text` is supported.
    Array(Vec<ChatCompletionRequestToolMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestUserMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    pub content: ChatCompletionRequestUserMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageAudio {
    /// Unique identifier for a previous audio response from the model.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestAssistantMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestAssistantMessage {
    /// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ChatCompletionRequestAssistantMessageContent>,
    /// The refusal message by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Data about a previous audio response from the model.
    /// [Learn more](https://platform.openai.com/docs/guides/audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<ChatCompletionRequestAssistantMessageAudio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCalls>>,
    /// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

/// Tool message
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestToolMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestToolMessage {
    /// The contents of the tool message.
    pub content: ChatCompletionRequestToolMessageContent,
    pub tool_call_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestFunctionMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestFunctionMessage {
    /// The return value from the function call, to return to the model.
    pub content: Option<String>,
    /// The name of the function to call.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "role")]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionRequestMessage {
    Developer(ChatCompletionRequestDeveloperMessage),
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    Assistant(ChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionMessageToolCalls {
    Function(ChatCompletionMessageToolCall),
    Custom(ChatCompletionMessageCustomToolCall),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The function that the model called.
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionMessageCustomToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The custom tool that the model called.
    pub custom_tool: CustomTool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct CustomTool {
    /// The name of the custom tool to call.
    pub name: String,
    /// The input for the custom tool call generated by the model.
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionResponseMessageAudio {
    /// Unique identifier for this audio response.
    pub id: String,
    /// The Unix timestamp (in seconds) for when this audio response will no longer be accessible on the server for use in multi-turn conversations.
    pub expires_at: u64,
    /// Base64 encoded audio bytes generated by the model, in the format specified in the request.
    pub data: String,
    /// Transcript of the audio generated by the model.
    pub transcript: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatCompletionResponseMessageAnnotation {
    UrlCitation { url_citation: UrlCitation },
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct UrlCitation {
    /// The index of the last character of the URL citation in the message.
    pub end_index: u32,
    /// The index of the first character of the URL citation in the message.
    pub start_index: u32,
    /// The title of the web resource.
    pub title: String,
    /// The URL of the web resource.
    pub url: String,
}

/// A chat completion message generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessage {
    /// The contents of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The refusal message generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// The tool calls generated by the model, such as function calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCalls>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<ChatCompletionResponseMessageAnnotation>>,

    /// The role of the author of this message.
    pub role: Role,

    /// Deprecated and replaced by `tool_calls`.
    /// The name and arguments of a function that should be called, as generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub function_call: Option<FunctionCall>,

    /// If the audio output modality is requested, this object contains data about the audio response from the model. [Learn more](https://platform.openai.com/docs/guides/audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<ChatCompletionResponseMessageAudio>,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, Builder, PartialEq)]
#[builder(name = "ChatCompletionFunctionsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionFunctions {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the functions accepts, described as a JSON Schema object. See the [guide](https://platform.openai.com/docs/guides/text-generation/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting `parameters` defines a function with an empty parameter list.
    pub parameters: serde_json::Value,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatCompletionTools {
    /// A function tool that can be used to generate a response.
    Function(ChatCompletionTool),
    /// A custom tool that processes input using a specified format.
    Custom(CustomToolChatCompletions),
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionTool {
    pub function: FunctionObject,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct CustomToolChatCompletions {
    pub custom: CustomToolProperties,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct CustomToolProperties {
    /// The name of the custom tool, used to identify it in tool calls.
    pub name: String,

    /// Optional description of the custom tool, used to provide more context.
    pub description: Option<String>,

    /// The input format for the custom tool. Default is unconstrained text.
    pub format: CustomToolPropertiesFormat,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CustomToolPropertiesFormat {
    /// Unconstrained free-form text.
    #[default]
    Text,
    /// A grammar defined by the user.
    Grammar { grammar: CustomGrammarFormatParam },
}

/// Specifies a tool the model should use. Use to force the model to call a specific function.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionNamedToolChoice {
    pub function: FunctionName,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionNamedToolChoiceCustom {
    pub custom: CustomName,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct CustomName {
    /// The name of the custom tool to call.
    pub name: String,
}

/// Controls which (if any) tool is called by the model.
/// `none` means the model will not call any tool and instead generates a message.
/// `auto` means the model can pick between generating a message or calling one or more tools.
/// `required` means the model must call one or more tools.
/// Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
///
/// `none` is the default when no tools are present. `auto` is the default if tools are present.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatCompletionToolChoiceOption {
    AllowedTools(ChatCompletionAllowedToolsChoice),
    Function(ChatCompletionNamedToolChoice),
    Custom(ChatCompletionNamedToolChoiceCustom),

    #[serde(untagged)]
    Mode(ToolChoiceOptions),
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionAllowedToolsChoice {
    pub allowed_tools: Vec<ChatCompletionAllowedTools>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceAllowedMode {
    Auto,
    Required,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionAllowedTools {
    /// Constrains the tools available to the model to a pre-defined set.
    ///
    /// `auto` allows the model to pick from among the allowed tools and generate a
    /// message.
    ///
    /// `required` requires the model to call one or more of the allowed tools.
    pub mode: ToolChoiceAllowedMode,
    /// A list of tool definitions that the model should be allowed to call.
    ///
    /// For the Chat Completions API, the list of tool definitions might look like:
    /// ```json
    /// [
    ///   { "type": "function", "function": { "name": "get_weather" } },
    ///   { "type": "function", "function": { "name": "get_time" } }
    /// ]
    /// ```
    pub tools: Vec<serde_json::Value>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceOptions {
    #[default]
    None,
    Auto,
    Required,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
/// The amount of context window space to use for the search.
pub enum WebSearchContextSize {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchUserLocationType {
    Approximate,
}

/// Approximate location parameters for the search.
#[derive(Clone, Serialize, Debug, Default, Deserialize, PartialEq)]
pub struct WebSearchLocation {
    ///  The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g. `US`.
    pub country: Option<String>,
    /// Free text input for the region of the user, e.g. `California`.
    pub region: Option<String>,
    /// Free text input for the city of the user, e.g. `San Francisco`.
    pub city: Option<String>,
    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the user, e.g. `America/Los_Angeles`.
    pub timezone: Option<String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct WebSearchUserLocation {
    //  The type of location approximation. Always `approximate`.
    pub r#type: WebSearchUserLocationType,

    pub approximate: WebSearchLocation,
}

/// Options for the web search tool.
#[derive(Clone, Serialize, Debug, Default, Deserialize, PartialEq)]
pub struct WebSearchOptions {
    /// High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default.
    pub search_context_size: Option<WebSearchContextSize>,

    /// Approximate location parameters for the search.
    pub user_location: Option<WebSearchUserLocation>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

/// Constrains the verbosity of the model's response. Lower values will result in more concise responses, while higher values will result in more verbose responses. Currently supported values are `low`, `medium`, and `high`.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    #[default]
    Medium,
    High,
}

/// Output types that you would like the model to generate for this request.
///
/// Most models are capable of generating text, which is the default: `["text"]`
///
/// The `gpt-4o-audio-preview` model can also be used to [generate
/// audio](https://platform.openai.com/docs/guides/audio). To request that this model generate both text and audio responses, you can use: `["text", "audio"]`
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseModalities {
    Text,
    Audio,
}

/// The content that should be matched when generating a model response. If generated tokens would match this content, the entire model response can be returned much more quickly.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PredictionContentContent {
    /// The content used for a Predicted Output. This is often the text of a file you are regenerating with minor changes.
    Text(String),
    /// An array of content parts with a defined type. Supported options differ based on the [model](https://platform.openai.com/docs/models) being used to generate the response. Can contain text inputs.
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

/// Static predicted output content, such as the content of a text file that is being regenerated.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase", content = "content")]
pub enum PredictionContent {
    /// The type of the predicted content you want to provide. This type is
    /// currently always `content`.
    Content(PredictionContentContent),
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionAudioVoice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Nova,
    Onyx,
    Sage,
    Shimmer,
    #[serde(untagged)]
    Other(String),
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionAudioFormat {
    Wav,
    Aac,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionAudio {
    /// The voice the model uses to respond. Supported voices are
    /// `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`, `sage`, and `shimmer`.
    pub voice: ChatCompletionAudioVoice,
    /// Specifies the output audio format. Must be one of `wav`, `aac`, `mp3`, `flac`, `opus`, or `pcm16`.
    pub format: ChatCompletionAudioFormat,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "CreateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateChatCompletionRequest {
    /// A list of messages comprising the conversation so far. Depending on the
    /// [model](https://platform.openai.com/docs/models) you use, different message types (modalities)
    /// are supported, like [text](https://platform.openai.com/docs/guides/text-generation),
    /// [images](https://platform.openai.com/docs/guides/vision), and
    /// [audio](https://platform.openai.com/docs/guides/audio).
    pub messages: Vec<ChatCompletionRequestMessage>, // min: 1

    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI
    /// offers a wide range of models with different capabilities, performance
    /// characteristics, and price points. Refer to the
    /// [model guide](https://platform.openai.com/docs/models)
    /// to browse and compare available models.
    pub model: String,

    /// Output types that you would like the model to generate. Most models are capable of generating
    /// text, which is the default:
    ///
    /// `["text"]`
    /// The `gpt-4o-audio-preview` model can also be used to
    /// [generate audio](https://platform.openai.com/docs/guides/audio). To request that this model
    /// generate both text and audio responses, you can use:
    ///
    /// `["text", "audio"]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<ResponseModalities>>,

    /// Constrains the verbosity of the model's response. Lower values will result in
    /// more concise responses, while higher values will result in more verbose responses.
    /// Currently supported values are `low`, `medium`, and `high`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,

    /// Constrains effort on reasoning for
    /// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
    /// Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing
    /// reasoning effort can result in faster responses and fewer tokens used
    /// on reasoning in a response.
    /// Note: The `gpt-5-pro` model defaults to (and only supports) `high` reasoning effort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,

    /// An upper bound for the number of tokens that can be generated for a completion, including
    /// visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on
    /// their existing frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on
    /// whether they appear in the text so far, increasing the model's likelihood
    /// to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0

    /// This tool searches the web for relevant results to use in a response.
    /// Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search?api-mode=chat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<WebSearchOptions>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    /// `logprobs` must be set to `true` if this parameter is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// An object specifying the format that the model must output.
    ///
    /// Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
    /// Structured Outputs which ensures the model will match your supplied JSON
    /// schema. Learn more in the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    ///
    /// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
    /// ensures the message the model generates is valid JSON. Using `json_schema`
    /// is preferred for models that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Parameters for audio output. Required when audio output is requested with
    /// `modalities: ["audio"]`. [Learn more](https://platform.openai.com/docs/guides/audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<ChatCompletionAudio>,

    /// Whether or not to store the output of this chat completion request for
    /// use in our [model distillation](https://platform.openai.com/docs/guides/distillation) or
    /// [evals](https://platform.openai.com/docs/guides/evals) products.
    ///
    /// Supports text and image inputs. Note: image inputs over 8MB will be dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>, // nullable: true, default: false

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    /// See the [Streaming section below](https://platform.openai.com/docs/api-reference/chat/streaming)
    /// for more information, along with the [streaming responses](https://platform.openai.com/docs/guides/streaming-responses)
    /// guide for more information on how to handle the streaming events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Not supported with latest reasoning models `o3` and `o4-mini`.
    ///
    /// Up to 4 sequences where the API will stop generating further tokens. The
    /// returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopConfiguration>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i8>>, // default: null

    /// Whether to return log probabilities of the output tokens or not. If true,
    /// returns the log probabilities of each output token returned in the `content` of `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) that can be generated in
    /// the chat completion. This value can be used to control [costs](https://openai.com/api/pricing/) for text generated via API.
    /// This value is now deprecated in favor of `max_completion_tokens`, and is
    /// not compatible with [o-series models](https://platform.openai.com/docs/guides/reasoning).
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// How many chat completion choices to generate for each input message. Note that you will be
    /// charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to
    /// minimize costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1

    /// Configuration for a [Predicted Output](https://platform.openai.com/docs/guides/predicted-outputs),
    /// which can greatly improve response times when large parts of the model
    /// response are known ahead of time. This is most common when you are
    /// regenerating a file with only minor changes to most of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<PredictionContent>,

    /// This feature is in Beta.
    ///
    /// If specified, our system will make a best effort to sample deterministically, such that
    /// repeated requests with the same `seed` and parameters should return the same result.
    ///
    /// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response
    /// parameter to monitor changes in the backend.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatCompletionStreamOptions>,

    /// Specifies the processing type used for serving the request.
    /// - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.
    /// - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.
    /// - If set to '[flex](https://platform.openai.com/docs/guides/flex-processing)' or '[priority](https://openai.com/api-priority-processing/)', then the request will be processed with the corresponding service tier.
    /// - When not set, the default behavior is 'auto'.
    ///
    /// When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min: 0, max: 2, default: 1,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///  We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1

    /// A list of tools the model may call. You can provide either
    /// [custom tools](https://platform.openai.com/docs/guides/function-calling#custom-tools) or
    /// [function tools](https://platform.openai.com/docs/guides/function-calling).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTools>>,

    /// Controls which (if any) tool is called by the model.
    /// `none` means the model will not call any tool and instead generates a message.
    /// `auto` means the model can pick between generating a message or calling one or more tools.
    /// `required` means the model must call one or more tools.
    /// Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces
    /// the model to call that tool.
    /// `none` is the default when no tools are present. `auto` is the default if tools are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,

    /// Whether to enable [parallel function calling](https://platform.openai.com/docs/guides/function-calling#configuring-parallel-function-calling)
    /// during tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key`
    /// instead to maintain caching optimizations.
    /// A stable identifier for your end-users.
    /// Used to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and
    /// prevent abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// A stable identifier used to help detect users of your application that may be violating OpenAI's
    /// usage policies.
    ///
    /// The IDs should be a string that uniquely identifies each user. We recommend hashing their username
    /// or email address, in order to avoid sending us any identifying information. [Learn
    /// more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    /// Used by OpenAI to cache responses for similar requests to optimize your cache hit rates. Replaces
    /// the `user` field. [Learn more](https://platform.openai.com/docs/guides/prompt-caching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// Deprecated in favor of `tool_choice`.
    ///
    /// Controls which (if any) function is called by the model.
    /// `none` means the model will not call a function and instead generates a message.
    /// `auto` means the model can pick between generating a message or calling a function.
    /// Specifying a particular function via `{"name": "my_function"}` forces the model to call that function.
    ///
    /// `none` is the default when no functions are present. `auto` is the default if functions are present.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,

    /// Deprecated in favor of `tools`.
    ///
    /// A list of functions the model may generate JSON inputs for.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<ChatCompletionFunctions>>,

    ///  Developer-defined tags and values used for filtering completions in the [dashboard](https://platform.openai.com/chat-completions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>, // nullable: true
}

/// Options for streaming response. Only set this when you set `stream: true`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ChatCompletionStreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]`
    /// message. The `usage` field on this chunk shows the token usage statistics
    /// for the entire request, and the `choices` field will always be an empty
    /// array.
    ///
    /// All other chunks will also include a `usage` field, but with a null
    /// value. **NOTE:** If the stream is interrupted, you may not receive the
    /// final usage chunk which contains the total token usage for the request.
    pub include_usage: Option<bool>,

    /// When true, stream obfuscation will be enabled. Stream obfuscation adds
    /// random characters to an `obfuscation` field on streaming delta events to
    /// normalize payload sizes as a mitigation to certain side-channel attacks.
    /// These obfuscation fields are included by default, but add a small amount
    /// of overhead to the data stream. You can set `include_obfuscation` to
    /// false to optimize for bandwidth if you trust the network links between
    /// your application and the OpenAI API.
    pub include_obfuscation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    FunctionCall,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TopLogprobs {
    /// The token.
    pub token: String,
    /// The log probability of this token.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionTokenLogprob {
    /// The token.
    pub token: String,
    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
    pub logprob: f32,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
    ///  List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.
    pub top_logprobs: Vec<TopLogprobs>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoiceLogprobs {
    /// A list of message content tokens with log probability information.
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
    pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoice {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub message: ChatCompletionResponseMessage,
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    /// Log probability information for the choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatChoiceLogprobs>,
}

/// Represents a chat completion response returned by model, based on the provided input.
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if `n` is greater than 1.
    pub choices: Vec<ChatChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u32,
    /// The model used for the chat completion.
    pub model: String,
    /// The service tier used for processing the request. This field is only included if the `service_tier` parameter is specified in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: String,
    pub usage: Option<CompletionUsage>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
#[cfg(feature = "_api")]
pub type ChatCompletionResponseStream = std::pin::Pin<
    Box<dyn futures::Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>,
>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FunctionCallStream {
    /// The name of the function to call.
    pub name: Option<String>,
    /// The arguments to call the function with, as generated by the model in JSON format.
    /// Note that the model does not always generate valid JSON, and may hallucinate
    /// parameters not defined by your function schema. Validate the arguments in your
    /// code before calling your function.
    pub arguments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCallChunk {
    pub index: u32,
    /// The ID of the tool call.
    pub id: Option<String>,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: Option<FunctionType>,
    pub function: Option<FunctionCallStream>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionStreamResponseDelta {
    /// The contents of the chunk message.
    pub content: Option<String>,
    /// Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    pub function_call: Option<FunctionCallStream>,

    pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
    /// The role of the author of this message.
    pub role: Option<Role>,
    /// The refusal message generated by the model.
    pub refusal: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoiceStream {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub delta: ChatCompletionStreamResponseDelta,
    /// The reason the model stopped generating tokens. This will be
    /// `stop` if the model hit a natural stop point or a provided
    /// stop sequence,
    ///
    /// `length` if the maximum number of tokens specified in the
    /// request was reached,
    /// `content_filter` if content was omitted due to a flag from our
    /// content filters,
    /// `tool_calls` if the model called a tool, or `function_call`
    /// (deprecated) if the model called a function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    /// Log probability information for the choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatChoiceLogprobs>,
}

/// Represents a streamed chunk of a chat completion response returned by the model, based on the provided input. [Learn more](https://platform.openai.com/docs/guides/streaming-responses).
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: String,
    /// A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the last chunk if you set `stream_options: {"include_usage": true}`.
    pub choices: Vec<ChatChoiceStream>,

    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: u32,
    /// The model to generate the completion.
    pub model: String,
    /// The service tier used for processing the request. This field is only included if the `service_tier` parameter is specified in the request.
    pub service_tier: Option<ServiceTier>,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[deprecated]
    pub system_fingerprint: Option<String>,
    /// The object type, which is always `chat.completion.chunk`.
    pub object: String,

    /// An optional field that will only be present when you set `stream_options: {"include_usage": true}` in your request.
    /// When present, it contains a null value except for the last chunk which contains the token usage statistics for the entire request.
    pub usage: Option<CompletionUsage>,
}

/// An object representing a list of Chat Completions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionList {
    /// The type of this object. It is always set to "list".
    pub object: String,
    /// An array of chat completion objects.
    pub data: Vec<CreateChatCompletionResponse>,
    /// The identifier of the first chat completion in the data array.
    pub first_id: String,
    /// The identifier of the last chat completion in the data array.
    pub last_id: String,
    /// Indicates whether there are more Chat Completions available.
    pub has_more: bool,
}

/// Response when deleting a chat completion.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionDeleted {
    /// The type of object being deleted.
    pub object: String,
    /// The ID of the chat completion that was deleted.
    pub id: String,
    /// Whether the chat completion was deleted.
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    ImageUrl(ChatCompletionRequestMessageContentPartImage),
}

/// A chat completion message with additional fields for listing.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageListItem {
    /// The identifier of the chat message.
    pub id: String,
    /// If a content parts array was provided, this is an array of `text` and `image_url` parts. Otherwise, null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_parts: Option<Vec<ContentPart>>,

    #[serde(flatten)]
    pub message: ChatCompletionResponseMessage,
}

/// An object representing a list of chat completion messages.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageList {
    /// The type of this object. It is always set to "list".
    pub object: String,
    /// An array of chat completion message objects.
    pub data: Vec<ChatCompletionMessageListItem>,
    /// The identifier of the first chat message in the data array.
    pub first_id: String,
    /// The identifier of the last chat message in the data array.
    pub last_id: String,
    /// Indicates whether there are more chat messages available.
    pub has_more: bool,
}

/// Request to update a chat completion.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(name = "UpdateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateChatCompletionRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: Metadata,
}
