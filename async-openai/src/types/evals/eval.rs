use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::chat::{ChatCompletionTool, ImageDetail, InputAudio, ResponseFormat};
use crate::types::graders::{
    GraderLabelModel, GraderPython, GraderScoreModel, GraderStringCheck, GraderTextSimilarity,
};
use crate::types::responses::{ResponseTextParam, Tool};
use crate::types::Metadata;

// Re-export commonly used types
pub use crate::types::responses::{EasyInputMessage, InputTextContent, ReasoningEffort};

/// An Eval object with a data source config and testing criteria.
/// An Eval represents a task to be done for your LLM integration.
/// Like:
/// - Improve the quality of my chatbot
/// - See how well my chatbot handles customer support
/// - Check if o4-mini is better at my usecase than gpt-4o
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Eval {
    /// The object type, which is always "eval".
    pub object: String,
    /// Unique identifier for the evaluation.
    pub id: String,
    /// The name of the evaluation.
    pub name: String,
    /// Configuration of data sources used in runs of the evaluation.
    pub data_source_config: EvalDataSourceConfig,
    /// A list of testing criteria.
    pub testing_criteria: Vec<EvalTestingCriterion>,
    /// The Unix timestamp (in seconds) for when the eval was created.
    pub created_at: u64,
    pub metadata: Metadata,
}

/// Configuration of data sources used in runs of the evaluation.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalDataSourceConfig {
    /// Custom data source config.
    Custom(EvalCustomDataSourceConfig),
    /// Logs data source config.
    Logs(EvalLogsDataSourceConfig),
    /// Stored completions data source config (deprecated).
    #[serde(rename = "stored_completions")]
    StoredCompletions(EvalStoredCompletionsDataSourceConfig),
}

/// Custom data source config.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalCustomDataSourceConfig {
    /// The type of data source. Always "custom".
    #[serde(rename = "type")]
    pub r#type: String,
    /// The json schema for the run data source items.
    pub schema: serde_json::Value,
}

/// Logs data source config.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalLogsDataSourceConfig {
    /// The type of data source. Always "logs".
    #[serde(rename = "type")]
    pub r#type: String,
    /// Metadata filters for the logs data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The json schema for the run data source items.
    pub schema: serde_json::Value,
}

/// Stored completions data source config (deprecated).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalStoredCompletionsDataSourceConfig {
    /// The type of data source. Always "stored_completions".
    #[serde(rename = "type")]
    pub r#type: String,
    /// Metadata filters for the stored completions data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The json schema for the run data source items.
    pub schema: serde_json::Value,
}

/// A list of testing criteria.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalTestingCriterion {
    /// Label model grader.
    LabelModel(EvalGraderLabelModel),
    /// String check grader.
    StringCheck(EvalGraderStringCheck),
    /// Text similarity grader.
    TextSimilarity(EvalGraderTextSimilarity),
    /// Python grader.
    Python(EvalGraderPython),
    /// Score model grader.
    ScoreModel(EvalGraderScoreModel),
}

/// Label model grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct EvalGraderLabelModel(pub GraderLabelModel);

/// String check grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct EvalGraderStringCheck(pub GraderStringCheck);

/// Text similarity grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalGraderTextSimilarity {
    #[serde(flatten)]
    pub grader: GraderTextSimilarity,
    pub pass_threshold: f64,
}

/// Text similarity metric.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextSimilarityMetric {
    /// Cosine similarity.
    Cosine,
    /// Fuzzy match.
    FuzzyMatch,
    /// BLEU score.
    Bleu,
    /// GLEU score.
    Gleu,
    /// METEOR score.
    Meteor,
    /// ROUGE-1.
    Rouge1,
    /// ROUGE-2.
    Rouge2,
    /// ROUGE-3.
    Rouge3,
    /// ROUGE-4.
    Rouge4,
    /// ROUGE-5.
    Rouge5,
    /// ROUGE-L.
    RougeL,
}

