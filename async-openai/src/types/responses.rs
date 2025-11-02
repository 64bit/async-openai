use crate::error::OpenAIError;
pub use crate::types::{
    CompletionTokensDetails, ImageDetail, PromptTokensDetails, ReasoningEffort,
    ResponseFormatJsonSchema,
};
use derive_builder::Builder;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;

/// Role of messages in the API.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Developer,
}

/// Status of input/output items.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutputStatus {
    InProgress,
    Completed,
    Incomplete,
}

/// Input payload: raw text or structured context items.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Input {
    /// A text input to the model, equivalent to a text input with the user role.
    Text(String),
    /// A list of one or many input items to the model, containing different content types.
    Items(Vec<InputItem>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum InputItem {
    Message(InputMessage),
    Custom(serde_json::Value),
}

/// A message to prime the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "InputMessageArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InputMessage {
    #[serde(default, rename = "type")]
    pub kind: InputMessageType,
    /// The role of the message input.
    pub role: Role,
    /// Text, image, or audio input to the model, used to generate a response. Can also contain
    /// previous assistant responses.
    pub content: InputContent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum InputMessageType {
    #[default]
    Message,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum InputContent {
    /// A text input to the model.
    TextInput(String),
    /// A list of one or many input items to the model, containing different content types.
    InputItemContentList(Vec<ContentType>),
}

/// Parts of a message: text, image, file, or audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentType {
    /// A text input to the model.
    InputText(InputText),
    /// An image input to the model.
    InputImage(InputImage),
    /// A file input to the model.
    InputFile(InputFile),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputText {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "InputImageArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InputImage {
    /// The detail level of the image to be sent to the model.
    detail: ImageDetail,
    /// The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<String>,
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image
    /// in a data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "InputFileArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InputFile {
    /// The content of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_data: Option<String>,
    /// The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<String>,
    /// The name of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    /// The URL of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Conversation {
    /// The unique ID of the conversation.
    pub id: String,
}

/// Builder for a Responses API request.
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder, PartialEq)]
#[builder(
    name = "CreateResponseArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateResponse {
    /// Text, image, or file inputs to the model, used to generate a response.
    pub input: Input,

    /// Model ID used to generate the response, like `gpt-4o`.
    /// OpenAI offers a wide range of models with different capabilities,
    /// performance characteristics, and price points.
    pub model: String,

    /// Whether to run the model response in the background.
    /// boolean or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Specify additional output data to include in the model response.
    ///
    /// Supported values:
    /// - `file_search_call.results`
    ///   Include the search results of the file search tool call.
    /// - `message.input_image.image_url`
    ///   Include image URLs from the input message.
    /// - `computer_call_output.output.image_url`
    ///   Include image URLs from the computer call output.
    /// - `reasoning.encrypted_content`
    ///   Include an encrypted version of reasoning tokens in reasoning item outputs.
    ///   This enables reasoning items to be used in multi-turn conversations when
    ///   using the Responses API statelessly (for example, when the `store` parameter
    ///   is set to `false`, or when an organization is enrolled in the zero-data-
    ///   retention program).
    ///
    /// If `None`, no additional data is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    /// Inserts a system (or developer) message as the first item in the model's context.
    ///
    /// When using along with previous_response_id, the instructions from a previous response will
    /// not be carried over to the next response. This makes it simple to swap out system
    /// (or developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// An upper bound for the number of tokens that can be generated for a
    /// response, including visible output tokens and reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// The maximum number of total calls to built-in tools that can be processed in a response.
    /// This maximum number applies across all built-in tool calls, not per individual tool.
    /// Any further attempts to call a tool by the model will be ignored.
    pub max_tool_calls: Option<u32>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be
    /// useful for storing additional information about the object in a structured
    /// format, and querying for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are
    /// strings with a maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// The unique ID of the previous response to the model. Use this to create
    /// multi-turn conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Reference to a prompt template and its variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptConfig>,

    /// **o-series models only**: Configuration options for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Specifies the latency tier to use for processing the request.
    ///
    /// This parameter is relevant for customers subscribed to the Scale tier service.
    ///
    /// Supported values:
    /// - `auto`
    ///   - If the Project is Scale tier enabled, the system will utilize Scale tier credits until
    ///     they are exhausted.
    ///   - If the Project is not Scale tier enabled, the request will be processed using the
    ///     default service tier with a lower uptime SLA and no latency guarantee.
    /// - `default`
    ///   The request will be processed using the default service tier with a lower uptime SLA and
    ///   no latency guarantee.
    /// - `flex`
    ///   The request will be processed with the Flex Processing service tier. Learn more.
    ///
    /// When not set, the default behavior is `auto`.
    ///
    /// When this parameter is set, the response body will include the `service_tier` utilized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Whether to store the generated model response for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client as it is
    /// generated using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8
    /// will make the output more random, while lower values like 0.2 will make it
    /// more focused and deterministic. We generally recommend altering this or
    /// `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Configuration options for a text response from the model. Can be plain text
    /// or structured JSON data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,

    /// How the model should select which tool (or tools) to use when generating
    /// a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// An array of tools the model may call while generating a response.
    /// Can include built-in tools (file_search, web_search_preview,
    /// computer_use_preview) or custom function definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return
    /// at each token position, each with an associated log probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>, // TODO add validation of range

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability
    /// mass. So 0.1 means only the tokens comprising the top 10% probability mass
    /// are considered. We generally recommend altering this or `temperature` but
    /// not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// The truncation strategy to use for the model response:
    /// - `auto`: drop items in the middle to fit context window.
    /// - `disabled`: error if exceeding context window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Service tier request options.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PromptConfig {
    /// The unique identifier of the prompt template to use.
    pub id: String,

    /// Optional version of the prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Optional map of values to substitute in for variables in your prompt. The substitution
    /// values can either be strings, or other Response input types like images or files.
    /// For now only supporting Strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
}

