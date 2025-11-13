use serde::{Deserialize, Serialize};

/// Represents a ChatKit thread and its current status.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ThreadResource {
    /// Identifier of the thread.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread`.
    #[serde(default = "default_thread_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the thread was created.
    pub created_at: u64,
    /// Optional human-readable title for the thread. Defaults to null when no title has been generated.
    pub title: Option<String>,
    /// Current status for the thread. Defaults to `active` for newly created threads.
    #[serde(flatten)]
    pub status: ThreadStatus,
    /// Free-form string that identifies your end user who owns the thread.
    pub user: String,
    /// Thread items (only present when retrieving a thread)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ThreadItemListResource>,
}

fn default_thread_object() -> String {
    "chatkit.thread".to_string()
}

/// Current status for the thread.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThreadStatus {
    /// Indicates that a thread is active.
    Active,
    /// Indicates that a thread is locked and cannot accept new input.
    Locked { reason: Option<String> },
    /// Indicates that a thread has been closed.
    Closed { reason: Option<String> },
}

/// A paginated list of ChatKit threads.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct ThreadListResource {
    /// The type of object returned, must be `list`.
    #[serde(default = "default_list_object")]
    pub object: String,
    /// A list of items
    pub data: Vec<ThreadResource>,
    /// The ID of the first item in the list.
    pub first_id: Option<String>,
    /// The ID of the last item in the list.
    pub last_id: Option<String>,
    /// Whether there are more items available.
    pub has_more: bool,
}

fn default_list_object() -> String {
    "list".to_string()
}

/// Confirmation payload returned after deleting a thread.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct DeletedThreadResource {
    /// Identifier of the deleted thread.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread.deleted`.
    #[serde(default = "default_deleted_object")]
    pub object: String,
    /// Indicates that the thread has been deleted.
    pub deleted: bool,
}

fn default_deleted_object() -> String {
    "chatkit.thread.deleted".to_string()
}

/// A paginated list of thread items rendered for the ChatKit API.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct ThreadItemListResource {
    /// The type of object returned, must be `list`.
    #[serde(default = "default_list_object")]
    pub object: String,
    /// A list of items
    pub data: Vec<ThreadItem>,
    /// The ID of the first item in the list.
    pub first_id: Option<String>,
    /// The ID of the last item in the list.
    pub last_id: Option<String>,
    /// Whether there are more items available.
    pub has_more: bool,
}

/// The thread item - discriminated union based on type field.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThreadItem {
    /// User-authored messages within a thread.
    #[serde(rename = "chatkit.user_message")]
    UserMessage(UserMessageItem),
    /// Assistant-authored message within a thread.
    #[serde(rename = "chatkit.assistant_message")]
    AssistantMessage(AssistantMessageItem),
    /// Thread item that renders a widget payload.
    #[serde(rename = "chatkit.widget")]
    Widget(WidgetMessageItem),
    /// Record of a client side tool invocation initiated by the assistant.
    #[serde(rename = "chatkit.client_tool_call")]
    ClientToolCall(ClientToolCallItem),
    /// Task emitted by the workflow to show progress and status updates.
    #[serde(rename = "chatkit.task")]
    Task(TaskItem),
    /// Collection of workflow tasks grouped together in the thread.
    #[serde(rename = "chatkit.task_group")]
    TaskGroup(TaskGroupItem),
}

/// User-authored messages within a thread.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct UserMessageItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Ordered content elements supplied by the user.
    pub content: Vec<UserMessageContent>,
    /// Attachments associated with the user message. Defaults to an empty list.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Inference overrides applied to the message. Defaults to null when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_options: Option<InferenceOptions>,
}

fn default_thread_item_object() -> String {
    "chatkit.thread_item".to_string()
}

/// Content blocks that comprise a user message.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserMessageContent {
    /// Text block that a user contributed to the thread.
    #[serde(rename = "input_text")]
    InputText { text: String },
    /// Quoted snippet that the user referenced in their message.
    #[serde(rename = "quoted_text")]
    QuotedText { text: String },
}

/// Attachment metadata included on thread items.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct Attachment {
    /// Attachment discriminator.
    #[serde(rename = "type")]
    pub attachment_type: AttachmentType,
    /// Identifier for the attachment.
    pub id: String,
    /// Original display name for the attachment.
    pub name: String,
    /// MIME type of the attachment.
    pub mime_type: String,
    /// Preview URL for rendering the attachment inline.
    pub preview_url: Option<String>,
}

/// Attachment discriminator.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentType {
    Image,
    File,
}

