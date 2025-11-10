use crate::error::OpenAIError;
pub use crate::types::chat::{
    CompletionTokensDetails, ImageDetail, PromptTokensDetails, ReasoningEffort,
    ResponseFormatJsonSchema,
};
use crate::types::{MCPListToolsTool, MCPTool};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum InputParam {
    ///  A text input to the model, equivalent to a text input with the
    /// `user` role.
    Text(String),
    /// A list of one or many input items to the model, containing
    /// different content types.
    Items(Vec<InputItem>),
}

impl Default for InputParam {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

/// Content item used to generate a response.
///
/// This is a properly discriminated union based on the `type` field, using Rust's
/// type-safe enum with serde's tag attribute for efficient deserialization.
///
/// # OpenAPI Specification
/// Corresponds to the `Item` schema in the OpenAPI spec with a `type` discriminator.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Item {
    /// A message (type: "message").
    /// Can represent InputMessage (user/system/developer) or OutputMessage (assistant).
    ///
    /// InputMessage:
    ///     A message input to the model with a role indicating instruction following hierarchy.
    ///     Instructions given with the developer or system role take precedence over instructions given with the user role.
    /// OutputMessage:
    ///     A message output from the model.
    Message(MessageItem),

    /// The results of a file search tool call. See the
    /// [file search guide](https://platform.openai.com/docs/guides/tools-file-search) for more information.
    FileSearchCall(FileSearchToolCall),

    /// A tool call to a computer use tool. See the
    /// [computer use guide](https://platform.openai.com/docs/guides/tools-computer-use) for more information.
    ComputerCall(ComputerToolCall),

    /// The output of a computer tool call.
    ComputerCallOutput(ComputerCallOutputItemParam),

    /// The results of a web search tool call. See the
    /// [web search guide](https://platform.openai.com/docs/guides/tools-web-search) for more information.
    WebSearchCall(WebSearchToolCall),

    /// A tool call to run a function. See the
    ///
    /// [function calling guide](https://platform.openai.com/docs/guides/function-calling) for more information.
    FunctionCall(FunctionToolCall),

    /// The output of a function tool call.
    FunctionCallOutput(FunctionCallOutputItemParam),

    /// A description of the chain of thought used by a reasoning model while generating
    /// a response. Be sure to include these items in your `input` to the Responses API
    /// for subsequent turns of a conversation if you are manually
    /// [managing context](https://platform.openai.com/docs/guides/conversation-state).
    Reasoning(ReasoningItem),

    /// An image generation request made by the model.
    ImageGenerationCall(ImageGenToolCall),

    /// A tool call to run code.
    CodeInterpreterCall(CodeInterpreterToolCall),

    /// A tool call to run a command on the local shell.
    LocalShellCall(LocalShellToolCall),

    /// The output of a local shell tool call.
    LocalShellCallOutput(LocalShellToolCallOutput),

    /// A list of tools available on an MCP server.
    McpListTools(MCPListTools),

    /// A request for human approval of a tool invocation.
    McpApprovalRequest(MCPApprovalRequest),

    /// A response to an MCP approval request.
    McpApprovalResponse(MCPApprovalResponse),

    /// An invocation of a tool on an MCP server.
    McpCall(MCPToolCall),

    /// The output of a custom tool call from your code, being sent back to the model.
    CustomToolCallOutput(CustomToolCallOutput),

    /// A call to a custom tool created by the model.
    CustomToolCall(CustomToolCall),
}

/// Input item that can be used in the context for generating a response.
///
/// This represents the OpenAPI `InputItem` schema which is an `anyOf`:
/// 1. `EasyInputMessage` - Simple, user-friendly message input (can use string content)
/// 2. `Item` - Structured items with proper type discrimination (including InputMessage, OutputMessage, tool calls)
/// 3. `ItemReferenceParam` - Reference to an existing item by ID (type can be null)
///
/// Uses untagged deserialization because these types overlap in structure.
/// Order matters: more specific structures are tried first.
///
/// # OpenAPI Specification
/// Corresponds to the `InputItem` schema: `anyOf[EasyInputMessage, Item, ItemReferenceParam]`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum InputItem {
    /// A reference to an existing item by ID.
    /// Has a required `id` field and optional `type` (can be "item_reference" or null).
    /// Must be tried first as it's the most minimal structure.
    ItemReference(ItemReference),

    /// All structured items with proper type discrimination.
    /// Includes InputMessage, OutputMessage, and all tool calls/outputs.
    /// Uses the discriminated `Item` enum for efficient, type-safe deserialization.
    Item(Item),

    /// A simple, user-friendly message input (EasyInputMessage).
    /// Supports string content and can include assistant role for previous responses.
    /// Must be tried last as it's the most flexible structure.
    ///
    /// A message input to the model with a role indicating instruction following
    /// hierarchy. Instructions given with the `developer` or `system` role take
    /// precedence over instructions given with the `user` role. Messages with the
    /// `assistant` role are presumed to have been generated by the model in previous
    /// interactions.
    EasyMessage(EasyInputMessage),
}

impl InputItem {
    /// Creates an InputItem from an item reference ID.
    pub fn from_reference(id: impl Into<String>) -> Self {
        Self::ItemReference(ItemReference::new(id))
    }

    /// Creates an InputItem from a structured Item.
    pub fn from_item(item: Item) -> Self {
        Self::Item(item)
    }