/// Service tier request options.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

/// Truncation strategies.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Truncation {
    Auto,
    Disabled,
}

/// o-series reasoning settings.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "ReasoningConfigArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ReasoningConfig {
    /// Constrain effort on reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// Summary mode for reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummary>,
}

/// o-series reasoning settings.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}

/// Configuration for text response format.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextConfig {
    /// Defines the format: plain text, JSON object, or JSON schema.
    pub format: TextResponseFormat,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextResponseFormat {
    /// The type of response format being defined: `text`
    Text,
    /// The type of response format being defined: `json_object`
    JsonObject,
    /// The type of response format being defined: `json_schema`
    JsonSchema(ResponseFormatJsonSchema),
}

/// Definitions for model-callable tools.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    /// File search tool.
    FileSearch(FileSearch),
    /// Custom function call.
    Function(Function),
    /// Web search preview tool.
    WebSearchPreview(WebSearchPreview),
    /// Virtual computer control tool.
    ComputerUsePreview(ComputerUsePreview),
    /// Remote Model Context Protocol server.
    Mcp(Mcp),
    /// Python code interpreter tool.
    CodeInterpreter(CodeInterpreter),
    /// Image generation tool.
    ImageGeneration(ImageGeneration),
    /// Local shell command execution tool.
    LocalShell,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "FileSearchArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct FileSearch {
    /// The IDs of the vector stores to search.
    pub vector_store_ids: Vec<String>,
    /// The maximum number of results to return. This number should be between 1 and 50 inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<u32>,
    /// A filter to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filter>,
    /// Ranking options for search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_options: Option<RankingOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "FunctionArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct Function {
    /// The name of the function to call.
    pub name: String,
    /// A JSON schema object describing the parameters of the function.
    pub parameters: serde_json::Value,
    /// Whether to enforce strict parameter validation.
    pub strict: bool,
    /// A description of the function. Used by the model to determine whether or not to call the
    /// function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "WebSearchPreviewArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct WebSearchPreview {
    /// The user's location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<Location>,
    /// High level guidance for the amount of context window space to use for the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<WebSearchContextSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchContextSize {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "ComputerUsePreviewArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct ComputerUsePreview {
    /// The type of computer environment to control.
    environment: String,
    /// The width of the computer display.
    display_width: u32,
    /// The height of the computer display.
    display_height: u32,
}

/// Options for search result ranking.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RankingOptions {
    /// The ranker to use for the file search.
    pub ranker: String,
    /// The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will
    /// attempt to return only the most relevant results, but may return fewer results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
}

/// Filters for file search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Filter {
    /// A filter used to compare a specified attribute key to a given value using a defined
    /// comparison operation.
    Comparison(ComparisonFilter),
    /// Combine multiple filters using and or or.
    Compound(CompoundFilter),
}

/// Single comparison filter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComparisonFilter {
    /// Specifies the comparison operator
    #[serde(rename = "type")]
    pub op: ComparisonType,
    /// The key to compare against the value.
    pub key: String,
    /// The value to compare against the attribute key; supports string, number, or boolean types.
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ComparisonType {
    #[serde(rename = "eq")]
    Equals,
    #[serde(rename = "ne")]
    NotEquals,
    #[serde(rename = "gt")]
    GreaterThan,
    #[serde(rename = "gte")]
    GreaterThanOrEqualTo,
    #[serde(rename = "lt")]
    LessThan,
    #[serde(rename = "lte")]
    LessThanOrEqualTo,
}

/// Combine multiple filters.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompoundFilter {
    /// Type of operation
    #[serde(rename = "type")]
    pub op: CompoundType,
    /// Array of filters to combine. Items can be ComparisonFilter or CompoundFilter.
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompoundType {
    And,
    Or,
}