/// Python grader.
/// also in openapi spec: GraderPython
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalGraderPython {
    #[serde(flatten)]
    pub grader: GraderPython,
    pub pass_threshold: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SamplingParams {
    /// A seed value to initialize the randomness, during sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// An alternative to temperature for nucleus sampling; 1.0 includes all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// A higher temperature increases randomness in the outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// The maximum number of tokens the grader model may generate in its response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    /// Optional reasoning effort parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

/// Score model grader.
/// also in openapi spec: GraderScoreModel
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalGraderScoreModel {
    #[serde(flatten)]
    pub grader: GraderScoreModel,
    /// The threshold for the score.
    pub pass_threshold: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalItem {
    /// The role of the message input. One of `user`, `assistant`, `system`, or
    /// `developer`.
    pub role: EvalItemRole,
    /// Inputs to the model - can contain template strings.
    pub content: EvalItemContent,
}

/// The role of the message input.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EvalItemRole {
    /// User role.
    User,
    /// Assistant role.
    Assistant,
    /// System role.
    System,
    /// Developer role.
    Developer,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OutputText {
    /// The text output from the model.
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct InputImage {
    /// The URL of the image input.
    pub image_url: String,
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`.
    /// Defaults to `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Inputs to the model - can contain template strings.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalItemContent {
    /// An input text content object.
    InputText(InputTextContent),
    /// An output text from the model.
    OutputText(OutputText),
    /// An image input to the model.
    InputImage(InputImage),
    /// An audio input to the model.
    InputAudio(InputAudio),
    /// An array of Input text, Input image, and Input audio
    Array(Vec<EvalItemContent>),
    #[serde(untagged)]
    /// A text input to the model.
    Text(String),
}

/// List of evals.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalList {
    /// The object type, which is always "list".
    pub object: String,
    /// An array of eval objects.
    pub data: Vec<Eval>,
    /// The identifier of the first eval in the data array.
    pub first_id: String,
    /// The identifier of the last eval in the data array.
    pub last_id: String,
    /// Indicates whether there are more evals available.
    pub has_more: bool,
}

#[derive(Debug, Serialize, Clone, Builder, PartialEq, Default)]
#[builder(name = "CreateEvalRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEvalRequest {
    /// The name of the evaluation.
    pub name: Option<String>,
    ///The configuration for the data source used for the evaluation runs.
    /// Dictates the schema of the data used in the evaluation.
    pub data_source_config: CreateEvalDataSourceConfig,
    /// A list of graders for all eval runs in this group. Graders can reference variables in the data
    /// source using double curly braces notation, like `{{item.variable_name}}`. To reference the model's
    /// output, use the `sample` namespace (ie, `{{sample.output_text}}`).
    pub testing_criteria: Vec<CreateEvalTestingCriterion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateEvalDataSourceConfig {
    /// A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation
    /// runs. This schema is used to define the shape of the data that will be:
    /// - Used to define your testing criteria and
    /// - What data is required when creating a run
    Custom(CreateEvalCustomDataSourceConfig),
    /// A data source config which specifies the metadata property of your logs query.
    /// This is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.
    Logs(CreateEvalLogsDataSourceConfig),
}

impl Default for CreateEvalDataSourceConfig {
    fn default() -> Self {
        Self::Custom(CreateEvalCustomDataSourceConfig::default())
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "CreateEvalCustomDataSourceConfigArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEvalCustomDataSourceConfig {
    /// The json schema for each row in the data source.
    pub item_schema: serde_json::Value,
    /// Whether the eval should expect you to populate the sample namespace (ie, by generating responses
    /// off of your data source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_sample_schema: Option<bool>,
}

/// Logs data source config for creating an eval.
#[derive(Debug, Serialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "CreateEvalLogsDataSourceConfigArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEvalLogsDataSourceConfig {
    /// Metadata filters for the logs data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateEvalTestingCriterion {
    /// A LabelModelGrader object which uses a model to assign labels to each item
    /// in the evaluation.
    LabelModel(CreateEvalLabelModelGrader),
    /// A StringCheckGrader object that performs a string comparison between input and reference using a
    /// specified operation.
    StringCheck(EvalGraderStringCheck),
    /// Text similarity grader.
    TextSimilarity(EvalGraderTextSimilarity),
    /// Python grader.
    Python(EvalGraderPython),
    /// Score model grader.
    ScoreModel(EvalGraderScoreModel),
}

/// Label model grader for creating an eval.
#[derive(Debug, Serialize, Clone, PartialEq, Builder, Default)]
#[builder(name = "CreateEvalLabelModelGraderArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEvalLabelModelGrader {
    /// The name of the grader.
    pub name: String,
    /// The model to use for the evaluation. Must support structured outputs.
    pub model: String,
    /// A list of chat messages forming the prompt or context. May include variable references to the
    /// `item` namespace, ie `{{item.name}}`.
    pub input: Vec<CreateEvalItem>,
    /// The labels to classify to each item in the evaluation.
    pub labels: Vec<String>,
    /// The labels that indicate a passing result. Must be a subset of labels.
    pub passing_labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SimpleInputMessage {
    /// The role of the message.
    pub role: String,
    /// The content of the message.
    pub content: String,
}

/// A chat message that makes up the prompt or context.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateEvalItem {
    /// A message input to the model with a role indicating instruction following
    /// hierarchy. Instructions given with the `developer` or `system` role take
    /// precedence over instructions given with the `user` role. Messages with the
    /// `assistant` role are presumed to have been generated by the model in previous
    /// interactions.
    Message(EvalItem),

    /// SimpleInputMessage
    #[serde(untagged)]
    Simple(SimpleInputMessage),
}

/// Request to update an eval.
#[derive(Debug, Serialize, Clone, Builder, PartialEq, Default)]
#[builder(name = "UpdateEvalRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct UpdateEvalRequest {
    /// Rename the evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Metadata attached to the eval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Response from deleting an eval.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeleteEvalResponse {
    /// The object type, which is always "eval.deleted".
    pub object: String,
    /// Whether the eval was deleted.
    pub deleted: bool,
    /// The ID of the deleted eval.
    pub eval_id: String,
}

// EvalRun types

/// A schema representing an evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRun {
    /// The object type, which is always "eval.run".
    pub object: String,
    /// Unique identifier for the evaluation run.
    pub id: String,
    /// The identifier of the associated evaluation.
    pub eval_id: String,
    /// The status of the evaluation run.
    pub status: EvalRunStatus,
    /// The model that is evaluated, if applicable.
    pub model: String,
    /// The name of the evaluation run.
    pub name: String,
    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub created_at: u64,
    /// The URL to the rendered evaluation run report on the UI dashboard.
    pub report_url: String,
    /// Counters summarizing the outcomes of the evaluation run.
    pub result_counts: EvalRunResultCounts,
    /// Usage statistics for each model during the evaluation run.
    pub per_model_usage: Option<Vec<EvalRunModelUsage>>,
    /// Results per testing criteria applied during the evaluation run.
    pub per_testing_criteria_results: Option<Vec<EvalRunTestingCriteriaResult>>,
    /// Information about the run's data source.
    pub data_source: EvalRunDataSource,
    /// Metadata attached to the run.
    pub metadata: Metadata,
    /// Error information, if any.
    pub error: Option<EvalApiError>,
}

/// Status of an evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvalRunStatus {
    /// Queued.
    Queued,
    /// In progress.
    InProgress,
    /// Completed.
    Completed,
    /// Failed.
    Failed,
    /// Canceled.
    Canceled,
}

