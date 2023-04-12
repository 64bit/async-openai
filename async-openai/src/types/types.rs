use std::{collections::HashMap, path::PathBuf, pin::Pin};

use derive_builder::Builder;
use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListModelResponse {
    pub object: String,
    pub data: Vec<Model>,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,

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
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Logprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<Option<f32>>, // Option is to account for null value in the list
    pub top_logprobs: Vec<serde_json::Value>,
    pub text_offset: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<Logprobs>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
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
    pub usage: Usage,
}

#[derive(Default, Debug, Serialize, Clone, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    S1024x1024,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
    #[default]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}

#[derive(Debug, Clone, Serialize, Default, Builder, PartialEq)]
#[builder(name = "CreateImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageData {
    Url(std::sync::Arc<String>),
    #[serde(rename = "b64_json")]
    B64Json(std::sync::Arc<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImageResponse {
    pub created: u32,
    pub data: Vec<std::sync::Arc<ImageData>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageInput {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateImageEditRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageEditRequest {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square. If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: ImageInput,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    pub mask: Option<ImageInput>,

    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    pub response_format: Option<ResponseFormat>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    pub user: Option<String>,
}

#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateImageVariationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
    pub image: ImageInput,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    pub response_format: Option<ResponseFormat>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationInput {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
pub enum TextModerationModel {
    #[default]
    #[serde(rename = "text-moderation-latest")]
    Latest,
    #[serde(rename = "text-moderation-stable")]
    Stable,
}

#[derive(Debug, Default, Clone, Serialize, Builder, PartialEq)]
#[builder(name = "CreateModerationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateModerationRequest {
    /// The input text to classify
    pub input: ModerationInput,

    /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.
    ///
    /// The default is `text-moderation-latest` which will be automatically upgraded over time. This ensures you are always using our most accurate model. If you use `text-moderation-stable`, we will provide advanced notice before updating the model. Accuracy of `text-moderation-stable` may be slightly lower than for `text-moderation-latest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<TextModerationModel>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Category {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CategoryScore {
    pub hate: f32,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f32,
    #[serde(rename = "self-harm")]
    pub self_harm: f32,
    pub sexual: f32,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f32,
    pub violence: f32,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContentModerationResult {
    pub flagged: bool,
    pub categories: Category,
    pub category_scores: CategoryScore,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ContentModerationResult>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileInput {
    pub path: PathBuf,
}

#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateFileRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFileRequest {
    /// Name of the [JSON Lines](https://jsonlines.readthedocs.io/en/latest/) file to be uploaded.
    ///
    /// If the `purpose` is set to "fine-tune", each line is a JSON record with "prompt" and "completion" fields representing your [training examples](https://platform.openai.com/docs/guides/fine-tuning/prepare-training-data).
    pub file: FileInput,

    /// The intended purpose of the uploaded documents.
    ///
    /// Use "fine-tune" for [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tunes). This allows us to validate the format of the uploaded file.
    pub purpose: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFilesResponse {
    pub object: String,
    pub data: Vec<OpenAIFile>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OpenAIFile {
    pub id: String,
    pub object: String,
    pub bytes: u32,
    pub created_at: u32,
    pub filename: String,
    pub purpose: String,
    pub status: Option<String>,
    pub status_details: Option<serde_json::Value>, // nullable: true
}

#[derive(Debug, Serialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuneRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFineTuneRequest {
    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file, where each training
    /// example is a JSON object with the keys "prompt" and "completion".
    /// Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning/creating-training-data) for more details.
    pub training_file: String,

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation
    /// metrics periodically during fine-tuning. These metrics can be viewed in
    /// the [fine-tuning results file](https://platform.openai.com/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).
    /// Your train and validation data should be mutually exclusive.
    ///
    /// Your dataset must be formatted as a JSONL file, where each validation
    /// example is a JSON object with the keys "prompt" and "completion".
    /// Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning/creating-training-data) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    /// The name of the base model to fine-tune. You can select one of "ada",
    /// "babbage", "curie", "davinci", or a fine-tuned model created after 2022-04-21.
    /// To learn more about these models, see the [Models](https://platform.openai.com/docs/models) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The number of epochs to train the model for. An epoch refers to one
    /// full cycle through the training dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<u32>, // default: 4

    /// The batch size to use for training. The batch size is the number of
    /// training examples used to train a single forward and backward pass.
    ///
    /// By default, the batch size will be dynamically configured to be
    /// ~0.2% of the number of examples in the training set, capped at 256 -
    /// in general, we've found that larger batch sizes tend to work better
    /// for larger datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>, // default: null

    /// The learning rate multiplier to use for training.
    /// The fine-tuning learning rate is the original learning rate used for
    /// pretraining multiplied by this value.
    ///
    /// By default, the learning rate multiplier is the 0.05, 0.1, or 0.2
    /// depending on final `batch_size` (larger learning rates tend to
    /// perform better with larger batch sizes). We recommend experimenting
    /// with values in the range 0.02 to 0.2 to see what produces the best
    /// results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<f32>, // default: null

    /// The weight to use for loss on the prompt tokens. This controls how
    /// much the model tries to learn to generate the prompt (as compared
    /// to the completion which always has a weight of 1.0), and can add
    /// a stabilizing effect to training when completions are short.
    ///
    /// If prompts are extremely long (relative to completions), it may make
    /// sense to reduce this weight so as to avoid over-prioritizing
    /// learning the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_loss_weight: Option<f32>, // default: 0.01

    /// If set, we calculate classification-specific metrics such as accuracy
    /// and F-1 score using the validation set at the end of every epoch.
    /// These metrics can be viewed in the [results file](https://platform.openai.com/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).
    ///
    /// In order to compute classification metrics, you must provide a
    /// `validation_file`. Additionally, you must
    /// specify `classification_n_classes` for multiclass classification or
    /// `classification_positive_class` for binary classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_classification_metrics: Option<bool>, // default: false

    /// The number of classes in a classification task.
    ///
    /// This parameter is required for multiclass classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_n_classes: Option<u32>, // default: null

    /// The positive class in binary classification.
    ///
    /// This parameter is needed to generate precision, recall, and F1
    /// metrics when doing binary classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_positive_class: Option<String>, // default: null

    /// If this is provided, we calculate F-beta scores at the specified
    /// beta values. The F-beta score is a generalization of F-1 score.
    /// This is only used for binary classification.
    ///
    /// With a beta of 1 (i.e. the F-1 score), precision and recall are
    /// given the same weight. A larger beta score puts more weight on
    /// recall and less on precision. A smaller beta score puts more weight
    /// on precision and less on recall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_betas: Option<Vec<f32>>, // default: null

    /// A string of up to 40 characters that will be added to your fine-tuned model name.
    ///
    /// For example, a `suffix` of "custom-model-name" would produce a model name like `ada:ft-your-org:custom-model-name-2022-02-15-04-21-04`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>, // default: null, minLength:1, maxLength:40
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneResponse {
    pub object: String,
    pub data: Vec<FineTune>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTune {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub updated_at: u32,
    pub model: String,
    pub fine_tuned_model: Option<String>, // nullable: true
    pub organization_id: String,
    pub status: String,
    pub hyperparams: serde_json::Value,
    pub training_files: Vec<OpenAIFile>,
    pub validation_files: Vec<OpenAIFile>,
    pub result_files: Vec<OpenAIFile>,
    pub events: Option<Vec<FineTuneEvent>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneEvent {
    pub object: String,
    pub created_at: u32,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneEventsResponse {
    pub object: String,
    pub data: Vec<FineTuneEvent>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type FineTuneEventsResponseStream =
    Pin<Box<dyn Stream<Item = Result<ListFineTuneEventsResponse, OpenAIError>> + Send>>;

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

    /// Input text to get embeddings for, encoded as a string or array of tokens.
    /// To get embeddings for multiple inputs in a single request, pass an array
    /// of strings or array of token arrays. Each input must not exceed 8192
    /// tokens in length.
    pub input: EmbeddingInput,

    /// A unique identifier representing your end-user, which will help OpenAI
    ///  to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Embedding {
    pub index: u32,
    pub object: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateEmbeddingResponse {
    pub object: String,
    pub model: String,
    pub data: Vec<Embedding>,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder, PartialEq)]
#[builder(name = "ChatCompletionRequestMessageArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ChatCompletionRequestMessage {
    /// The role of the author of this message.
    pub role: Role,
    /// The contents of the message
    pub content: String,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Serialize, Default, Debug, Builder, Deserialize, PartialEq)]
#[builder(name = "CreateChatCompletionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateChatCompletionRequest {
    /// ID of the model to use. Currently, only `gpt-3.5-turbo` and `gpt-3.5-turbo-0301` are supported.
    pub model: String,

    /// The messages to generate chat completions for, in the [chat format](https://platform.openai.com/docs/guides/chat/introduction).
    pub messages: Vec<ChatCompletionRequestMessage>, // min: 1

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

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1

    /// If set, partial message deltas will be sent, like in ChatGPT. Tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// The maximum number of tokens allowed for the generated answer. By default, the number of tokens the model can return will be (4096 - prompt tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>, // default: inf

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

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>, // default: null

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatCompletionResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type ChatCompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>>;

// For reason (not documented by OpenAI) the response from stream is different

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatCompletionResponseStreamMessage {
    pub content: Option<String>,
    pub role: Option<Role>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatChoiceDelta {
    pub index: u32,
    pub delta: ChatCompletionResponseStreamMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateChatCompletionStreamResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AudioInput {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateTranscriptionRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateTranscriptionRequest {
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg, mpga, m4a, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. Only `whisper-1` is currently available.
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text/prompting) should match the audio language.
    pub prompt: Option<String>,

    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: Option<AudioResponseFormat>,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0

    /// The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format will improve accuracy and latency.
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponse {
    pub text: String,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateTranslationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateTranslationRequest {
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg, mpga, m4a, wav, or webm.
    pub file: AudioInput,

    /// ID of the model to use. Only `whisper-1` is currently available.
    pub model: String,

    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](https://platform.openai.com/docs/guides/speech-to-text/prompting) should be in English.
    pub prompt: Option<String>,

    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: Option<AudioResponseFormat>,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.
    pub temperature: Option<f32>, // default: 0
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateTranslationResponse {
    pub text: String,
}