/// Approximate user location for web search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "LocationArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct Location {
    /// The type of location approximation. Always approximate.
    #[serde(rename = "type")]
    pub kind: String,
    /// Free text input for the city of the user, e.g. San Francisco.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter ISO country code of the user, e.g. US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Free text input for the region of the user, e.g. California.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The IANA timezone of the user, e.g. America/Los_Angeles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// MCP (Model Context Protocol) tool configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "McpArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct Mcp {
    /// A label for this MCP server.
    pub server_label: String,
    /// The URL for the MCP server.
    pub server_url: String,
    /// List of allowed tool names or filter object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<AllowedTools>,
    /// Optional HTTP headers for the MCP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,
    /// Approval policy or filter for tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<RequireApproval>,
}

/// Allowed tools configuration for MCP.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum AllowedTools {
    /// A flat list of allowed tool names.
    List(Vec<String>),
    /// A filter object specifying allowed tools.
    Filter(McpAllowedToolsFilter),
}

/// Filter object for MCP allowed tools.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct McpAllowedToolsFilter {
    /// Names of tools in the filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// Approval policy or filter for MCP tools.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RequireApproval {
    /// A blanket policy: "always" or "never".
    Policy(RequireApprovalPolicy),
    /// A filter object specifying which tools require approval.
    Filter(McpApprovalFilter),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequireApprovalPolicy {
    Always,
    Never,
}

/// Filter object for MCP tool approval.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct McpApprovalFilter {
    /// A list of tools that always require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always: Option<McpAllowedToolsFilter>,
    /// A list of tools that never require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never: Option<McpAllowedToolsFilter>,
}

/// Container configuration for a code interpreter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CodeInterpreterContainer {
    /// A simple container ID.
    Id(String),
    /// Auto-configured container with optional files.
    Container(CodeInterpreterContainerKind),
}

/// Auto configuration for code interpreter container.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeInterpreterContainerKind {
    Auto {
        /// Optional list of uploaded file IDs.
        #[serde(skip_serializing_if = "Option::is_none")]
        file_ids: Option<Vec<String>>,
    },
}

/// Code interpreter tool definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "CodeInterpreterArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CodeInterpreter {
    /// Container configuration for running code.
    pub container: CodeInterpreterContainer,
}

/// Mask image input for image generation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImageMask {
    /// Base64-encoded mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// File ID for the mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

/// Image generation tool definition.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "ImageGenerationArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ImageGeneration {
    /// Background type: transparent, opaque, or auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageGenerationBackground>,
    /// Optional mask for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_mask: Option<InputImageMask>,
    /// Model to use (default: gpt-image-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Moderation level (default: auto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<String>,
    /// Compression level (0-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,
    /// Output format: png, webp, or jpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageGenerationOutputFormat>,
    /// Number of partial images (0-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u8>,
    /// Quality: low, medium, high, or auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageGenerationQuality>,
    /// Size: e.g. "1024x1024" or auto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageGenerationSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationBackground {
    Transparent,
    Opaque,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationOutputFormat {
    Png,
    Webp,
    Jpeg,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationQuality {
    Low,
    Medium,
    High,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationSize {
    Auto,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1024x1536")]
    Size1024x1536,
    #[serde(rename = "1536x1024")]
    Size1536x1024,
}

/// Control how the model picks or is forced to pick a tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Controls which (if any) tool is called by the model.
    Mode(ToolChoiceMode),
    /// Indicates that the model should use a built-in tool to generate a response.
    Hosted {
        /// The type of hosted tool the model should to use.
        #[serde(rename = "type")]
        kind: HostedToolType,
    },
    /// Use this option to force the model to call a specific function.
    Function {
        /// The name of the function to call.
        name: String,
    },
}

/// Simple tool-choice modes.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    /// The model will not call any tool and instead generates a message.
    None,
    /// The model can pick between generating a message or calling one or more tools.
    Auto,
    /// The model must call one or more tools.
    Required,
}

/// Hosted tool type identifiers.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HostedToolType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

/// Error returned by the API when a request fails.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorObject {
    /// The error code for the response.
    pub code: String,
    /// A human-readable description of the error.
    pub message: String,
}