/// Counters summarizing the outcomes of the evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunResultCounts {
    /// Total number of executed output items.
    pub total: u32,
    /// Number of output items that resulted in an error.
    pub errored: u32,
    /// Number of output items that failed to pass the evaluation.
    pub failed: u32,
    /// Number of output items that passed the evaluation.
    pub passed: u32,
}

/// Usage statistics for each model during the evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunModelUsage {
    /// The name of the model.
    pub model_name: String,
    /// The number of invocations.
    pub invocation_count: u32,
    /// The number of prompt tokens used.
    pub prompt_tokens: u32,
    /// The number of completion tokens generated.
    pub completion_tokens: u32,
    /// The total number of tokens used.
    pub total_tokens: u32,
    /// The number of tokens retrieved from cache.
    pub cached_tokens: u32,
}

/// Results per testing criteria applied during the evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunTestingCriteriaResult {
    /// A description of the testing criteria.
    pub testing_criteria: String,
    /// Number of tests passed for this criteria.
    pub passed: u32,
    /// Number of tests failed for this criteria.
    pub failed: u32,
}

/// Information about the run's data source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalRunDataSource {
    /// A JsonlRunDataSource object with that specifies a JSONL file that matches the eval
    Jsonl(CreateEvalJsonlRunDataSource),
    /// A CompletionsRunDataSource object describing a model sampling configuration.
    Completions(CreateEvalCompletionsRunDataSource),
    /// A ResponsesRunDataSource object describing a model sampling configuration.
    Responses(CreateEvalResponsesRunDataSource),
}

