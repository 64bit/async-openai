#![allow(deprecated)]
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::images::{ImageGenInputUsageDetails, ImageGenOutputTokensDetails, ImageInput};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ImageSize {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1792x1024")]
    S1792x1024,
    #[serde(rename = "1024x1792")]
    S1024x1792,
    #[serde(rename = "1536x1024")]
    S1536x1024,
    #[serde(rename = "1024x1536")]
    S1024x1536,
    #[serde(untagged)]
    Other(String),
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DallE2ImageSize {
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    S1024x1024,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DallE3ImageSize {
    #[default]
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1792x1024")]
    S1792x1024,
    #[serde(rename = "1024x1792")]
    S1024x1792,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GptImage1ImageSize {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1536x1024")]
    S1536x1024,
    #[serde(rename = "1024x1536")]
    S1024x1536,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageResponseFormat {
    #[default]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum ImageModel {
    #[default]
    #[serde(rename = "gpt-image-2")]
    GptImage2,
    #[serde(rename = "gpt-image-2-2026-04-21")]
    GptImage2_2026_04_21,
    #[serde(rename = "gpt-image-2.5-sunburst")]
    GptImage2_5Sunburst,
    #[serde(rename = "gpt-image-2.5-sunburst-2026-09-08")]
    GptImage2_5Sunburst2026_09_08,
    #[serde(rename = "gpt-image-2.5-flare")]
    GptImage2_5Flare,
    #[serde(rename = "gpt-image-2.5-flare-2026-09-08")]
    GptImage2_5Flare2026_09_08,
    #[serde(rename = "gpt-image-1.5")]
    GptImage1dot5,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
    #[serde(rename = "chatgpt-image-latest")]
    ChatGptImageLatest,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    Standard,
    HD,
    High,
    Medium,
    Low,
    XHigh,
    Max,
    #[default]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    #[default]
    Vivid,
    Natural,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageModeration {
    #[default]
    Auto,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageOutputFormat {
    #[default]
    Png,
    Jpeg,
    Webp,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageBackground {
    #[default]
    Auto,
    Transparent,
    Opaque,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder, PartialEq)]
#[builder(name = "CreateImageRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 32000 characters.
    pub prompt: String,

    /// The GPT image model to use for image generation. Specify a model explicitly.
    /// Supported models include `gpt-image-1-mini`, `gpt-image-1.5`, `gpt-image-2`,
    /// `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,
    /// `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`,
    /// `gpt-image-2.5-flare-2026-09-08`, and `chatgpt-image-latest`.
    ///
    /// `gpt-image-1` was shut down on October 23, 2026. Use
    /// `gpt-image-2.5-sunburst` or `gpt-image-2.5-flare` instead.
    pub model: ImageModel,

    /// The number of images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The quality of the image that will be generated.
    ///
    /// - `auto` (default value) will automatically select the best quality for the given
    ///   model.
    /// - `high`, `medium` and `low` are supported for the GPT image models.
    /// - `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08`
    ///   snapshots, also support `xhigh` and `max`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// Legacy response format parameter for retired image models. Unsupported for GPT image models, which
    /// always return base64-encoded images.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(
        note = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."
    )]
    pub response_format: Option<ImageResponseFormat>,

    /// The format in which the generated images are returned. This parameter is only supported for
    /// the GPT image models. Must be one of `png`, `jpeg`, or `webp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageOutputFormat>,

    /// The compression level (0-100%) for the generated images. This parameter is only supported for
    /// the GPT image models with the `webp` or `jpeg` output formats, and defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,

    /// Generate the image in streaming mode. Defaults to `false`. See the
    /// [Image generation guide](https://platform.openai.com/docs/guides/image-generation) for more
    /// information. This parameter is only supported for the GPT image models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// The number of partial images to generate. This parameter is used for
    /// streaming responses that return partial images. Value must be between 0 and 3.
    /// When set to 0, the response will be a single image sent in one streaming event.
    /// Note that the final image may be sent before the full number of partial images
    /// are generated if the full image is generated more quickly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u8>,

    /// The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`,
    /// `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and
    /// `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for
    /// example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio
    /// must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum
    /// supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel
    /// and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the
    /// GPT image models; `auto` is supported for models that allow automatic sizing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Control the content-moderation level for images generated by the GPT image models. Must be either `low`
    /// for less restrictive filtering or `auto` (default value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ImageModeration>,

    /// Set the background of the generated image(s). This parameter is only
    /// supported for the GPT image models. Must be one of `transparent`, `opaque`,
    /// or `auto` (default value). When `auto` is used, the model will automatically
    /// determine the best background for the image.
    ///
    /// Transparent backgrounds are available for supported GPT Image models. For
    /// `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When
    /// using `transparent`, set the output format to `png` or `webp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageBackground>,

    /// Legacy style parameter for retired image models. Unsupported for GPT image models; describe the
    /// desired style in the prompt instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(
        note = "Legacy style parameter for retired image models. Unsupported for GPT image models; describe the desired style in the prompt instead."
    )]
    pub style: Option<ImageStyle>,

    /// Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward
    /// compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn
    /// more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = " Deprecated: use `safety_identifier` instead.")]
    pub user: Option<String>,

    /// A stable identifier for an end user in your organization, used for safety monitoring and abuse
    /// detection. Hash a username or email address to avoid sending identifying information. A non-empty
    /// value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API
    /// uses `user` when available. [Learn
    /// more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageResponseBackground {
    Transparent,
    Opaque,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ImageGenUsage {
    /// The number of tokens (images and text) in the input prompt.
    pub input_tokens: u32,
    /// The total number of tokens (images and text) used for the image generation.
    pub total_tokens: u32,
    /// The number of output tokens generated by the model.
    pub output_tokens: u32,
    /// The output token details for the image generation.
    pub output_token_details: Option<ImageGenOutputTokensDetails>,
    /// The input tokens detailed information for the image generation.
    pub input_tokens_details: ImageGenInputUsageDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ImagesResponse {
    /// The Unix timestamp (in seconds) of when the image was created.
    pub created: u32,
    /// The list of generated images.
    pub data: Vec<std::sync::Arc<Image>>,
    /// The background parameter used for the image generation. Either `transparent` or `opaque`.
    pub background: Option<ImageResponseBackground>,
    /// The output format of the image generation. Either `png`, `webp`, or `jpeg`.
    pub output_format: Option<ImageOutputFormat>,
    /// The size of the generated image as a `WIDTHxHEIGHT` string.
    pub size: Option<ImageSize>,
    /// The quality of the image generated. One of `low`, `medium`, `high`, `xhigh`, or `max`.
    pub quality: Option<ImageQuality>,
    /// For the GPT image models only, the token usage information for the image generation.
    pub usage: Option<ImageGenUsage>,
    /// The effective safety identifier used for safety monitoring and abuse detection. This is the
    /// non-empty `safety_identifier` from the request, or the deprecated `user` value when
    /// `safety_identifier` is omitted, null, or empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputFidelity {
    High,
    #[default]
    Low,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageEditInput {
    Image(ImageInput),
    Images(Vec<ImageInput>),
}

#[derive(Debug, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateImageEditRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageEditRequest {
    /// The image(s) to edit. Must be a supported image file or an array of images.
    ///
    /// For the GPT image models (`gpt-image-1-mini`, `gpt-image-1.5`,
    /// `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,
    /// `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and
    /// `gpt-image-2.5-flare-2026-09-08`), each image should be a `png`, `webp`, or `jpg`
    /// file less than 50MB. You can provide up to 16 images. `chatgpt-image-latest`
    /// follows the same input constraints as GPT image models.
    pub image: ImageEditInput,

    /// A text description of the desired image(s). The maximum length is 32000 characters for the GPT image
    /// models.
    pub prompt: String,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where
    /// `image` should be edited. If there are multiple images provided, the mask will be applied on the
    /// first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    pub mask: Option<ImageInput>,

    /// Set the background of the generated image(s). This parameter is only
    /// supported for the GPT image models. Must be one of `transparent`, `opaque`,
    /// or `auto` (default value). When `auto` is used, the model will automatically
    /// determine the best background for the image.
    ///
    /// Transparent backgrounds are available for supported GPT Image models. For
    /// `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When
    /// using `transparent`, set the output format to `png` or `webp`.
    pub background: Option<ImageBackground>,

    /// The GPT image model to use for image editing
    /// (`gpt-image-1-mini`, `gpt-image-1.5`, `gpt-image-2`,
    /// `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,
    /// `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`,
    /// `gpt-image-2.5-flare-2026-09-08`, or `chatgpt-image-latest`).
    ///
    /// `gpt-image-1` was shut down on October 23, 2026. Use
    /// `gpt-image-2.5-sunburst` or `gpt-image-2.5-flare` instead.
    pub model: ImageModel,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Defaults to `auto`. For `gpt-image-2`, `gpt-image-2-2026-04-21`,
    /// `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and
    /// `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for
    /// example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio
    /// must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum
    /// supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel
    /// and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the
    /// GPT image models; `auto` is supported for models that allow automatic sizing.
    pub size: Option<ImageSize>,

    /// Legacy response format parameter for retired image models. Unsupported for GPT image models, which
    /// always return base64-encoded images.
    #[deprecated(
        note = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."
    )]
    pub response_format: Option<ImageResponseFormat>,

    /// The format in which the generated images are returned. This parameter is
    /// only supported for the GPT image models. Must be one of `png`, `jpeg`, or `webp`.
    /// The default value is `png`.
    pub output_format: Option<ImageOutputFormat>,

    /// The compression level (0-100%) for the generated images. This parameter
    /// is only supported for the GPT image models with the `webp` or `jpeg` output
    /// formats, and defaults to 100.
    pub output_compression: Option<u8>,

    /// Deprecated: use `safety_identifier` instead. This parameter remains accepted for backward
    /// compatibility and is used when `safety_identifier` is omitted, null, or empty. [Learn
    /// more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).
    #[deprecated(note = "Deprecated: use `safety_identifier` instead.")]
    pub user: Option<String>,

    /// Control how much effort the model will exert to match the style and features, especially facial
    /// features, of input images. This parameter is only supported for the GPT image models. Unsupported for
    /// `gpt-image-1-mini`. Supports `high` and `low`. Defaults to `low`.
    pub input_fidelity: Option<InputFidelity>,

    /// Edit the image in streaming mode. Defaults to `false`. See the
    /// [Image generation guide](https://platform.openai.com/docs/guides/image-generation) for more
    /// information.
    pub stream: Option<bool>,

    /// The number of partial images to generate. This parameter is used for
    /// streaming responses that return partial images. Value must be between 0 and 3.
    /// When set to 0, the response will be a single image sent in one streaming event.

    /// Note that the final image may be sent before the full number of partial images
    /// are generated if the full image is generated more quickly.
    pub partial_images: Option<u8>,

    /// The quality of the image that will be generated. GPT image models support
    /// quality levels through `max`; `dall-e-2` supports `standard`. Defaults to `auto`.
    pub quality: Option<ImageQuality>,

    /// A stable identifier for an end user in your organization, used for safety monitoring and abuse
    /// detection. Hash a username or email address to avoid sending identifying information. A non-empty
    /// value takes precedence over the deprecated `user` parameter. If omitted, null, or empty, the API
    /// uses `user` when available. [Learn
    /// more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).
    pub safety_identifier: Option<String>,

    /// A transparent PNG overlaid on the first image to guide the edit. Must match that image's dimensions
    /// after orientation and contain transparent pixels. Supports GPT Image 2.5. Cannot be combined with
    /// mask.
    pub markup: Option<ImageInput>,
}

#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateImageVariationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
#[deprecated(note = "DALL-E 2 and image variations shut down on February 26, 2027.")]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and
    /// square.
    pub image: ImageInput,

    /// The legacy model used by the retired image variations endpoint. This endpoint no longer accepts
    /// requests.
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs
    /// are only valid for 60 minutes after the image has been generated.
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<DallE2ImageSize>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
    pub user: Option<String>,
}

