use std::{collections::HashMap, path::PathBuf, pin::Pin};

use bytes::Bytes;
use derive_builder::Builder;
use futures::Stream;
use reqwest::Body;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Prompt {
    String(String),
    StringArray(Vec<String>),
    // Minimum value is 0, maximum value is 50256 (inclusive).
    IntegerArray(Vec<u16>),
    ArrayOfIntegerArray(Vec<Vec<u16>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Stop {
    String(String),           // nullable: true
    StringArray(Vec<String>), // minItems: 1; maxItems: 4
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

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateCompletionRequest {
    /// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models/overview) for descriptions of them.
    pub model: String,

    /// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.
    ///
    /// Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.
    pub prompt: Prompt,

    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>, // default: null

    /// The maximum number of [tokens](/tokenizer) to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min: 0, max: 2, default: 1,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///  We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1

    /// How many completions to generate for each prompt.

    /// **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max: 128, default: 1

    /// Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a `data: [DONE]` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>, // nullable: true

    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

    /// The maximum value for `logprobs` is 5. If you need more than this, please contact us through our [Help center](https://help.openai.com) and describe your use case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>, // min:0 , max: 5, default: null, nullable: true

    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    ///  Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0

    /// Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.
    ///
    /// When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.
    ///
    /// **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u8>, //min: 0, max: 20, default: 1

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    ///
    /// As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>, // default: null

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.
    ///
    /// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
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
    pub logprobs: Option<Logprobs>,
    pub finish_reason: Option<CompletionFinishReason>,
}

/// Usage statistics for the completion request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CompletionUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the generated completion.
    pub completion_tokens: u32,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateCompletionResponse {
    /// A unique identifier for the completion.
    pub id: String,
    pub choices: Vec<Choice>,
    /// The Unix timestamp (in seconds) of when the completion was created.
    pub created: u32,

    /// The model used for completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been
    /// made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always "text_completion"
    pub object: String,
    pub usage: Option<CompletionUsage>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type CompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateCompletionResponse, OpenAIError>> + Send>>;

#[derive(Debug, Clone, Serialize, Default, Builder, PartialEq)]
#[builder(name = "CreateEditRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEditRequest {
    /// ID of the model to use. You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: String,

    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>, // default ''

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,

    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max: 20 default:1

    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min:0 ,max: 2, default: 1,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///  We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateEditResponse {
    pub object: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: CompletionUsage,
}

//// Start Fine Tuning

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(untagged)]
pub enum NEpochs {
    NEpochs(u8),
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Hyperparameters {
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    ///
    /// "auto" decides the optimal number of epochs based on the size of the dataset.
    /// If setting the number manually, we support any number between 1 and 50 epochs.
    pub n_epochs: NEpochs,
}

#[derive(Debug, Serialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuningJobRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFineTuningJobRequest {
    /// The name of the model to fine-tune. You can select one of the
    /// [supported models](https://platform.openai.com/docs/guides/fine-tuning/what-models-can-be-fine-tuned).
    pub model: String,

    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    pub training_file: String,

    /// The hyperparameters used for the fine-tuning job.
    pub hyperparameters: Option<Hyperparameters>,

    /// A string of up to 18 characters that will be added to your fine-tuned model name.
    ///
    /// For example, a `suffix` of "custom-model-name" would produce a model name
    /// like `ft:gpt-3.5-turbo:openai:custom-model-name:7p4lURel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>, // default: null, minLength:1, maxLength:40

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation
    /// metrics periodically during fine-tuning. These metrics can be viewed in
    /// the fine-tuning results file.
    /// The same data should not be present in both train and validation files.
    ///
    /// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

/// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneJobError {
    ///  A machine-readable error code.
    pub code: String,
    ///  A human-readable error message.
    pub message: String,
    /// The parameter that was invalid, usually `training_file` or `validation_file`.
    /// This field will be null if the failure was not parameter-specific.
    pub param: Option<String>, // nullable true
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningJobStatus {
    ValidatingFiles,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: u32,
    /// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
    pub error: Option<FineTuneJobError>,
    /// The name of the fine-tuned model that is being created.
    /// The value will be null if the fine-tuning job is still running.
    pub fine_tuned_model: Option<String>, // nullable: true
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished.
    /// The value will be null if the fine-tuning job is still running.
    pub finished_at: Option<u32>, // nullable true

    /// The hyperparameters used for the fine-tuning job.
    /// See the [fine-tuning guide](/docs/guides/fine-tuning) for more details.
    pub hyperparameters: Hyperparameters,

    ///  The base model that is being fine-tuned.
    pub model: String,

    /// The object type, which is always "fine_tuning.job".
    pub object: String,
    /// The organization that owns the fine-tuning job.
    pub organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    /// You can retrieve the results with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub result_files: Vec<String>,

    /// The current status of the fine-tuning job, which can be either
    /// `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`.
    pub status: FineTuningJobStatus,

    /// The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running.
    pub trained_tokens: Option<u32>,

    /// The file ID used for training. You can retrieve the training data with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub training_file: String,

    ///  The file ID used for validation. You can retrieve the validation results with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub validation_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListPaginatedFineTuningJobsResponse {
    pub data: Vec<FineTuningJob>,
    pub has_more: bool,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningJobEventsResponse {
    pub data: Vec<FineTuningJobEvent>,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Info,
    Warn,
    Error,
}

///Fine-tuning job event object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningJobEvent {
    pub id: String,
    pub created_at: u32,
    pub level: Level,
    pub message: String,
    pub object: String,
}

//// End Fine Tuning

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteModelResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum EmbeddingInput {
    String(String),
    StringArray(Vec<String>),
    // Minimum value is 0, maximum value is 100257 (inclusive).
    IntegerArray(Vec<u32>),
    ArrayOfIntegerArray(Vec<Vec<u32>>),
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    #[default]
    Float,
    Base64,
}

#[derive(Debug, Serialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateEmbeddingRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEmbeddingRequest {
    /// ID of the model to use. You can use the
    /// [List models](https://platform.openai.com/docs/api-reference/models/list)
    /// API to see all of your available models, or see our
    /// [Model overview](https://platform.openai.com/docs/models/overview)
    /// for descriptions of them.
    pub model: String,

    /// Input text to embed, encoded as a string or array of tokens. To embed multiple
    ///  inputs in a single request, pass an array of strings or array of token arrays.
    ///  The input must not exceed the max input tokens for the model (8192 tokens for
    ///  `text-embedding-ada-002`) and cannot be an empty string.
    /// [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.
    pub input: EmbeddingInput,

    /// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/). Defaults to float
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,

    /// A unique identifier representing your end-user, which will help OpenAI
    ///  to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub index: u32,
    /// The object type, which is always "embedding".
    pub object: String,
    /// The embedding vector, which is a list of floats. The length of vector
    /// depends on the model as listed in the [embedding guide](https://platform.openai.com/docs/guides/embeddings).
    pub embedding: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmbeddingUsage {
    /// The number of tokens used by the prompt.
    pub prompt_tokens: u32,
    /// The total number of tokens used by the request.
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateEmbeddingResponse {
    pub object: String,
    /// The name of the model used to generate the embedding.
    pub model: String,
    /// The list of embeddings generated by the model.
    pub data: Vec<Embedding>,
    /// The usage information for the request.
    pub usage: EmbeddingUsage,
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

/// The name and arguments of a function that should be called, as generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FunctionCall {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestSystemMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestSystemMessage {
    /// The contents of the system message.
    pub content: Option<String>,
    /// The role of the messages author, in this case `system`.
    #[builder(default = "Role::System")]
    pub role: Role,
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
    #[builder(default = "\"text\".into()")]
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageUrlDetail {
    #[default]
    Auto,
    Low,
    High,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ImageUrlArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ImageUrl {
    /// Either a URL of the image or the base64 encoded image data.
    pub url: String,
    /// Specifies the detail level of the image.
    pub detail: ImageUrlDetail,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageContentPartImageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessageContentPartImage {
    #[builder(default = "\"image_url\".into()")]
    pub r#type: String,
    pub image_url: ImageUrl,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Image(ChatCompletionRequestMessageContentPartImage),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    /// The text contents of the message.
    Text(String),
    ///  An array of content parts with a defined type, each can be of type `text` or `image_url`
    /// when passing in images. You can pass multiple images by adding multiple `image_url` content parts.
    ///  Image input is only supported when using the `gpt-4-visual-preview` model.
    Array(Vec<ChatCompletionRequestMessageContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestUserMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    pub content: Option<ChatCompletionRequestUserMessageContent>,
    /// The role of the messages author, in this case `user`.
    #[builder(default = "Role::User")]
    pub role: Role,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestAssistantMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestAssistantMessage {
    /// The contents of the assistant message.
    pub content: Option<String>,
    /// The role of the messages author, in this case `assistant`.
    #[builder(default = "Role::Assistant")]
    pub role: Role,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
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
    /// The role of the messages author, in this case `tool`.
    #[builder(default = "Role::Tool")]
    pub role: Role,
    /// The contents of the tool message.
    pub content: Option<String>,
    pub tool_call_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestFunctionMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestFunctionMessage {
    /// The role of the messages author, in this case `function`.
    #[builder(default = "Role::Function")]
    pub role: Role,
    /// The return value from the function call, to return to the model.
    pub content: Option<String>,
    /// The name of the function to call.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessage {
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    Assistant(ChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: ChatCompletionToolType,
    /// The function that the model called.
    pub function: FunctionCall,
}

/// A chat completion message generated by the model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessage {
    /// The contents of the message.
    pub content: Option<String>,

    /// The tool calls generated by the model, such as function calls.
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,

    /// The role of the author of this message.
    pub role: Role,

    /// Deprecated and replaced by `tool_calls`.
    /// The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    pub function_call: Option<FunctionCall>,
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
    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [guide](https://platform.openai.com/docs/guides/gpt/function-calling) for examples,
    /// and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for
    /// documentation about the format.
    ///
    /// To describe a function that accepts no parameters, provide the
    /// value `{\"type\": \"object\", \"properties\": {}}`.
    pub parameters: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionResponseFormatType {
    Text,
    JsonObject,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseFormat {
    /// Setting to `json_object` enables JSON mode. This guarantees that the message the model generates is valid JSON.
    ///
    /// Note that your system prompt must still instruct the model to produce JSON, and to help ensure you don't forget,
    /// the API will throw an error if the string `JSON` does not appear in your system message. Also note that the message
    /// content may be partial (i.e. cut off) if `finish_reason="length"`, which indicates the generation
    /// exceeded `max_tokens` or the conversation exceeded the max context length.
    ///
    /// Must be one of `text` or `json_object`.
    pub r#type: ChatCompletionResponseFormatType,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolType {
    #[default]
    Function,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "ChatCompletionToolArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionTool {
    #[builder(default = "ChatCompletionToolType::Function")]
    pub r#type: ChatCompletionToolType,
    pub function: ChatCompletionFunctions,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}

/// Specifies a tool the model should use. Use to force the model to call a specific function.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
pub struct ChatCompletionNamedToolChoice {
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: ChatCompletionToolType,

    pub function: FunctionName,
}

/// Controls which (if any) function is called by the model.
/// `none` means the model will not call a function and instead generates a message.
/// `auto` means the model can pick between generating a message or calling a function.
/// Specifying a particular function via `{"type: "function", "function": {"name": "my_function"}}` forces the model to call that function.

/// `none` is the default when no functions are present. `auto` is the default if functions are present.
#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceOption {
    #[default]
    None,
    Auto,
    #[serde(untagged)]
    Named(ChatCompletionNamedToolChoice),
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "CreateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateChatCompletionRequest {
    /// A list of messages comprising the conversation so far. [Example Python code](https://cookbook.openai.com/examples/how_to_format_inputs_to_chatgpt_models).
    pub messages: Vec<ChatCompletionRequestMessage>, // min: 1

    /// ID of the model to use.
    /// See the [model endpoint compatibility](https://platform.openai.com/docs/models/model-endpoint-compatibility) table for details on which models work with the Chat API.
    pub model: String,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>, // default: null

    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the model's context length. [Example Python code](https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb) for counting tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0

    /// An object specifying the format that the model must output.
    ///
    /// Setting to `{ "type": "json_object" }` enables JSON mode, which guarantees the message the model generates is valid JSON.
    ///
    /// **Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in increased latency and appearance of a "stuck" request. Also note that the message content may be partially cut off if `finish_reason="length"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ChatCompletionResponseFormat>,

    ///  This feature is in Beta.
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests
    /// with the same `seed` and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

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

    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Controls how the model responds to function calls.
    /// "none" means the model does not call a function, and responds to the end-user.
    /// "auto" means the model can pick between an end-user or calling a function.
    /// Specifying a particular function via `{"name":\ "my_function"}` forces the model to call that function.
    /// "none" is the default when no functions are present. "auto" is the default if functions are present.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,

    /// A list of functions the model may generate JSON inputs for.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<ChatCompletionFunctions>>,
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
pub struct ChatChoice {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub message: ChatCompletionResponseMessage,
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    pub finish_reason: Option<FinishReason>,
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
    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: String,
    pub usage: Option<CompletionUsage>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type ChatCompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>>;

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
    pub index: i32,
    /// The ID of the tool call.
    pub id: Option<String>,
    /// The type of the tool. Currently, only `function` is supported.
    pub r#type: Option<ChatCompletionToolType>,
    pub function: Option<FunctionCallStream>,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionStreamResponseDelta {
    /// The contents of the chunk message.
    pub content: Option<String>,
    /// The name and arguments of a function that should be called, as generated by the model.
    #[deprecated]
    pub function_call: Option<FunctionCallStream>,

    pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
    /// The role of the author of this message.
    pub role: Option<Role>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseStreamMessage {
    /// The index of the choice in the list of choices.
    pub index: u32,
    pub delta: ChatCompletionStreamResponseDelta,
    pub finish_reason: Option<FinishReason>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
/// Represents a streamed chunk of a chat completion response returned by model, based on the provided input.
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if `n` is greater than 1.
    pub choices: Vec<ChatCompletionResponseStreamMessage>,

    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: u32,
    /// The model to generate the completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,
    /// The object type, which is always `chat.completion.chunk`.
    pub object: String,
}