/// Details about an incomplete response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IncompleteDetails {
    /// The reason why the response is incomplete.
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TopLogProb {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogProb {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
    pub top_logprobs: Vec<TopLogProb>,
}

/// A simple text output from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTextContent {
    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,
    pub logprobs: Option<LogProb>,
    /// The text output from the model.
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    /// A citation to a file.
    FileCitation(FileCitationBody),
    /// A citation for a web resource used to generate a model response.
    UrlCitation(UrlCitationBody),
    /// A citation for a container file used to generate a model response.
    ContainerFileCitation(ContainerFileCitationBody),
    /// A path to a file.
    FilePath(FilePath),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileCitationBody {
    /// The ID of the file.
    file_id: String,
    /// The filename of the file cited.
    filename: String,
    /// The index of the file in the list of files.
    index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UrlCitationBody {
    /// The index of the last character of the URL citation in the message.
    end_index: u32,
    /// The index of the first character of the URL citation in the message.
    start_index: u32,
    /// The title of the web resource.
    title: String,
    /// The URL of the web resource.
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContainerFileCitationBody {
    /// The ID of the container file.
    container_id: String,
    /// The index of the last character of the container file citation in the message.
    end_index: u32,
    /// The ID of the file.
    file_id: String,
    /// The filename of the container file cited.
    filename: String,
    /// The index of the first character of the container file citation in the message.
    start_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FilePath {
    /// The ID of the file.
    file_id: String,
    /// The index of the file in the list of files.
    index: u32,
}

/// A refusal explanation from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RefusalContent {
    /// The refusal explanation from the model.
    pub refusal: String,
}

/// A message generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputMessage {
    /// The content of the output message.
    pub content: Vec<OutputMessageContent>,
    /// The unique ID of the output message.
    pub id: String,
    /// The role of the output message. Always `assistant`.
    pub role: Role,
    /// The status of the message input. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when input items are returned via API.
    pub status: OutputStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputMessageContent {
    /// A text output from the model.
    OutputText(OutputTextContent),
    /// A refusal from the model.
    Refusal(RefusalContent),
}

/// Nested content within an output message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputContent {
    /// An output message from the model.
    Message(OutputMessage),
    /// The results of a file search tool call.
    FileSearchCall(FileSearchCallOutput),
    /// A tool call to run a function.
    FunctionCall(FunctionCall),
    /// The results of a web search tool call.
    WebSearchCall(WebSearchCallOutput),
    /// A tool call to a computer use tool.
    ComputerCall(ComputerCallOutput),
    /// A description of the chain of thought used by a reasoning model while generating a response.
    /// Be sure to include these items in your input to the Responses API for subsequent turns of a
    /// conversation if you are manually managing context.
    Reasoning(ReasoningItem),
    /// Image generation tool call output.
    ImageGenerationCall(ImageGenerationCallOutput),
    /// Code interpreter tool call output.
    CodeInterpreterCall(CodeInterpreterCallOutput),
    /// Local shell tool call output.
    LocalShellCall(LocalShellCallOutput),
    /// MCP tool invocation output.
    McpCall(McpCallOutput),
    /// MCP list-tools output.
    McpListTools(McpListToolsOutput),
    /// MCP approval request output.
    McpApprovalRequest(McpApprovalRequestOutput),
}

/// Reasoning text content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningTextContent {
    /// The reasoning text from the model.
    pub text: String,
}

/// A reasoning item representing the model's chain of thought, including summary paragraphs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItem {
    /// Unique identifier of the reasoning content.
    pub id: String,
    /// Reasoning summary content.
    pub summary: Vec<Summary>,
    /// Reasoning text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningTextContent>>,
    /// The encrypted content of the reasoning item - populated when a response is generated with
    /// `reasoning.encrypted_content` in the `include` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>,
}

/// A single summary text fragment from reasoning.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Summary {
    /// A summary of the reasoning output from the model so far.
    pub text: String,
}

/// File search tool call output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    pub id: String,
    /// The queries used to search for files.
    pub queries: Vec<String>,
    /// The status of the file search tool call. One of `in_progress`, `searching`,
    /// `incomplete`,`failed`, or `completed`.
    pub status: FileSearchToolCallStatus,
    /// The results of the file search tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FileSearchToolCallResult>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileSearchToolCallStatus {
    InProgress,
    Searching,
    Incomplete,
    Failed,
    Completed,
}

/// A single result from a file search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileSearchToolCallResult {
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing
    /// additional information about the object in a structured format, and querying for objects
    /// API or the dashboard. Keys are strings with a maximum length of 64 characters
    /// . Values are strings with a maximum length of 512 characters, booleans, or numbers.
    pub attributes: HashMap<String, serde_json::Value>,
    /// The unique ID of the file.
    pub file_id: String,
    /// The name of the file.
    pub filename: String,
    /// The relevance score of the file - a value between 0 and 1.
    pub score: f32,
    /// The text that was retrieved from the file.
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerCallSafetyCheckParam {
    /// The ID of the pending safety check.
    pub id: String,
    /// The type of the pending safety check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Details about the pending safety check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchToolCallStatus {
    InProgress,
    Searching,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchActionSearchSource {
    /// The type of source. Always `url`.
    pub r#type: String,
    /// The URL of the source.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchActionSearch {
    /// The search query.
    pub query: String,
    /// The sources used in the search.
    pub sources: Option<Vec<WebSearchActionSearchSource>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchActionOpenPage {
    /// The URL opened by the model.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchActionFind {
    /// The URL of the page searched for the pattern.
    pub url: String,
    /// The pattern or text to search for within the page.
    pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSearchToolCallAction {
    /// Action type "search" - Performs a web search query.
    Search(WebSearchActionSearch),
    /// Action type "open_page" - Opens a specific URL from search results.
    OpenPage(WebSearchActionOpenPage),
    /// Action type "find": Searches for a pattern within a loaded page.
    Find(WebSearchActionFind),
}

/// Web search tool call output.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchToolCall {
    /// An object describing the specific action taken in this web search call. Includes
    /// details on how the model used the web (search, open_page, find).
    pub action: WebSearchToolCallAction,
    /// The unique ID of the web search tool call.
    pub id: String,
    /// The status of the web search tool call.
    pub status: WebSearchToolCallStatus,
}

/// Output from a computer tool call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerToolCall {
    pub action: ComputerAction,
    /// An identifier used when responding to the tool call with output.
    pub call_id: String,
    /// The unique ID of the computer call.
    pub id: String,
    /// The pending safety checks for the computer call.
    pub pending_safety_checks: Vec<ComputerCallSafetyCheckParam>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when items are returned via API.
    pub status: OutputStatus,
}

/// A point in 2D space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragPoint {
    /// The x-coordinate.
    pub x: i32,
    /// The y-coordinate.
    pub y: i32,
}