/// Reference an input image by either URL or uploaded file ID.
/// Provide exactly one of `image_url` or `file_id`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ImageRefParam {
    /// A fully qualified URL or base64-encoded data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The File API ID of an uploaded image to use as input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

/// JSON request body for image edits.
///
/// Use `images` (array of `ImageRefParam`) instead of multipart `image` uploads.
/// You can reference images via external URLs, data URLs, or uploaded file IDs.
/// JSON edits support GPT image models only; DALL-E edits require multipart (`dall-e-2` only).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder, Default)]
#[builder(name = "EditImageJsonRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct EditImageJsonRequest {
    /// The model to use for image editing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ImageModel>,

    /// Input image references to edit. For GPT image models, you can provide up to 16 images.
    pub images: Vec<ImageRefParam>,

    /// An optional mask image reference indicating which areas of the image should be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<ImageRefParam>,

    /// A text description of the desired image edit.
    pub prompt: String,

    /// The number of edited images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,

    /// The quality of the image that will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// Control how much effort the model will exert to match the style and features,
    /// especially facial features, of input images. Supports `high` and `low`. Defaults to `low`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_fidelity: Option<InputFidelity>,

    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use safety_identifier instead.")]
    pub user: Option<String>,

    /// The output format for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageOutputFormat>,

    /// The compression level (0-100%) for the generated images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,

    /// Control the content-moderation level for images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ImageModeration>,

    /// The background style for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageBackground>,

    /// Whether to stream the image generation. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// The number of partial images to generate during streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u8>,

    /// Opaque end-user identifier, up to 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// An optional markup image containing edit instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<ImageRefParam>,
}