    /// Creates an InputItem from an EasyInputMessage.
    pub fn from_easy_message(message: EasyInputMessage) -> Self {
        Self::EasyMessage(message)
    }

    /// Creates a simple text message with the given role and content.
    pub fn text_message(role: Role, content: impl Into<String>) -> Self {
        Self::EasyMessage(EasyInputMessage {
            r#type: MessageType::Message,
            role,
            content: EasyInputContent::Text(content.into()),
        })
    }
}

/// A message item used within the `Item` enum.
///
/// Both InputMessage and OutputMessage have `type: "message"`, so we use an untagged
/// enum to distinguish them based on their structure:
/// - OutputMessage: role=assistant, required id & status fields
/// - InputMessage: role=user/system/developer, content is Vec<ContentType>, optional id/status
///
/// Note: EasyInputMessage is NOT included here - it's a separate variant in `InputItem`,
/// not part of the structured `Item` enum.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageItem {
    /// An output message from the model (role: assistant, has required id & status).
    /// This must come first as it has the most specific structure (required id and status fields).
    Output(OutputMessage),

    /// A structured input message (role: user/system/developer, content is Vec<ContentType>).
    /// Has structured content list and optional id/status fields.
    ///
    /// A message input to the model with a role indicating instruction following hierarchy.
    /// Instructions given with the `developer` or `system` role take precedence over instructions
    /// given with the `user` role.
    Input(InputMessage),
}

/// A reference to an existing item by ID.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemReference {
    /// The type of item to reference. Can be "item_reference" or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemReferenceType>,
    /// The ID of the item to reference.
    pub id: String,
}

impl ItemReference {
    /// Create a new item reference with the given ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            r#type: Some(ItemReferenceType::ItemReference),
            id: id.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemReferenceType {
    ItemReference,
}

/// Output from a function call that you're providing back to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallOutputItemParam {
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// Text, image, or file output of the function tool call.
    pub output: FunctionCallOutput,
    /// The unique ID of the function tool call output.
    /// Populated when this item is returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum FunctionCallOutput {
    /// A JSON string of the output of the function tool call.
    Text(String),
    Content(Vec<InputContent>), // TODO use shape which allows null from OpenAPI spec?
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerCallOutputItemParam {
    /// The ID of the computer tool call that produced the output.
    pub call_id: String,
    /// A computer screenshot image used with the computer use tool.
    pub output: ComputerScreenshotImage,
    /// The safety checks reported by the API that have been acknowledged by the developer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_safety_checks: Option<Vec<ComputerCallSafetyCheckParam>>,
    /// The unique ID of the computer tool call output. Optional when creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the message input. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when input items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>, // TODO rename OutputStatus?
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComputerScreenshotImageType {
    ComputerScreenshot,
}

/// A computer screenshot image used with the computer use tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComputerScreenshotImage {
    /// Specifies the event type. For a computer screenshot, this property is always
    /// set to `computer_screenshot`.
    pub r#type: ComputerScreenshotImageType,
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Output from a local shell tool call that you're providing back to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalShellToolCallOutput {
    /// The unique ID of the local shell tool call generated by the model.
    pub id: String,

    /// A JSON string of the output of the local shell tool call.
    pub output: String,

    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>,
}

/// Output from a local shell command execution.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalShellOutput {
    /// The stdout output from the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,

    /// The stderr output from the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,

    /// The exit code of the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
}

/// An MCP approval response that you're providing back to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MCPApprovalResponse {
    /// The ID of the approval request being answered.
    pub approval_request_id: String,

    /// Whether the request was approved.
    pub approve: bool,

    /// The unique ID of the approval response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional reason for the decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CustomToolCallOutputOutput {
    /// A string of the output of the custom tool call.
    Text(String),
    /// Text, image, or file output of the custom tool call.
    List(Vec<InputContent>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomToolCallOutput {
    /// The call ID, used to map this custom tool call output to a custom tool call.
    pub call_id: String,

    /// The output from the custom tool call generated by your code.
    /// Can be a string or an list of output content.
    pub output: CustomToolCallOutputOutput,

    /// The unique ID of the custom tool call output in the OpenAI platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A simplified message input to the model (EasyInputMessage in the OpenAPI spec).
///
/// This is the most user-friendly way to provide messages, supporting both simple
/// string content and structured content. Role can include `assistant` for providing
/// previous assistant responses.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "EasyInputMessageArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct EasyInputMessage {
    /// The type of the message input. Always set to `message`.
    pub r#type: MessageType,
    /// The role of the message input. One of `user`, `assistant`, `system`, or `developer`.
    pub role: Role,
    /// Text, image, or audio input to the model, used to generate a response.
    /// Can also contain previous assistant responses.
    pub content: EasyInputContent,
}

/// A structured message input to the model (InputMessage in the OpenAPI spec).
///
/// This variant requires structured content (not a simple string) and does not support
/// the `assistant` role (use OutputMessage for that). Used when items are returned via API
/// with additional metadata.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "InputMessageArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct InputMessage {
    /// A list of one or many input items to the model, containing different content types.
    pub content: Vec<InputContent>,
    /// The role of the message input. One of `user`, `system`, or `developer`.
    /// Note: `assistant` is NOT allowed here; use OutputMessage instead.
    pub role: InputRole,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>,
    /////The type of the message input. Always set to `message`.
    //pub r#type: MessageType,
}