/// Represents all user‐triggered actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComputerAction {
    /// A click action.
    Click(ClickParam),

    /// A double click action.
    DoubleClick(DoubleClickAction),

    /// A drag action.
    Drag(Drag),

    /// A collection of keypresses the model would like to perform.
    Keypress(KeyPressAction),

    /// A mouse move action.
    Move(Move),

    /// A screenshot action.
    Screenshot,

    /// A scroll action.
    Scroll(Scroll),

    /// An action to type in text.
    Type(Type),

    /// A wait action.
    Wait,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClickButtonType {
    Left,
    Right,
    Wheel,
    Back,
    Forward,
}

/// A click action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClickParam {
    /// Indicates which mouse button was pressed during the click. One of `left`,
    /// `right`, `wheel`, `back`, or `forward`.
    pub button: ClickButtonType,
    /// The x-coordinate where the click occurred.
    pub x: i32,
    /// The y-coordinate where the click occurred.
    pub y: i32,
}

/// A double click action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoubleClickAction {
    /// The x-coordinate where the double click occurred.
    pub x: i32,
    /// The y-coordinate where the double click occurred.
    pub y: i32,
}

/// A drag action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drag {
    /// The path of points the cursor drags through.
    pub path: Vec<DragPoint>,
}

/// A keypress action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPressAction {
    /// The combination of keys the model is requesting to be pressed.
    /// This is an array of strings, each representing a key.
    pub keys: Vec<String>,
}

/// A mouse move action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Move {
    /// The x-coordinate to move to.
    pub x: i32,
    /// The y-coordinate to move to.
    pub y: i32,
}

/// A scroll action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scroll {
    /// The horizontal scroll distance.
    pub scroll_x: i32,
    /// The vertical scroll distance.
    pub scroll_y: i32,
    /// The x-coordinate where the scroll occurred.
    pub x: i32,
    /// The y-coordinate where the scroll occurred.
    pub y: i32,
}

/// A typing (text entry) action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    /// The text to type.
    pub text: String,
}

/// Metadata for a function call request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCall {
    /// The unique ID of the function tool call.
    pub id: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to run.
    pub name: String,
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The status of the item.
    pub status: OutputStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenToolCallStatus {
    InProgress,
    Completed,
    Generating,
    Failed,
}

/// Output of an image generation request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageGenToolCall {
    /// The unique ID of the image generation call.
    pub id: String,
    /// The generated image encoded in base64.
    pub result: Option<String>,
    /// The status of the image generation call.
    pub status: ImageGenToolCallStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CodeInterpreterToolCallStatus {
    InProgress,
    Completed,
    Incomplete,
    Interpreting,
    Failed,
}

/// Output of a code interpreter request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterToolCall {
    /// The code to run, or null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// ID of the container used to run the code.
    pub container_id: String,
    /// The unique ID of the code interpreter tool call.
    pub id: String,
    /// The outputs generated by the code interpreter, such as logs or images.
    /// Can be null if no outputs are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<CodeInterpreterToolCallOutput>>,
    /// The status of the code interpreter tool call.
    /// Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.
    pub status: CodeInterpreterToolCallStatus,
}

/// Individual result from a code interpreter: either logs or files.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeInterpreterToolCallOutput {
    /// Code interpreter output logs
    Logs(CodeInterpreterOutputLogs),
    /// Code interpreter output image
    Image(CodeInterpreterOutputImage),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterOutputLogs {
    /// The logs output from the code interpreter.
    pub logs: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterOutputImage {
    /// The URL of the image output from the code interpreter.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CodeInterpreterFile {
    /// The ID of the file.
    file_id: String,
    /// The MIME type of the file.
    mime_type: String,
}

/// Output of a local shell command request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalShellToolCall {
    /// Execute a shell command on the server.
    pub action: LocalShellExecAction,
    /// The unique ID of the local shell tool call generated by the model.
    pub call_id: String,
    /// The unique ID of the local shell call.
    pub id: String,
    /// The status of the local shell call.
    pub status: String,
}

/// Define the shape of a local shell action (exec).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalShellExecAction {
    /// The command to run.
    pub command: Vec<String>,
    /// Environment variables to set for the command.
    pub env: HashMap<String, String>,
    /// Optional timeout in milliseconds for the command.
    pub timeout_ms: Option<u64>,
    /// Optional user to run the command as.
    pub user: Option<String>,
    /// Optional working directory to run the command in.
    pub working_directory: Option<String>,
}

