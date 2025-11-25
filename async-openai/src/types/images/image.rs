use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::images::ImageInput;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
    #[default]
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    DallE3,
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
    /// A text description of the desired image(s). The maximum length is 32000 characters for
    /// `gpt-image-1`, 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    pub prompt: String,

    /// The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults
    /// to `dall-e-2` unless a parameter specific to `gpt-image-1` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The quality of the image that will be generated.
    ///
    /// - `auto` (default value) will automatically select the best quality for the given model.
    /// - `high`, `medium` and `low` are supported for `gpt-image-1`.
    /// - `hd` and `standard` are supported for `dall-e-3`.
    /// - `standard` is the only option for `dall-e-2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,

    /// The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of
    /// `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This
    /// parameter isn't supported for `gpt-image-1` which will always return base64-encoded images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The format in which the generated images are returned. This parameter is only supported for
    /// `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageOutputFormat>,

    /// The compression level (0-100%) for the generated images. This parameter is only supported for
    /// `gpt-image-1` with the `webp` or `jpeg` output formats, and defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,

    /// Generate the image in streaming mode. Defaults to `false`. See the
    /// [Image generation guide](https://platform.openai.com/docs/guides/image-generation) for more
    /// information. This parameter is only supported for `gpt-image-1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// The number of partial images to generate. This parameter is used for
    /// streaming responses that return partial images. Value must be between 0 and 3.
    /// When set to 0, the response will be a single image sent in one streaming event.
    /// Note that the final image may be sent before the full number of partial images
    /// are generated if the full image is generated more quickly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u8>,

    /// The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536`
    /// (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or
    /// `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low`
    /// for less restrictive filtering or `auto` (default value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<ImageModeration>,

    /// Allows to set transparency for the background of the generated image(s).
    /// This parameter is only supported for `gpt-image-1`. Must be one of
    /// `transparent`, `opaque` or `auto` (default value). When `auto` is used, the
    /// model will automatically determine the best background for the image.
    /// If `transparent`, the output format needs to support transparency, so it
    /// should be set to either `png` (default value) or `webp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageBackground>,

    /// The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of
    ///`vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic
    /// images. Natural causes the model to produce more natural, less hyper-real looking images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    ///[Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImageGenInputUsageDetails {
    /// The number of text tokens in the input prompt.
    pub text_tokens: u32,
    /// The number of image tokens in the input prompt.
    pub image_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImageGenUsage {
    /// The number of tokens (images and text) in the input prompt.
    pub input_tokens: u32,
    /// The total number of tokens (images and text) used for the image generation.
    pub total_tokens: u32,
    /// The number of output tokens generated by the model.
    pub output_tokens: u32,
    /// The input tokens detailed information for the image generation.
    pub input_tokens_details: ImageGenInputUsageDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ImagesResponse {
    /// The Unix timestamp (in seconds) of when the image was created.
    pub created: u32,
    /// The list of generated images.
    pub data: Vec<std::sync::Arc<Image>>,
    /// The background parameter used for the image generation. Either `transparent` or `opaque`.
    pub background: Option<ImageResponseBackground>,
    /// The output format of the image generation. Either `png`, `webp`, or `jpeg`.
    pub output_format: Option<ImageOutputFormat>,
    /// The size of the generated image. Either `1024x1024`, `1536x1024`, `1024x1536`.
    pub size: Option<ImageSize>,
    /// The quality of the image generated. Either `low`, `medium`, or `high`.
    pub quality: Option<ImageQuality>,
    /// For `gpt-image-1` only, the token usage information for the image generation.
    pub usage: Option<ImageGenUsage>,
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
    /// For `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less
    /// than 50MB. You can provide up to 16 images.
    ///
    /// For `dall-e-2`, you can only provide one image, and it should be a square
    /// `png` file less than 4MB.
    pub image: ImageEditInput,

    /// A text description of the desired image(s). The maximum length is 1000 characters
    /// for `dall-e-2`, and 32000 characters for `gpt-image-1`.
    pub prompt: String,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where
    /// `image` should be edited. If there are multiple images provided, the mask will be applied on the
    /// first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    pub mask: Option<ImageInput>,

    /// Allows to set transparency for the background of the generated image(s).
    /// This parameter is only supported for `gpt-image-1`. Must be one of
    /// `transparent`, `opaque` or `auto` (default value). When `auto` is used, the
    /// model will automatically determine the best background for the image.
    ///
    /// If `transparent`, the output format needs to support transparency, so it
    /// should be set to either `png` (default value) or `webp`.
    pub background: Option<ImageBackground>,

    /// The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported.
    /// Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used.
    pub model: Option<ImageModel>,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: Option<u8>, // min:1 max:10 default:1

    /// The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape),
    /// `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`,
    /// `512x512`, or `1024x1024` for `dall-e-2`.
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs
    /// are only valid for 60 minutes after the image has been generated. This parameter is only supported
    /// for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images.
    pub response_format: Option<ImageResponseFormat>,

    /// The format in which the generated images are returned. This parameter is
    /// only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`.
    /// The default value is `png`.
    pub output_format: Option<ImageOutputFormat>,

    /// The compression level (0-100%) for the generated images. This parameter
    /// is only supported for `gpt-image-1` with the `webp` or `jpeg` output
    /// formats, and defaults to 100.
    pub output_compression: Option<u8>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices#end-user-ids).
    pub user: Option<String>,

    /// Control how much effort the model will exert to match the style and features, especially facial
    /// features, of input images. This parameter is only supported for `gpt-image-1`. Unsupported for
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

    /// The quality of the image that will be generated. `high`, `medium` and `low` are only supported for
    /// `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.
    pub quality: Option<ImageQuality>,
}

#[derive(Debug, Default, Clone, Builder, PartialEq)]
#[builder(name = "CreateImageVariationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and
    /// square.
    pub image: ImageInput,

    /// The model to use for image generation. Only `dall-e-2` is supported at this time.
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