/// JSONL run data source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateEvalJsonlRunDataSource {
    /// Determines what populates the `item` namespace in the data source.
    pub source: EvalJsonlSource,
}

/// JSONL source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalJsonlSource {
    /// File content source.
    FileContent(EvalJsonlFileContentSource),
    /// File ID source.
    FileId(EvalJsonlFileIdSource),
}

/// JSONL file content source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalJsonlFileContentSource {
    /// The content of the jsonl file.
    pub content: Vec<EvalJsonlContentItem>,
}

/// JSONL file ID source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalJsonlFileIdSource {
    /// The identifier of the file.
    pub id: String,
}

/// JSONL content item.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalJsonlContentItem {
    /// The item data.
    pub item: serde_json::Value,
    /// The sample data, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<serde_json::Value>,
}

/// Completions run data source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateEvalCompletionsRunDataSource {
    /// Used when sampling from a model. Dictates the structure of the messages passed into the model. Can
    /// either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with
    /// variable references to the `item` namespace.
    pub input_messages: EvalInputMessages,
    /// The sampling parameters for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_params: Option<EvalSamplingParams>,
    /// The name of the model to use for generating completions (e.g. "o3-mini").
    pub model: String,
    /// Determines what populates the `item` namespace in this run's data source.
    pub source: EvalCompletionsSource,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TemplateInputMessages {
    /// A list of chat messages forming the prompt or context. May include variable references to
    /// the `item` namespace, ie {{item.name}}.
    pub template: Vec<CreateEvalItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ItemReference {
    /// A reference to a variable in the `item` namespace. Ie, "item.input_trajectory"
    pub item_reference: String,
}

/// Input messages for completions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalInputMessages {
    /// Template input messages.
    Template(TemplateInputMessages),
    /// Item reference input messages.
    ItemReference(ItemReference),
}

/// Sampling parameters for the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct EvalSamplingParams {
    /// A seed value to initialize the randomness, during sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// An alternative to temperature for nucleus sampling; 1.0 includes all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// A higher temperature increases randomness in the outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// The maximum number of tokens in the generated output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    /// Optional reasoning effort parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct EvalResponsesSamplingParams {
    /// A seed value to initialize the randomness, during sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// An alternative to temperature for nucleus sampling; 1.0 includes all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// A higher temperature increases randomness in the outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// The maximum number of tokens in the generated output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Optional reasoning effort parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Configuration options for a text response from the model. Can be plain
    /// text or structured JSON data. Learn more:
    /// - [Text inputs and outputs](https://platform.openai.com/docs/guides/text)
    /// - [Structured Outputs](https://platform.openai.com/docs/guides/structured-outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextParam>,
}

/// Completions source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalCompletionsSource {
    /// File content source.
    FileContent(EvalJsonlFileContentSource),
    /// File ID source.
    FileId(EvalJsonlFileIdSource),
    /// Stored completions source.
    StoredCompletions(EvalStoredCompletionsSource),
}

/// Stored completions source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalStoredCompletionsSource {
    /// Metadata filters for the stored completions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// An optional model to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// An optional Unix timestamp to filter items created after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<u64>,
    /// An optional Unix timestamp to filter items created before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<u64>,
    /// An optional maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Responses run data source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateEvalResponsesRunDataSource {
    /// Used when sampling from a model. Dictates the structure of the messages passed into the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_messages: Option<EvalInputMessages>,
    /// The sampling parameters for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_params: Option<EvalResponsesSamplingParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Determines what populates the `item` namespace in this run's data source.
    pub source: EvalResponsesRunSource,
}

/// Responses source.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvalResponsesRunSource {
    /// File content source.
    FileContent(EvalJsonlFileContentSource),
    /// File ID source.
    FileId(EvalJsonlFileIdSource),
    /// A EvalResponsesSource object describing a run data source configuration.
    Responses(EvalResponsesSource),
}

/// A EvalResponsesSource object describing a run data source configuration.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalResponsesSource {
    /// Metadata filter for the responses. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// The name of the model to find responses for. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional string to search the 'instructions' field. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_search: Option<String>,
    /// Only include items created after this timestamp (inclusive). This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<u64>,
    /// Only include items created before this timestamp (inclusive). This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<u64>,
    /// Optional reasoning effort parameter. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Sampling temperature. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling parameter. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// List of user identifiers. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// List of tool names. This is a query parameter used to select responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
}