/// Output of an MCP server tool invocation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MCPToolCall {
    /// A JSON string of the arguments passed to the tool.
    pub arguments: String,
    /// The unique ID of the tool call.
    pub id: String,
    /// The name of the tool that was run.
    pub name: String,
    /// The label of the MCP server running the tool.
    pub server_label: String,
    /// Unique identifier for the MCP tool call approval request. Include this value
    /// in a subsequent `mcp_approval_response` input to approve or reject the corresponding
    /// tool call.
    pub approval_request_id: Option<String>,
    /// Error message from the call, if any.
    pub error: Option<String>,
    /// The output from the tool call.
    pub output: Option<String>,
    /// The status of the tool call. One of `in_progress`, `completed`, `incomplete`,
    /// `calling`, or `failed`.
    pub status: Option<MCPToolCallStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MCPToolCallStatus {
    InProgress,
    Completed,
    Incomplete,
    Calling,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MCPListTools {
    /// The unique ID of the list.
    pub id: String,
    /// The label of the MCP server.
    pub server_label: String,
    /// The tools available on the server.
    pub tools: Vec<MCPListToolsTool>,
    /// Error message if listing failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MCPListToolsTool {
    /// The JSON schema describing the tool's input.
    pub input_schema: serde_json::Value,
    /// The name of the tool.
    pub name: String,
    /// Additional annotations about the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    /// The description of the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Output representing a human approval request for an MCP tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MCPApprovalRequest {
    /// JSON string of arguments for the tool.
    pub arguments: String,
    /// The unique ID of the approval request.
    pub id: String,
    /// The name of the tool to run.
    pub name: String,
    /// The label of the MCP server making the request.
    pub server_label: String,
}

/// Usage statistics for a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Usage {
    /// The number of input tokens.
    pub input_tokens: u32,
    /// A detailed breakdown of the input tokens.
    pub input_tokens_details: PromptTokensDetails,
    /// The number of output tokens.
    pub output_tokens: u32,
    /// A detailed breakdown of the output tokens.
    pub output_tokens_details: CompletionTokensDetails,
    /// The total number of tokens used.
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Instructions {
    /// A text input to the model, equivalent to a text input with the `developer` role.
    Text(String),
    /// A list of one or many input items to the model, containing different content types.
    Array(Vec<InputItem>),
}

/// The complete response returned by the Responses API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Response {
    /// Whether to run the model response in the background.
    /// [Learn more](https://platform.openai.com/docs/guides/background).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// The conversation that this response belongs to. Input items and output
    /// items from this response are automatically added to this conversation.
    pub conversation: Option<Conversation>,

    /// Unix timestamp (in seconds) when this Response was created.
    pub created_at: u64,

    /// An error object returned when the model fails to generate a Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,

    /// Unique identifier for this response.
    pub id: String,

    /// Details about why the response is incomplete, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,

    /// A system (or developer) message inserted into the model's context.
    ///
    /// When using along with `previous_response_id`, the instructions from a previous response
    /// will not be carried over to the next response. This makes it simple to swap out
    /// system (or developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Instructions>,

    /// An upper bound for the number of tokens that can be generated for a response,
    /// including visible output tokens and
    /// [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be
    /// useful for storing additional information about the object in a structured
    /// format, and querying for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are strings
    /// with a maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Model ID used to generate the response, like gpt-4o or o3. OpenAI offers a
    /// wide range of models with different capabilities, performance characteristics,
    /// and price points. Refer to the [model guide](https://platform.openai.com/docs/models) to browse and compare available models.
    pub model: String,

    /// The object type of this resource - always set to `response`.
    pub object: String,

    /// An array of content items generated by the model.
    ///
    /// The length and order of items in the output array is dependent on the model's response.
    /// Rather than accessing the first item in the output array and assuming it's an assistant
    /// message with the content generated by the model, you might consider using
    /// the `output_text` property where supported in SDKs.
    pub output: Vec<OutputItem>,

    /// SDK-only convenience property that contains the aggregated text output from all
    /// `output_text` items in the `output` array, if any are present.
    /// Supported in the Python and JavaScript SDKs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,

    /// Whether parallel tool calls were enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Previous response ID, if creating part of a multi-turn conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Reasoning configuration echoed back (effort, summary settings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Whether to store the generated model response for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// The service tier that actually processed this response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// The status of the response generation.
    pub status: Status,

    /// Sampling temperature that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Text format configuration echoed back (plain, json_object, json_schema).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,

    /// How the model chose or was forced to choose a tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Tool definitions that were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Nucleus sampling cutoff that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Truncation strategy that was applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,

    /// Token usage statistics for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// End-user ID for which this response was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Completed,
    Failed,
    InProgress,
    Incomplete,
}

