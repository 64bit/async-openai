use serde::{Deserialize, Serialize};

use crate::types::images::{
    ImageBackground, ImageGenUsage, ImageOutputFormat, ImageQuality, ImageSize,
};

/// Emitted when a partial image is available during image generation streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenPartialImageEvent {
    /// Base64-encoded partial image data, suitable for rendering as an image.
    pub b64_json: String,
    /// The Unix timestamp when the event was created.
    pub created_at: u64,
    /// The size of the requested image.
    pub size: ImageSize,
    /// The quality setting for the requested image.
    pub quality: ImageQuality,
    /// The background setting for the requested image.
    pub background: ImageBackground,
    /// The output format for the requested image.
    pub output_format: ImageOutputFormat,
    /// 0-based index for the partial image (streaming).
    pub partial_image_index: u8,
}

/// Emitted when image generation has completed and the final image is available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenCompletedEvent {
    /// Base64-encoded image data, suitable for rendering as an image.
    pub b64_json: String,
    /// The Unix timestamp when the event was created.
    pub created_at: u64,
    /// The size of the generated image.
    pub size: ImageSize,
    /// The quality setting for the generated image.
    pub quality: ImageQuality,
    /// The background setting for the generated image.
    pub background: ImageBackground,
    /// The output format for the generated image.
    pub output_format: ImageOutputFormat,
    /// Token usage information for the image generation.
    pub usage: ImageGenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageGenStreamEvent {
    /// Emitted when a partial image is available during image generation streaming.
    #[serde(rename = "image_generation.partial_image")]
    PartialImage(ImageGenPartialImageEvent),
    /// Emitted when image generation has completed and the final image is available.
    #[serde(rename = "image_generation.completed")]
    Completed(ImageGenCompletedEvent),
}

/// Emitted when a partial image is available during image editing streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditPartialImageEvent {
    /// Base64-encoded partial image data, suitable for rendering as an image.
    pub b64_json: String,
    /// The Unix timestamp when the event was created.
    pub created_at: u64,
    /// The size of the requested edited image.
    pub size: ImageSize,
    /// The quality setting for the requested edited image.
    pub quality: ImageQuality,
    /// The background setting for the requested edited image.
    pub background: ImageBackground,
    /// The output format for the requested edited image.
    pub output_format: ImageOutputFormat,
    /// 0-based index for the partial image (streaming).
    pub partial_image_index: u8,
}

/// Emitted when image editing has completed and the final image is available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEditCompletedEvent {
    /// Base64-encoded final image data, suitable for rendering as an image.
    pub b64_json: String,
    /// The Unix timestamp when the event was created.
    pub created_at: u64,
    /// The size of the edited image.
    pub size: ImageSize,
    /// The quality setting for the edited image.
    pub quality: ImageQuality,
    /// The background setting for the edited image.
    pub background: ImageBackground,
    /// The output format for the edited image.
    pub output_format: ImageOutputFormat,
    /// Token usage information for the image edit.
    pub usage: ImageGenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageEditStreamEvent {
    /// Emitted when a partial image is available during image editing streaming.
    #[serde(rename = "image_edit.partial_image")]
    PartialImage(ImageEditPartialImageEvent),
    /// Emitted when image editing has completed and the final image is available.
    #[serde(rename = "image_edit.completed")]
    Completed(ImageEditCompletedEvent),
}

#[cfg(feature = "_api")]
pub type ImageEditStream = std::pin::Pin<
    Box<dyn futures::Stream<Item = Result<ImageEditStreamEvent, crate::error::OpenAIError>> + Send>,
>;

#[cfg(feature = "_api")]
pub type ImageGenStream = std::pin::Pin<
    Box<dyn futures::Stream<Item = Result<ImageGenStreamEvent, crate::error::OpenAIError>> + Send>,
>;

#[cfg(feature = "_api")]
macro_rules! impl_event_type {
    ($($ty:ty => $event_type:expr),* $(,)?) => {
        $(
            impl crate::traits::EventType for $ty {
                fn event_type(&self) -> &'static str {
                    $event_type
                }
            }
        )*
    };
}

#[cfg(feature = "_api")]
impl_event_type! {
    ImageGenPartialImageEvent => "image_generation.partial_image",
    ImageGenCompletedEvent => "image_generation.completed",
    ImageEditPartialImageEvent => "image_edit.partial_image",
    ImageEditCompletedEvent => "image_edit.completed",
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for ImageGenStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ImageGenStreamEvent::PartialImage(event) => event.event_type(),
            ImageGenStreamEvent::Completed(event) => event.event_type(),
        }
    }
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for ImageEditStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ImageEditStreamEvent::PartialImage(event) => event.event_type(),
            ImageEditStreamEvent::Completed(event) => event.event_type(),
        }
    }
}
