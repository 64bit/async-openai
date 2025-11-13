use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::{
    error::OpenAIError,
    traits::EventType,
    types::images::{ImageBackground, ImageGenUsage, ImageOutputFormat, ImageQuality, ImageSize},
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

pub type ImageGenStream =
    Pin<Box<dyn Stream<Item = Result<ImageGenStreamEvent, OpenAIError>> + Send>>;

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

pub type ImageEditStream =
    Pin<Box<dyn Stream<Item = Result<ImageEditStreamEvent, OpenAIError>> + Send>>;

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

impl EventType for ImageGenPartialImageEvent {
    fn event_type(&self) -> &'static str {
        "image_generation.partial_image"
    }
}

impl EventType for ImageGenCompletedEvent {
    fn event_type(&self) -> &'static str {
        "image_generation.completed"
    }
}

impl EventType for ImageGenStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ImageGenStreamEvent::PartialImage(event) => event.event_type(),
            ImageGenStreamEvent::Completed(event) => event.event_type(),
        }
    }
}

impl EventType for ImageEditPartialImageEvent {
    fn event_type(&self) -> &'static str {
        "image_edit.partial_image"
    }
}

impl EventType for ImageEditCompletedEvent {
    fn event_type(&self) -> &'static str {
        "image_edit.completed"
    }
}

impl EventType for ImageEditStreamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ImageEditStreamEvent::PartialImage(event) => event.event_type(),
            ImageEditStreamEvent::Completed(event) => event.event_type(),
        }
    }
}