/// Event types for streaming responses from the Responses API
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive] // Future-proof against breaking changes
pub enum ResponseEvent {
    /// Response creation started
    #[serde(rename = "response.created")]
    ResponseCreated(ResponseCreated),
    /// Processing in progress
    #[serde(rename = "response.in_progress")]
    ResponseInProgress(ResponseInProgress),
    /// Response completed (different from done)
    #[serde(rename = "response.completed")]
    ResponseCompleted(ResponseCompleted),
    /// Response failed
    #[serde(rename = "response.failed")]
    ResponseFailed(ResponseFailed),
    /// Response incomplete
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete(ResponseIncomplete),
    /// Response queued
    #[serde(rename = "response.queued")]
    ResponseQueued(ResponseQueued),
    /// Output item added
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(ResponseOutputItemAdded),
    /// Content part added
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(ResponseContentPartAdded),
    /// Text delta update
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta(ResponseOutputTextDelta),
    /// Text output completed
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone(ResponseOutputTextDone),
    /// Refusal delta update
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta(ResponseRefusalDelta),
    /// Refusal completed
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone(ResponseRefusalDone),
    /// Content part completed
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(ResponseContentPartDone),
    /// Output item completed
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(ResponseOutputItemDone),
    /// Function call arguments delta
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDelta),
    /// Function call arguments completed
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDone),
    /// File search call in progress
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInProgress),
    /// File search call searching
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching(ResponseFileSearchCallSearching),
    /// File search call completed
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted(ResponseFileSearchCallCompleted),
    /// Web search call in progress
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress(ResponseWebSearchCallInProgress),
    /// Web search call searching
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching(ResponseWebSearchCallSearching),
    /// Web search call completed
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted(ResponseWebSearchCallCompleted),
    /// Reasoning summary part added
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded(ResponseReasoningSummaryPartAdded),
    /// Reasoning summary part done
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone(ResponseReasoningSummaryPartDone),
    /// Reasoning summary text delta
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta(ResponseReasoningSummaryTextDelta),
    /// Reasoning summary text done
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone(ResponseReasoningSummaryTextDone),
    /// Reasoning summary delta
    #[serde(rename = "response.reasoning_summary.delta")]
    ResponseReasoningSummaryDelta(ResponseReasoningSummaryDelta),
    /// Reasoning summary done
    #[serde(rename = "response.reasoning_summary.done")]
    ResponseReasoningSummaryDone(ResponseReasoningSummaryDone),
    /// Image generation call in progress
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenerationCallInProgress(ResponseImageGenerationCallInProgress),
    /// Image generation call generating
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenerationCallGenerating(ResponseImageGenerationCallGenerating),
    /// Image generation call partial image
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenerationCallPartialImage(ResponseImageGenerationCallPartialImage),
    /// Image generation call completed
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenerationCallCompleted(ResponseImageGenerationCallCompleted),
    /// MCP call arguments delta
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta(ResponseMcpCallArgumentsDelta),
    /// MCP call arguments done
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone(ResponseMcpCallArgumentsDone),
    /// MCP call completed
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted(ResponseMcpCallCompleted),
    /// MCP call failed
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed(ResponseMcpCallFailed),
    /// MCP call in progress
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress(ResponseMcpCallInProgress),
    /// MCP list tools completed
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMcpListToolsCompleted(ResponseMcpListToolsCompleted),
    /// MCP list tools failed
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMcpListToolsFailed(ResponseMcpListToolsFailed),
    /// MCP list tools in progress
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMcpListToolsInProgress(ResponseMcpListToolsInProgress),
    /// Code interpreter call in progress
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress(ResponseCodeInterpreterCallInProgress),
    /// Code interpreter call interpreting
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallInterpreting),
    /// Code interpreter call completed
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallCompleted),
    /// Code interpreter call code delta
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeDelta),
    /// Code interpreter call code done
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeDone),
    /// Output text annotation added
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded(ResponseOutputTextAnnotationAdded),
    /// Error occurred
    #[serde(rename = "error")]
    ResponseError(ResponseError),

    /// Unknown event type
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

/// Stream of response events
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<ResponseEvent, OpenAIError>> + Send>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCreated {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseInProgress {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseOutputItemAdded {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item: OutputItem,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseContentPartAdded {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseOutputTextDelta {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseContentPartDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub part: ContentPart,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseOutputItemDone {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item: OutputItem,
}

/// Response completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCompleted {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

/// Response failed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFailed {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

/// Response incomplete event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseIncomplete {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

/// Response queued event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseQueued {
    pub sequence_number: u64,
    pub response: ResponseMetadata,
}

/// Text output completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseOutputTextDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub text: String,
    pub logprobs: Option<Vec<serde_json::Value>>,
}

/// Refusal delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseRefusalDelta {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub delta: String,
}

/// Refusal done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseRefusalDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub refusal: String,
}

/// Function call arguments delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFunctionCallArgumentsDelta {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub delta: String,
}

/// Function call arguments done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFunctionCallArgumentsDone {
    pub name: String,
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub arguments: String,
}

/// Error event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseError {
    pub sequence_number: u64,
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
}

