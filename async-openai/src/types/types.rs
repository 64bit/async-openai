use std::collections::HashMap;
#[cfg(feature = "tokio")]
use std::path::PathBuf;

use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::client::OpenAIEventStream;

use crate::error::OpenAIError;

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The object type, which is always "model".
    pub object: String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u32,
    /// The organization that owns the model.
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
pub type CompletionResponseStream = OpenAIEventStream<CreateCompletionResponse>;

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

#[cfg(feature = "tokio")]
#[derive(Default, Debug, Serialize, Clone, Copy, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1792x1024")]
    S1792x1024,
    #[serde(rename = "1024x1792")]
    S1024x1792,
}

#[derive(Default, Debug, Serialize, Clone, Copy, PartialEq)]
pub enum DallE2ImageSize {
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    S1024x1024,
}

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
    #[default]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
pub enum ImageModel {
    #[default]
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    DallE3,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    #[default]
    Standard,
    HD,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    #[default]
    Vivid,
    Natural,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Clone, Serialize, Default, Builder, PartialEq)]
#[builder(name = "CreateImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`
    /// and 4000 characters for `dall-e-3`.
    pub prompt: String,

    /// The model to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The quality of the image that will be generated. `hd` creates images with finer details and greater
    /// consistency across the image. This param is only supported for `dall-e-3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
    /// Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// The style of the generated images. Must be one of `vivid` or `natural`.
    /// Vivid causes the model to lean towards generating hyper-real and dramatic images.
    /// Natural causes the model to produce more natural, less hyper-real looking images.
    /// This param is only supported for `dall-e-3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Image {
    /// The URL of the generated image, if `response_format` is `url` (default).
    Url {
        url: String,
        revised_prompt: Option<String>,
    },
    /// The base64-encoded JSON of the generated image, if `response_format` is `b64_json`.
    B64Json {
        b64_json: std::sync::Arc<String>,
        revised_prompt: Option<String>,
    },
}

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImagesResponse {
    pub created: u32,
    pub data: Vec<std::sync::Arc<Image>>,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageInput {
    pub path: PathBuf,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateImageEditRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageEditRequest {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square. If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: ImageInput,

    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    pub mask: Option<ImageInput>,

    /// The model to use for image generation. Only `dall-e-2` is supported at this time.
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<DallE2ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    pub response_format: Option<ResponseFormat>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/usage-policies/end-user-ids).
    pub user: Option<String>,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateImageVariationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
    pub image: ImageInput,

    /// The model to use for image generation. Only `dall-e-2` is supported at this time.
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<DallE2ImageSize>,

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

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
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
    /// Content that expresses, incites, or promotes hate based on race, gender,
    /// ethnicity, religion, nationality, sexual orientation, disability status, or
    /// caste. Hateful content aimed at non-protected groups (e.g., chess players)
    /// is harrassment.
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    /// Hateful content that also includes violence or serious harm towards the
    /// targeted group based on race, gender, ethnicity, religion, nationality,
    /// sexual orientation, disability status, or caste.
    pub hate_threatening: bool,
    /// Content that expresses, incites, or promotes harassing language towards any target.
    pub harassment: bool,
    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    /// Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    /// Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    /// Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
    pub sexual: bool,
    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    /// Content that depicts death, violence, or physical injury.
    pub violence: bool,
    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

/// A list of the categories along with their scores as predicted by model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CategoryScore {
    /// The score for the category 'hate'.
    pub hate: f32,
    /// The score for the category 'hate/threatening'.
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f32,
    /// The score for the category 'harassment'.
    pub harassment: f32,
    /// The score for the category 'harassment/threatening'.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f32,
    /// The score for the category 'self-harm'.
    #[serde(rename = "self-harm")]
    pub self_harm: f32,
    /// The score for the category 'self-harm/intent'.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f32,
    /// The score for the category 'self-harm/instructions'.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f32,
    /// The score for the category 'sexual'.
    pub sexual: f32,
    /// The score for the category 'sexual/minors'.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f32,
    /// The score for the category 'violence'.
    pub violence: f32,
    /// The score for the category 'violence/graphic'.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContentModerationResult {
    /// Whether the content violates [OpenAI's usage policies](https://platform.openai.com/policies/usage-policies).
    pub flagged: bool,
    /// A list of the categories, and whether they are flagged or not.
    pub categories: Category,
    /// A list of the categories along with their scores as predicted by model.
    pub category_scores: CategoryScore,
}

/// Represents policy compliance report by OpenAI's content moderation model against a given input.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateModerationResponse {
    /// The unique identifier for the moderation request.
    pub id: String,
    /// The model used to generate the moderation results.
    pub model: String,
    /// A list of moderation objects.
    pub results: Vec<ContentModerationResult>,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FileInput {
    pub path: PathBuf,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateFileRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFileRequest {
    /// The file object to be uploaded.
    ///
    /// If the `purpose` is set to "fine-tune", the file will be used for fine-tuning.
    pub file: FileInput,

    /// The intended purpose of the uploaded file.
    ///
    /// Use "fine-tune" for [fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning).
    /// This allows us to validate the format of the uploaded file is correct for fine-tuning.
    pub purpose: String,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFilesResponse {
    pub object: String,
    pub data: Vec<OpenAIFile>,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DeleteFileResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum OpenAIFilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
}

/// The `File` object represents a document that has been uploaded to OpenAI.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OpenAIFile {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The object type, which is always "file".
    pub object: String,
    /// The size of the file in bytes.
    pub bytes: u32,
    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: u32,
    /// The name of the file.
    pub filename: String,
    /// The intended purpose of the file. Supported values are `fine-tune`, `fine-tune-results`, `assistants`, and `assistants_output`.
    pub purpose: OpenAIFilePurpose,
    /// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
    #[deprecated]
    pub status: Option<String>,
    /// Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`.
    #[deprecated]
    pub status_details: Option<String>, // nullable: true
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

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneEventsStreamResponse {
    pub object: String,
    pub data: Option<Vec<FineTuneEvent>>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type FineTuneEventsResponseStream = OpenAIEventStream<ListFineTuneEventsStreamResponse>;

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
pub type ChatCompletionResponseStream = OpenAIEventStream<CreateChatCompletionStreamResponse>;

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

#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AudioInput {
    pub path: PathBuf,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SpeechResponseFormat {
    #[default]
    Mp3,
    Opus,
    Aac,
    Flac,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Voice {
    #[default]
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub enum SpeechModel {
    #[default]
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(untagged)]
    Other(String),
}

#[cfg(feature = "tokio")]
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

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateTranscriptionResponse {
    pub text: String,
}

#[derive(Clone, Default, Debug, Builder, PartialEq, Serialize)]
#[builder(name = "CreateSpeechRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateSpeechRequest {
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,

    /// One of the available [TTS models](https://platform.openai.com/docs/models/tts): `tts-1` or `tts-1-hd`
    pub model: SpeechModel,

    /// The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`.
    pub voice: Voice,

    /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,

    /// The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>, // default: 1.0
}

#[cfg(feature = "tokio")]
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

#[cfg(feature = "tokio")]
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateTranslationResponse {
    pub text: String,
}

#[cfg(feature = "tokio")]
#[derive(Debug, Clone)]
pub struct CreateSpeechResponse {
    pub bytes: Bytes,
}
