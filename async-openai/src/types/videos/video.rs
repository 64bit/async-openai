use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;
use crate::types::videos::ImageInput;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSize {
    #[default]
    #[serde(rename = "720x1280")]
    S720x1280,
    #[serde(rename = "1280x720")]
    S1280x720,
    #[serde(rename = "1024x1792")]
    S1024x1792,
    #[serde(rename = "1792x1024")]
    S1792x1024,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateVideoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateVideoRequest {
    /// ID of the model to use.
    pub model: String,

    /// The prompt to generate video from.
    pub prompt: String,

    pub size: Option<VideoSize>,

    pub seconds: Option<String>,

    pub input_reference: Option<ImageInput>,
}

#[derive(Clone, Default, Debug, Builder, PartialEq, Serialize)]
#[builder(name = "RemixVideoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct RemixVideoRequest {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJobError {
    pub code: String,
    pub message: String,
}

/// Structured information describing a generated video job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJob {
    /// Unix timestamp (seconds) for when the job completed, if finished.
    pub completed_at: Option<u64>,

    /// Unix timestamp (seconds) for when the job was created.
    pub created_at: u64,

    /// Error payload that explains why generation failed, if applicable.
    pub error: Option<VideoJobError>,

    /// Unix timestamp (seconds) for when the downloadable assets expire, if set.
    pub expires_at: Option<u64>,

    /// Unique identifier for the video job.
    pub id: String,

    /// The video generation model that produced the job.
    pub model: String,

    /// The object type, which is always video.
    pub object: String,

    /// Approximate completion percentage for the generation task.
    pub progress: u8,

    /// Identifier of the source video if this video is a remix.
    pub remixed_from_video_id: Option<String>,

    /// Duration of the generated clip in seconds.
    pub seconds: String,

    /// The resolution of the generated video.
    pub size: String,

    /// Current lifecycle status of the video job.
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJobMetadata {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVideosResponse {
    pub data: Vec<VideoJob>,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VideoVariant {
    #[default]
    Video,
    Thumbnail,
    Spritesheet,
}