/// Model and tool overrides applied when generating the assistant response.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct InferenceOptions {
    /// Preferred tool to invoke. Defaults to null when ChatKit should auto-select.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Model name that generated the response. Defaults to null when using the session default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Tool selection that the assistant should honor when executing the item.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct ToolChoice {
    /// Identifier of the requested tool.
    pub id: String,
}

/// Assistant-authored message within a thread.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct AssistantMessageItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Ordered assistant response segments.
    pub content: Vec<ResponseOutputText>,
}

/// Assistant response text accompanied by optional annotations.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ResponseOutputText {
    /// Type discriminator that is always `output_text`.
    #[serde(default = "default_output_text_type")]
    pub r#type: String,
    /// Assistant generated text.
    pub text: String,
    /// Ordered list of annotations attached to the response text.
    #[serde(default)]
    pub annotations: Vec<Annotation>,
}

fn default_output_text_type() -> String {
    "output_text".to_string()
}

/// Annotation object describing a cited source.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    /// Annotation that references an uploaded file.
    #[serde(rename = "file")]
    File(FileAnnotation),
    /// Annotation that references a URL.
    #[serde(rename = "url")]
    Url(UrlAnnotation),
}

/// Annotation that references an uploaded file.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct FileAnnotation {
    /// Type discriminator that is always `file` for this annotation.
    #[serde(default = "default_file_annotation_type")]
    pub r#type: String,
    /// File attachment referenced by the annotation.
    pub source: FileAnnotationSource,
}

fn default_file_annotation_type() -> String {
    "file".to_string()
}

/// Attachment source referenced by an annotation.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct FileAnnotationSource {
    /// Type discriminator that is always `file`.
    #[serde(default = "default_file_source_type")]
    pub r#type: String,
    /// Filename referenced by the annotation.
    pub filename: String,
}

fn default_file_source_type() -> String {
    "file".to_string()
}

/// Annotation that references a URL.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct UrlAnnotation {
    /// Type discriminator that is always `url` for this annotation.
    #[serde(default = "default_url_annotation_type")]
    pub r#type: String,
    /// URL referenced by the annotation.
    pub source: UrlAnnotationSource,
}

fn default_url_annotation_type() -> String {
    "url".to_string()
}

/// URL backing an annotation entry.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct UrlAnnotationSource {
    /// Type discriminator that is always `url`.
    #[serde(default = "default_url_source_type")]
    pub r#type: String,
    /// URL referenced by the annotation.
    pub url: String,
}

fn default_url_source_type() -> String {
    "url".to_string()
}

/// Thread item that renders a widget payload.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct WidgetMessageItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Serialized widget payload rendered in the UI.
    pub widget: String,
}

/// Record of a client side tool invocation initiated by the assistant.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ClientToolCallItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Execution status for the tool call.
    pub status: ClientToolCallStatus,
    /// Identifier for the client tool call.
    pub call_id: String,
    /// Tool name that was invoked.
    pub name: String,
    /// JSON-encoded arguments that were sent to the tool.
    pub arguments: String,
    /// JSON-encoded output captured from the tool. Defaults to null while execution is in progress.
    pub output: Option<String>,
}

/// Execution status for the tool call.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClientToolCallStatus {
    InProgress,
    Completed,
}

/// Task emitted by the workflow to show progress and status updates.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct TaskItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Subtype for the task.
    pub task_type: TaskType,
    /// Optional heading for the task. Defaults to null when not provided.
    pub heading: Option<String>,
    /// Optional summary that describes the task. Defaults to null when omitted.
    pub summary: Option<String>,
}

/// Subtype for the task.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Custom,
    Thought,
}

/// Collection of workflow tasks grouped together in the thread.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct TaskGroupItem {
    /// Identifier of the thread item.
    pub id: String,
    /// Type discriminator that is always `chatkit.thread_item`.
    #[serde(default = "default_thread_item_object")]
    pub object: String,
    /// Unix timestamp (in seconds) for when the item was created.
    pub created_at: u64,
    /// Identifier of the parent thread.
    pub thread_id: String,
    /// Tasks included in the group.
    pub tasks: Vec<TaskGroupTask>,
}

/// Task entry that appears within a TaskGroup.
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct TaskGroupTask {
    /// Subtype for the grouped task.
    pub task_type: TaskType,
    /// Optional heading for the grouped task. Defaults to null when not provided.
    pub heading: Option<String>,
    /// Optional summary that describes the grouped task. Defaults to null when omitted.
    pub summary: Option<String>,
}