/// List of eval runs.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunList {
    /// The object type, which is always "list".
    pub object: String,
    /// An array of eval run objects.
    pub data: Vec<EvalRun>,
    /// The identifier of the first eval run in the data array.
    pub first_id: String,
    /// The identifier of the last eval run in the data array.
    pub last_id: String,
    /// Indicates whether there are more evals available.
    pub has_more: bool,
}

/// Request to create an eval run.
#[derive(Debug, Serialize, Clone, Builder, PartialEq, Default)]
#[builder(name = "CreateEvalRunRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEvalRunRequest {
    /// The name of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details about the run's data source.
    pub data_source: CreateEvalRunDataSource,
    /// Metadata attached to the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Details about the run's data source.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateEvalRunDataSource {
    /// JSONL data source.
    Jsonl(CreateEvalJsonlRunDataSource),
    /// Completions data source.
    Completions(CreateEvalCompletionsRunDataSource),
    /// Responses data source.
    Responses(CreateEvalResponsesRunDataSource),
}

// Manual Default implementation for Builder compatibility
impl Default for CreateEvalRunDataSource {
    fn default() -> Self {
        todo!()
    }
}

/// Response from deleting an eval run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeleteEvalRunResponse {
    /// The object type, which is always "eval.run.deleted".
    pub object: String,
    /// Whether the eval run was deleted.
    pub deleted: bool,
    /// The ID of the deleted eval run.
    pub run_id: String,
}

// EvalRunOutputItem types

/// A schema representing an evaluation run output item.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunOutputItem {
    /// The object type, which is always "eval.run.output_item".
    pub object: String,
    /// Unique identifier for the evaluation run output item.
    pub id: String,
    /// The identifier of the evaluation run associated with this output item.
    pub run_id: String,
    /// The identifier of the evaluation group.
    pub eval_id: String,
    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub created_at: u64,
    /// The status of the evaluation run.
    pub status: String,
    /// The identifier for the data source item.
    pub datasource_item_id: u64,
    /// Details of the input data source item.
    pub datasource_item: serde_json::Value,
    /// A list of grader results for this output item.
    pub results: Vec<EvalRunOutputItemResult>,
    /// A sample containing the input and output of the evaluation run.
    pub sample: EvalRunOutputItemSample,
}

/// A single grader result for an evaluation run output item.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunOutputItemResult {
    /// The name of the grader.
    pub name: String,
    /// The numeric score produced by the grader.
    pub score: f64,
    /// Whether the grader considered the output a pass.
    pub passed: bool,
    /// Optional sample or intermediate data produced by the grader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SimpleOutputMessage {
    pub role: String,
    pub content: String,
}

/// A sample containing the input and output of the evaluation run.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunOutputItemSample {
    /// An array of input messages.
    pub input: Vec<SimpleInputMessage>,
    /// An array of output messages.
    pub output: Vec<SimpleOutputMessage>,
    /// The reason why the sample generation was finished.
    pub finish_reason: String,
    /// The model used for generating the sample.
    pub model: String,
    /// Token usage details for the sample.
    pub usage: EvalRunOutputItemUsage,
    /// Error information, if any.
    pub error: Option<EvalApiError>,
    /// The sampling temperature used.
    pub temperature: f64,
    /// The maximum number of tokens allowed for completion.
    pub max_completion_tokens: i32,
    /// The top_p value used for sampling.
    pub top_p: f64,
    /// The seed used for generating the sample.
    pub seed: i32,
}

/// Token usage details for the sample.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunOutputItemUsage {
    /// The total number of tokens used.
    pub total_tokens: i32,
    /// The number of completion tokens generated.
    pub completion_tokens: i32,
    /// The number of prompt tokens used.
    pub prompt_tokens: i32,
    /// The number of tokens retrieved from cache.
    pub cached_tokens: i32,
}

/// List of eval run output items.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalRunOutputItemList {
    /// The object type, which is always "list".
    pub object: String,
    /// An array of eval run output item objects.
    pub data: Vec<EvalRunOutputItem>,
    /// The identifier of the first eval run output item in the data array.
    pub first_id: String,
    /// The identifier of the last eval run output item in the data array.
    pub last_id: String,
    /// Indicates whether there are more eval run output items available.
    pub has_more: bool,
}

/// An object representing an error response from the Eval API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EvalApiError {
    /// The error code.
    pub code: String,
    /// The error message.
    pub message: String,
}