/// File search call in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFileSearchCallInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// File search call searching event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFileSearchCallSearching {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// File search call completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseFileSearchCallCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Web search call in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseWebSearchCallInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Web search call searching event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseWebSearchCallSearching {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Web search call completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseWebSearchCallCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Reasoning summary part added event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryPartAdded {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub part: serde_json::Value, // Could be more specific but using Value for flexibility
}

/// Reasoning summary part done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryPartDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub part: serde_json::Value,
}

/// Reasoning summary text delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryTextDelta {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub delta: String,
}

/// Reasoning summary text done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryTextDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub text: String,
}

/// Reasoning summary delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryDelta {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub delta: serde_json::Value,
}

/// Reasoning summary done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseReasoningSummaryDone {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub summary_index: u32,
    pub text: String,
}

/// Image generation call in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseImageGenerationCallInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Image generation call generating event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseImageGenerationCallGenerating {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Image generation call partial image event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseImageGenerationCallPartialImage {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub partial_image_index: u32,
    pub partial_image_b64: String,
}

/// Image generation call completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseImageGenerationCallCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP call arguments delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpCallArgumentsDelta {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub delta: String,
}

/// MCP call arguments done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpCallArgumentsDone {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub arguments: String,
}

/// MCP call completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpCallCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP call failed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpCallFailed {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP call in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpCallInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP list tools completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpListToolsCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP list tools failed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpListToolsFailed {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// MCP list tools in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMcpListToolsInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Code interpreter call in progress event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCodeInterpreterCallInProgress {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Code interpreter call interpreting event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCodeInterpreterCallInterpreting {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Code interpreter call completed event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCodeInterpreterCallCompleted {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
}

/// Code interpreter call code delta event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCodeInterpreterCallCodeDelta {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub delta: String,
}

/// Code interpreter call code done event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseCodeInterpreterCallCodeDone {
    pub sequence_number: u64,
    pub output_index: u32,
    pub item_id: String,
    pub code: String,
}

/// Response metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseMetadata {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    pub created_at: u64,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    /// Whether the model was run in background mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// The service tier that was actually used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// The effective value of top_logprobs parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// The effective value of max_tool_calls parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<OutputItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Prompt cache key for improved performance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Safety identifier for content filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

/// Output item
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OutputItem {
    /// An output message from the model.
    Message(OutputMessage),
    /// The results of a file search tool call. See the
    /// [file search guide](https://platform.openai.com/docs/guides/tools-file-search)
    /// for more information.
    FileSearchCall(FileSearchToolCall),
    /// A tool call to run a function. See the
    /// [function calling guide](https://platform.openai.com/docs/guides/function-calling)
    /// for more information.
    FunctionCall(FunctionToolCall),
    /// The results of a web search tool call. See the
    /// [web search guide](https://platform.openai.com/docs/guides/tools-web-search)
    /// for more information.
    WebSearchCall(WebSearchToolCall),
    /// A tool call to a computer use tool. See the
    /// [computer use guide](https://platform.openai.com/docs/guides/tools-computer-use)
    /// for more information.
    ComputerCall(ComputerToolCall),
    /// A description of the chain of thought used by a reasoning model while generating
    /// a response. Be sure to include these items in your `input` to the Responses API for
    /// subsequent turns of a conversation if you are manually
    /// [managing context](https://platform.openai.com/docs/guides/conversation-state).
    Reasoning(ReasoningItem),
    /// An image generation request made by the model.
    ImageGenerationCall(ImageGenToolCall),
    /// A tool call to run code.
    CodeInterpreterCall(CodeInterpreterToolCall),
    /// A tool call to run a command on the local shell.
    LocalShellCall(LocalShellToolCall),
    /// An invocation of a tool on an MCP server.
    McpCall(MCPToolCall),
    /// A list of tools available on an MCP server.
    McpListTools(MCPListTools),
    /// A request for human approval of a tool invocation.
    McpApprovalRequest(MCPApprovalRequest),
    /// A call to a custom tool created by the model.
    CustomToolCall(CustomToolCall),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct CustomToolCall {
    /// An identifier used to map this custom tool call to a tool call output.
    pub call_id: String,
    /// The input for the custom tool call generated by the model.
    pub input: String,
    /// The name of the custom tool being called.
    pub name: String,
    /// The unique ID of the custom tool call in the OpenAI platform.
    pub id: String,
}

/// Content part
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ContentPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<serde_json::Value>>,
}

// ===== RESPONSE COLLECTOR =====

/// Collects streaming response events into a complete response

/// Output text annotation added event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct ResponseOutputTextAnnotationAdded {
    pub sequence_number: u64,
    pub item_id: String,
    pub output_index: u32,
    pub content_index: u32,
    pub annotation_index: u32,
    pub annotation: TextAnnotation,
}

/// Text annotation object for output text
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[non_exhaustive]
pub struct TextAnnotation {
    #[serde(rename = "type")]
    pub annotation_type: String,
    pub text: String,
    pub start: u32,
    pub end: u32,
}