/// The role for an input message - can only be `user`, `system`, or `developer`.
/// This type ensures type safety by excluding the `assistant` role (use OutputMessage for that).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum InputRole {
    #[default]
    User,
    System,
    Developer,
}

/// Content for EasyInputMessage - can be a simple string or structured list.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum EasyInputContent {
    /// A text input to the model.
    Text(String),
    /// A list of one or many input items to the model, containing different content types.
    ContentList(Vec<InputContent>),
}

/// Parts of a message: text, image, file, or audio.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputContent {
    /// A text input to the model.
    InputText(InputTextContent),
    /// An image input to the model. Learn about
    /// [image inputs](https://platform.openai.com/docs/guides/vision).
    InputImage(InputImageContent),
    /// A file input to the model.
    InputFile(InputFileContent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTextContent {
    /// The text input to the model.
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
pub struct InputImageContent {
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`.
    /// Defaults to `auto`.
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
pub struct InputFileContent {
    /// The content of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_data: Option<String>,
    /// The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<String>,
    /// The URL of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    file_url: Option<String>,
    /// The name of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Conversation {
    /// The unique ID of the conversation.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ConversationParam {
    /// The unique ID of the conversation.
    ConversationID(String),
    /// The conversation that this response belongs to.
    Object(Conversation),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IncludeEnum {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "web_search_call.results")]
    WebSearchCallResults,
    #[serde(rename = "web_search_call.action.sources")]
    WebSearchCallActionSources,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseStreamOptions {
    /// When true, stream obfuscation will be enabled. Stream obfuscation adds
    /// random characters to an `obfuscation` field on streaming delta events to
    /// normalize payload sizes as a mitigation to certain side-channel attacks.
    /// These obfuscation fields are included by default, but add a small amount
    /// of overhead to the data stream. You can set `include_obfuscation` to
    /// false to optimize for bandwidth if you trust the network links between
    /// your application and the OpenAI API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
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
    /// Whether to run the model response in the background.
    /// [Learn more](https://platform.openai.com/docs/guides/background).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// The conversation that this response belongs to. Items from this conversation are prepended to
    ///  `input_items` for this response request.
    ///
    /// Input items and output items from this response are automatically added to this conversation after
    /// this response completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,

    /// Specify additional output data to include in the model response. Currently supported
    /// values are:
    ///
    /// - `web_search_call.action.sources`: Include the sources of the web search tool call.
    ///
    /// - `code_interpreter_call.outputs`: Includes the outputs of python code execution in code
    ///   interpreter tool call items.
    ///
    /// - `computer_call_output.output.image_url`: Include image urls from the computer call
    ///   output.
    ///
    /// - `file_search_call.results`: Include the search results of the file search tool call.
    ///
    /// - `message.input_image.image_url`: Include image urls from the input message.
    ///
    /// - `message.output_text.logprobs`: Include logprobs with assistant messages.
    ///
    /// - `reasoning.encrypted_content`: Includes an encrypted version of reasoning tokens in
    ///   reasoning item outputs. This enables reasoning items to be used in multi-turn
    ///   conversations when using the Responses API statelessly (like when the `store` parameter is
    ///   set to `false`, or when an organization is enrolled in the zero data retention program).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<IncludeEnum>>,

    /// Text, image, or file inputs to the model, used to generate a response.
    ///
    /// Learn more:
    /// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
    /// - [Image inputs](https://platform.openai.com/docs/guides/images)
    /// - [File inputs](https://platform.openai.com/docs/guides/pdf-files)
    /// - [Conversation state](https://platform.openai.com/docs/guides/conversation-state)
    /// - [Function calling](https://platform.openai.com/docs/guides/function-calling)
    pub input: InputParam,

    /// A system (or developer) message inserted into the model's context.
    ///
    /// When using along with `previous_response_id`, the instructions from a previous
    /// response will not be carried over to the next response. This makes it simple
    /// to swap out system (or developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// An upper bound for the number of tokens that can be generated for a response, including
    /// visible output tokens and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// The maximum number of total calls to built-in tools that can be processed in a response. This
    /// maximum number applies across all built-in tool calls, not per individual tool. Any further
    /// attempts to call a tool by the model will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be
    /// useful for storing additional information about the object in a structured
    /// format, and querying for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. Values are
    /// strings with a maximum length of 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI
    /// offers a wide range of models with different capabilities, performance
    /// characteristics, and price points. Refer to the [model guide](https://platform.openai.com/docs/models)
    /// to browse and compare available models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// The unique ID of the previous response to the model. Use this to create multi-turn conversations.
    /// Learn more about [conversation state](https://platform.openai.com/docs/guides/conversation-state).
    /// Cannot be used in conjunction with `conversation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Reference to a prompt template and its variables.
    /// [Learn more](https://platform.openai.com/docs/guides/text?api-mode=responses#reusable-prompts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,

    /// Used by OpenAI to cache responses for similar requests to optimize your cache hit rates. Replaces
    /// the `user` field. [Learn more](https://platform.openai.com/docs/guides/prompt-caching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// **gpt-5 and o-series models only**
    /// Configuration options for [reasoning models](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,

    /// A stable identifier used to help detect users of your application that may be violating OpenAI's
    /// usage policies.
    ///
    /// The IDs should be a string that uniquely identifies each user. We recommend hashing their username
    /// or email address, in order to avoid sending us any identifying information. [Learn
    /// more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    /// Specifies the processing type used for serving the request.
    /// - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.
    /// - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.
    /// - If set to '[flex](https://platform.openai.com/docs/guides/flex-processing)' or '[priority](https://openai.com/api-priority-processing/)', then the request will be processed with the corresponding service tier.
    /// - When not set, the default behavior is 'auto'.
    ///
    /// When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Whether to store the generated model response for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    /// See the [Streaming section below](https://platform.openai.com/docs/api-reference/responses-streaming)
    /// for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming responses. Only set this when you set `stream: true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ResponseStreamOptions>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8
    /// will make the output more random, while lower values like 0.2 will make it
    /// more focused and deterministic. We generally recommend altering this or
    /// `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Configuration options for a text response from the model. Can be plain
    /// text or structured JSON data. Learn more:
    /// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
    /// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextParam>,

    /// How the model should select which tool (or tools) to use when generating
    /// a response. See the `tools` parameter to see how to specify which tools
    /// the model can call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceParam>,

    /// An array of tools the model may call while generating a response. You
    /// can specify which tool to use by setting the `tool_choice` parameter.
    ///
    /// We support the following categories of tools:
    /// - **Built-in tools**: Tools that are provided by OpenAI that extend the
    ///   model's capabilities, like [web search](https://platform.openai.com/docs/guides/tools-web-search)
    ///   or [file search](https://platform.openai.com/docs/guides/tools-file-search). Learn more about
    ///   [built-in tools](https://platform.openai.com/docs/guides/tools).
    /// - **MCP Tools**: Integrations with third-party systems via custom MCP servers
    ///   or predefined connectors such as Google Drive and SharePoint. Learn more about
    ///   [MCP Tools](https://platform.openai.com/docs/guides/tools-connectors-mcp).
    /// - **Function calls (custom tools)**: Functions that are defined by you,
    ///   enabling the model to call your own code with strongly typed arguments
    ///   and outputs. Learn more about
    ///   [function calling](https://platform.openai.com/docs/guides/function-calling). You can also use
    ///   custom tools to call your own code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each
    /// token position, each with an associated log probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability
    /// mass. So 0.1 means only the tokens comprising the top 10% probability mass
    /// are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    ///The truncation strategy to use for the model response.
    /// - `auto`: If the input to this Response exceeds
    ///   the model's context window size, the model will truncate the
    ///   response to fit the context window by dropping items from the beginning of the conversation.
    /// - `disabled` (default): If the input size will exceed the context window
    ///   size for a model, the request will fail with a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResponsePromptVariables {
    String(String),
    Content(InputContent),
    Custom(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Prompt {
    /// The unique identifier of the prompt template to use.
    pub id: String,

    /// Optional version of the prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Optional map of values to substitute in for variables in your
    /// prompt. The substitution values can either be strings, or other
    /// Response input types like images or files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<ResponsePromptVariables>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Billing {
    pub payer: String,
}

/// o-series reasoning settings.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "ReasoningArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct Reasoning {
    /// Constrains effort on reasoning for
    /// [reasoning models](https://platform.openai.com/docs/guides/reasoning).
    /// Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing
    /// reasoning effort can result in faster responses and fewer tokens used
    /// on reasoning in a response.
    ///
    /// Note: The `gpt-5-pro` model defaults to (and only supports) `high` reasoning effort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// A summary of the reasoning performed by the model. This can be
    /// useful for debugging and understanding the model's reasoning process.
    /// One of `auto`, `concise`, or `detailed`.
    ///
    /// `concise` is only supported for `computer-use-preview` models.
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
pub struct ResponseTextParam {
    /// An object specifying the format that the model must output.
    ///
    /// Configuring `{ "type": "json_schema" }` enables Structured Outputs,
    /// which ensures the model will match your supplied JSON schema. Learn more in the
    /// [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    ///
    /// The default format is `{ "type": "text" }` with no additional options.
    ///
    /// **Not recommended for gpt-4o and newer models:**
    ///
    /// Setting to `{ "type": "json_object" }` enables the older JSON mode, which
    /// ensures the message the model generates is valid JSON. Using `json_schema`
    /// is preferred for models that support it.
    pub format: TextResponseFormatConfiguration,

    /// Constrains the verbosity of the model's response. Lower values will result in
    /// more concise responses, while higher values will result in more verbose responses.
    ///
    /// Currently supported values are `low`, `medium`, and `high`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextResponseFormatConfiguration {
    /// Default response format. Used to generate text responses.
    Text,
    /// JSON object response format. An older method of generating JSON responses.
    /// Using `json_schema` is recommended for models that support it.
    /// Note that the model will not generate JSON without a system or user message
    /// instructing it to do so.
    JsonObject,
    /// JSON Schema response format. Used to generate structured JSON responses.
    /// Learn more about [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs).
    JsonSchema(ResponseFormatJsonSchema),
}

/// Definitions for model-callable tools.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    /// Defines a function in your own code the model can choose to call. Learn more about [function
    /// calling](https://platform.openai.com/docs/guides/tools).
    Function(FunctionTool),
    /// A tool that searches for relevant content from uploaded files. Learn more about the [file search
    /// tool](https://platform.openai.com/docs/guides/tools-file-search).
    FileSearch(FileSearchTool),
    /// A tool that controls a virtual computer. Learn more about the [computer
    /// use tool](https://platform.openai.com/docs/guides/tools-computer-use).
    ComputerUsePreview(ComputerUsePreviewTool),
    /// Search the Internet for sources related to the prompt. Learn more about the
    /// [web search tool](https://platform.openai.com/docs/guides/tools-web-search).
    WebSearch(WebSearchTool),
    /// type: web_search_2025_08_26
    #[serde(rename = "web_search_2025_08_26")]
    WebSearch20250826(WebSearchTool),
    /// Give the model access to additional tools via remote Model Context Protocol
    /// (MCP) servers. [Learn more about MCP](https://platform.openai.com/docs/guides/tools-remote-mcp).
    Mcp(MCPTool),
    /// A tool that runs Python code to help generate a response to a prompt.
    CodeInterpreter(CodeInterpreterTool),
    /// A tool that generates images using a model like `gpt-image-1`.
    ImageGeneration(ImageGenTool),
    /// A tool that allows the model to execute shell commands in a local environment.
    LocalShell,
    /// A custom tool that processes input using a specified format. Learn more about   [custom
    /// tools](https://platform.openai.com/docs/guides/function-calling#custom-tools)
    Custom(CustomToolParam),
    /// This tool searches the web for relevant results to use in a response. Learn more about the [web search
    ///tool](https://platform.openai.com/docs/guides/tools-web-search).
    WebSearchPreview(WebSearchTool),
    /// type: web_search_preview_2025_03_11
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311(WebSearchTool),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
pub struct CustomToolParam {
    /// The name of the custom tool, used to identify it in tool calls.
    pub name: String,
    /// Optional description of the custom tool, used to provide more context.
    pub description: Option<String>,
    /// The input format for the custom tool. Default is unconstrained text.
    pub format: CustomToolParamFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum GrammarSyntax {
    Lark,
    #[default]
    Regex,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
pub struct CustomGrammarFormatParam {
    /// The grammar definition.
    pub definition: String,
    /// The syntax of the grammar definition. One of `lark` or `regex`.
    pub syntax: GrammarSyntax,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CustomToolParamFormat {
    /// Unconstrained free-form text.
    #[default]
    Text,
    /// A grammar defined by the user.
    Grammar(CustomGrammarFormatParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "FileSearchToolArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct FileSearchTool {
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
    name = "FunctionToolArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct FunctionTool {
    /// The name of the function to call.
    pub name: String,
    /// A JSON schema object describing the parameters of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// Whether to enforce strict parameter validation. Default `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// A description of the function. Used by the model to determine whether or not to call the
    /// function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebSearchToolFilters {
    /// Allowed domains for the search. If not provided, all domains are allowed.
    /// Subdomains of the provided domains are allowed as well.
    ///
    /// Example: `["pubmed.ncbi.nlm.nih.gov"]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "WebSearchToolArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct WebSearchTool {
    /// Filters for the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<WebSearchToolFilters>,
    /// The approximate location of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<WebSearchApproximateLocation>,
    /// High level guidance for the amount of context window space to use for the search. One of `low`,
    /// `medium`, or `high`. `medium` is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<WebSearchToolSearchContextSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchToolSearchContextSize {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ComputerEnvironment {
    Windows,
    Mac,
    Linux,
    Ubuntu,
    #[default]
    Browser,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "ComputerUsePreviewToolArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
pub struct ComputerUsePreviewTool {
    /// The type of computer environment to control.
    environment: ComputerEnvironment,
    /// The width of the computer display.
    display_width: u32,
    /// The height of the computer display.
    display_height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum RankVersionType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HybridSearch {
    /// The weight of the embedding in the reciprocal ranking fusion.
    pub embedding_weight: f32,
    /// The weight of the text in the reciprocal ranking fusion.
    pub text_weight: f32,
}

/// Options for search result ranking.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RankingOptions {
    /// Weights that control how reciprocal rank fusion balances semantic embedding matches versus
    /// sparse keyword matches when hybrid search is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_search: Option<HybridSearch>,
    /// The ranker to use for the file search.
    pub ranker: RankVersionType,
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
    /// Combine multiple filters using `and` or `or`.
    Compound(CompoundFilter),
}

/// Single comparison filter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComparisonFilter {
    /// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `nin`.
    /// - `eq`: equals
    /// - `ne`: not equal
    /// - `gt`: greater than
    /// - `gte`: greater than or equal
    /// - `lt`: less than
    /// - `lte`: less than or equal
    /// - `in`: in
    /// - `nin`: not in
    pub r#type: ComparisonType,
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
    GreaterThanOrEqual,
    #[serde(rename = "lt")]
    LessThan,
    #[serde(rename = "lte")]
    LessThanOrEqual,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "nin")]
    NotIn,
}

/// Combine multiple filters using `and` or `or`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompoundFilter {
    /// 'Type of operation: `and` or `or`.'
    pub r#type: CompoundType,
    /// Array of filters to combine. Items can be ComparisonFilter or CompoundFilter.
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompoundType {
    And,
    Or,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchApproximateLocationType {
    #[default]
    Approximate,
}

/// Approximate user location for web search.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "WebSearchApproximateLocationArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct WebSearchApproximateLocation {
    /// The type of location approximation. Always `approximate`.
    pub r#type: WebSearchApproximateLocationType,
    /// Free text input for the city of the user, e.g. `San Francisco`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,
    /// e.g. `US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Free text input for the region of the user, e.g. `California`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the user, e.g.
    /// `America/Los_Angeles`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// Container configuration for a code interpreter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeInterpreterToolContainer {
    /// Configuration for a code interpreter container. Optionally specify the IDs of the
    /// files to run the code on.
    Auto(CodeInterpreterContainerAuto),

    /// The container ID.
    #[serde(untagged)]
    ContainerID(String),
}

impl Default for CodeInterpreterToolContainer {
    fn default() -> Self {
        Self::Auto(CodeInterpreterContainerAuto::default())
    }
}

/// Auto configuration for code interpreter container.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CodeInterpreterContainerAuto {
    /// An optional list of uploaded files to make available to your code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Builder)]
#[builder(
    name = "CodeInterpreterToolArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CodeInterpreterTool {
    /// The code interpreter container. Can be a container ID or an object that
    /// specifies uploaded file IDs to make available to your code.
    pub container: CodeInterpreterToolContainer,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageGenToolInputImageMask {
    /// Base64-encoded mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// File ID for the mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum InputFidelity {
    #[default]
    High,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolModeration {
    #[default]
    Auto,
    Low,
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
pub struct ImageGenTool {
    /// Background type for the generated image. One of `transparent`,
    /// `opaque`, or `auto`. Default: `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageGenToolBackground>,
    /// Control how much effort the model will exert to match the style and features, especially facial features,
    /// of input images. This parameter is only supported for `gpt-image-1`. Unsupported
    /// for `gpt-image-1-mini`. Supports `high` and `low`. Defaults to `low`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_fidelity: Option<InputFidelity>,
    /// Optional mask for inpainting. Contains `image_url`
    /// (string, optional) and `file_id` (string, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_mask: Option<ImageGenToolInputImageMask>,
    /// The image generation model to use. Default: `gpt-image-1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Moderation level for the generated image. Default: `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ImageGenToolModeration>,
    /// Compression level for the output image. Default: 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,
    /// The output format of the generated image. One of `png`, `webp`, or
    /// `jpeg`. Default: `png`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageGenToolOutputFormat>,
    /// Number of partial images to generate in streaming mode, from 0 (default value) to 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u8>,
    /// The quality of the generated image. One of `low`, `medium`, `high`,
    /// or `auto`. Default: `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageGenToolQuality>,
    /// The size of the generated image. One of `1024x1024`, `1024x1536`,
    /// `1536x1024`, or `auto`. Default: `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageGenToolSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolBackground {
    Transparent,
    Opaque,
    #[default]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolOutputFormat {
    #[default]
    Png,
    Webp,
    Jpeg,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolQuality {
    Low,
    Medium,
    High,
    #[default]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenToolSize {
    #[default]
    Auto,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1024x1536")]
    Size1024x1536,
    #[serde(rename = "1536x1024")]
    Size1536x1024,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceAllowedMode {
    Auto,
    Required,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceAllowed {
    /// Constrains the tools available to the model to a pre-defined set.
    ///
    /// `auto` allows the model to pick from among the allowed tools and generate a
    /// message.
    ///
    /// `required` requires the model to call one or more of the allowed tools.
    pub mode: ToolChoiceAllowedMode,
    /// A list of tool definitions that the model should be allowed to call.
    ///
    /// For the Responses API, the list of tool definitions might look like:
    /// ```json
    /// [
    ///   { "type": "function", "name": "get_weather" },
    ///   { "type": "mcp", "server_label": "deepwiki" },
    ///   { "type": "image_generation" }
    /// ]
    /// ```
    pub tools: Vec<serde_json::Value>,
}

/// The type of hosted tool the model should to use. Learn more about
/// [built-in tools](https://platform.openai.com/docs/guides/tools).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoiceTypes {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
    CodeInterpreter,
    ImageGeneration,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceMCP {
    /// The name of the tool to call on the server.
    name: String,
    /// The label of the MCP server to use.
    server_label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoiceCustom {
    /// The name of the custom tool to call.
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoiceParam {
    /// Constrains the tools available to the model to a pre-defined set.
    AllowedTools(ToolChoiceAllowed),

    /// Use this option to force the model to call a specific function.
    Function(ToolChoiceFunction),

    /// Use this option to force the model to call a specific tool on a remote MCP server.
    Mcp(ToolChoiceMCP),

    /// Use this option to force the model to call a custom tool.
    Custom(ToolChoiceCustom),

    /// Indicates that the model should use a built-in tool to generate a response.
    /// [Learn more about built-in tools](https://platform.openai.com/docs/guides/tools).
    #[serde(untagged)]
    Hosted(ToolChoiceTypes),

    /// Controls which (if any) tool is called by the model.
    ///
    /// `none` means the model will not call any tool and instead generates a message.
    ///
    /// `auto` means the model can pick between generating a message or calling one or
    /// more tools.
    ///
    /// `required` means the model must call one or more tools.
    #[serde(untagged)]
    Mode(ToolChoiceOptions),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceOptions {
    None,
    Auto,
    Required,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseTopLobProb {
    /// The log probability of this token.
    pub logprob: f64,
    /// A possible text token.
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseLogProb {
    /// The log probability of this token.
    pub logprob: f64,
    /// A possible text token.
    pub token: String,
    /// The log probability of the top 20 most likely tokens.
    pub top_logprobs: Vec<ResponseTopLobProb>,
}

/// A simple text output from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTextContent {
    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,
    pub logprobs: Option<Vec<LogProb>>,
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
    pub role: AssistantRole,
    /// The status of the message input. One of `in_progress`, `completed`, or
    /// `incomplete`. Populated when input items are returned via API.
    pub status: OutputStatus,
    ///// The type of the output message. Always `message`.
    //pub r#type: MessageType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    #[default]
    Message,
}

/// The role for an output message - always `assistant`.
/// This type ensures type safety by only allowing the assistant role.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssistantRole {
    #[default]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputMessageContent {
    /// A text output from the model.
    OutputText(OutputTextContent),
    /// A refusal from the model.
    Refusal(RefusalContent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputContent {
    /// A text output from the model.
    OutputText(OutputTextContent),
    /// A refusal from the model.
    Refusal(RefusalContent),
    /// Reasoning text from the model.
    ReasoningText(ReasoningTextContent),
}

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
    pub summary: Vec<SummaryPart>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SummaryPart {
    SummaryText(Summary),
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolCall {
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to run.
    pub name: String,
    /// The unique ID of the function tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`.
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>, // TODO rename OutputStatus?
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenToolCallStatus {
    InProgress,
    Completed,
    Generating,
    Failed,
}

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalShellToolCall {
    /// Execute a shell command on the server.
    pub action: LocalShellExecAction,
    /// The unique ID of the local shell tool call generated by the model.
    pub call_id: String,
    /// The unique ID of the local shell call.
    pub id: String,
    /// The status of the local shell call.
    pub status: OutputStatus,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTokenDetails {
    /// The number of tokens that were retrieved from the cache.
    /// [More on prompt caching](https://platform.openai.com/docs/guides/prompt-caching).
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTokenDetails {
    /// The number of reasoning tokens.
    pub reasoning_tokens: u32,
}

/// Usage statistics for a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub input_tokens: u32,
    /// A detailed breakdown of the input tokens.
    pub input_tokens_details: InputTokenDetails,
    /// The number of output tokens.
    pub output_tokens: u32,
    /// A detailed breakdown of the output tokens.
    pub output_tokens_details: OutputTokenDetails,
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

    /// Billing information for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Billing>,

    /// The conversation that this response belongs to. Input items and output
    /// items from this response are automatically added to this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// - The length and order of items in the output array is dependent on the model's response.
    /// - Rather than accessing the first item in the output array and assuming it's an assistant
    ///   message with the content generated by the model, you might consider using
    ///   the `output_text` property where supported in SDKs.
    pub output: Vec<OutputItem>,

    /// SDK-only convenience property that contains the aggregated text output from all
    /// `output_text` items in the `output` array, if any are present.
    /// Supported in the Python and JavaScript SDKs.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub output_text: Option<String>,

    /// Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// The unique ID of the previous response to the model. Use this to create multi-turn conversations.
    /// Learn more about [conversation state](https://platform.openai.com/docs/guides/conversation-state).
    /// Cannot be used in conjunction with `conversation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Reference to a prompt template and its variables.
    /// [Learn more](https://platform.openai.com/docs/guides/text?api-mode=responses#reusable-prompts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,

    /// Used by OpenAI to cache responses for similar requests to optimize your cache hit rates. Replaces
    /// the `user` field. [Learn more](https://platform.openai.com/docs/guides/prompt-caching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// **gpt-5 and o-series models only**
    /// Configuration options for [reasoning models](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,

    /// A stable identifier used to help detect users of your application that may be violating OpenAI's
    /// usage policies.
    ///
    /// The IDs should be a string that uniquely identifies each user. We recommend hashing their username
    /// or email address, in order to avoid sending us any identifying information. [Learn
    /// more](https://platform.openai.com/docs/guides/safety-best-practices#safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    /// Specifies the processing type used for serving the request.
    /// - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.
    /// - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.
    /// - If set to '[flex](https://platform.openai.com/docs/guides/flex-processing)' or '[priority](https://openai.com/api-priority-processing/)', then the request will be processed with the corresponding service tier.
    /// - When not set, the default behavior is 'auto'.
    ///
    /// When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// The status of the response generation.
    /// One of `completed`, `failed`, `in_progress`, `cancelled`, `queued`, or `incomplete`.
    pub status: Status,

    /// What sampling temperature was used, between 0 and 2. Higher values like 0.8 make
    /// outputs more random, lower values like 0.2 make output more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Configuration options for a text response from the model. Can be plain
    /// text or structured JSON data. Learn more:
    /// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
    /// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextParam>,

    /// How the model should select which tool (or tools) to use when generating
    /// a response. See the `tools` parameter to see how to specify which tools
    /// the model can call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceParam>,

    /// An array of tools the model may call while generating a response. You
    /// can specify which tool to use by setting the `tool_choice` parameter.
    ///
    /// We support the following categories of tools:
    /// - **Built-in tools**: Tools that are provided by OpenAI that extend the
    ///   model's capabilities, like [web search](https://platform.openai.com/docs/guides/tools-web-search)
    ///   or [file search](https://platform.openai.com/docs/guides/tools-file-search). Learn more about
    ///   [built-in tools](https://platform.openai.com/docs/guides/tools).
    /// - **MCP Tools**: Integrations with third-party systems via custom MCP servers
    ///   or predefined connectors such as Google Drive and SharePoint. Learn more about
    ///   [MCP Tools](https://platform.openai.com/docs/guides/tools-connectors-mcp).
    /// - **Function calls (custom tools)**: Functions that are defined by you,
    ///   enabling the model to call your own code with strongly typed arguments
    ///   and outputs. Learn more about
    ///   [function calling](https://platform.openai.com/docs/guides/function-calling). You can also use
    ///   custom tools to call your own code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each
    /// token position, each with an associated log probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability
    /// mass. So 0.1 means only the tokens comprising the top 10% probability mass
    /// are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    ///The truncation strategy to use for the model response.
    /// - `auto`: If the input to this Response exceeds
    ///   the model's context window size, the model will truncate the
    ///   response to fit the context window by dropping items from the beginning of the conversation.
    /// - `disabled` (default): If the input size will exceed the context window
    ///   size for a model, the request will fail with a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,

    /// Represents token usage details including input tokens, output tokens,
    /// a breakdown of output tokens, and the total tokens used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
}

impl Response {
    /// SDK-only convenience property that contains the aggregated text output from all
    /// `output_text` items in the `output` array, if any are present.
    pub fn output_text(&self) -> Option<String> {
        let output = self
            .output
            .iter()
            .filter_map(|item| match item {
                OutputItem::Message(msg) => Some(
                    msg.content
                        .iter()
                        .filter_map(|content| match content {
                            OutputMessageContent::OutputText(ot) => Some(ot.text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<String>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<String>>()
            .join("");
        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Completed,
    Failed,
    InProgress,
    Cancelled,
    Queued,
    Incomplete,
}

/// Output item
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteResponse {
    pub object: String,
    pub deleted: bool,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnyItemReference {
    pub r#type: Option<String>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemResourceItem {
    Message(MessageItem),
    FileSearchCall(FileSearchToolCall),
    ComputerCall(ComputerToolCall),
    ComputerCallOutput(ComputerCallOutputItemParam),
    WebSearchCall(WebSearchToolCall),
    FunctionCall(FunctionToolCall),
    FunctionCallOutput(FunctionCallOutputItemParam),
    ImageGenerationCall(ImageGenToolCall),
    CodeInterpreterCall(CodeInterpreterToolCall),
    LocalShellCall(LocalShellToolCall),
    LocalShellCallOutput(LocalShellToolCallOutput),
    McpListTools(MCPListTools),
    McpApprovalRequest(MCPApprovalRequest),
    McpApprovalResponse(MCPApprovalResponse),
    McpCall(MCPToolCall),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ItemResource {
    ItemReference(AnyItemReference),
    Item(ItemResourceItem),
}

/// A list of Response items.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseItemList {
    /// The type of object returned, must be `list`.
    pub object: String,
    /// The ID of the first item in the list.
    pub first_id: Option<String>,
    /// The ID of the last item in the list.
    pub last_id: Option<String>,
    /// Whether there are more items in the list.
    pub has_more: bool,
    /// The list of items.
    pub data: Vec<ItemResource>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder, PartialEq)]
#[builder(
    name = "TokenCountsBodyArgs",
    pattern = "mutable",
    setter(into, strip_option),
    default
)]
#[builder(build_fn(error = "OpenAIError"))]
pub struct TokenCountsBody {
    /// The conversation that this response belongs to. Items from this
    /// conversation are prepended to `input_items` for this response request.
    /// Input items and output items from this response are automatically added to this
    /// conversation after this response completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,

    /// Text, image, or file inputs to the model, used to generate a response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputParam>,

    /// A system (or developer) message inserted into the model's context.
    ///
    /// When used along with `previous_response_id`, the instructions from a previous response will
    /// not be carried over to the next response. This makes it simple to swap out system (or
    /// developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI offers a
    /// wide range of models with different capabilities, performance characteristics,
    /// and price points. Refer to the [model guide](https://platform.openai.com/docs/models)
    /// to browse and compare available models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// The unique ID of the previous response to the model. Use this to create multi-turn
    /// conversations. Learn more about [conversation state](https://platform.openai.com/docs/guides/conversation-state).
    /// Cannot be used in conjunction with `conversation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// **gpt-5 and o-series models only**
    /// Configuration options for [reasoning models](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,

    /// Configuration options for a text response from the model. Can be plain
    /// text or structured JSON data. Learn more:
    /// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
    /// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextParam>,

    /// How the model should select which tool (or tools) to use when generating
    /// a response. See the `tools` parameter to see how to specify which tools
    /// the model can call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceParam>,

    /// An array of tools the model may call while generating a response. You can specify which tool
    /// to use by setting the `tool_choice` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    ///The truncation strategy to use for the model response.
    /// - `auto`: If the input to this Response exceeds
    ///   the model's context window size, the model will truncate the
    ///   response to fit the context window by dropping items from the beginning of the conversation.
    /// - `disabled` (default): If the input size will exceed the context window
    ///   size for a model, the request will fail with a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokenCountsResource {
    pub object: String,
    pub input_tokens: u32,
}
