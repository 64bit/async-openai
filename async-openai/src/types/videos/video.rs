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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSeconds {
    #[default]
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "8")]
    Eight,
    #[serde(rename = "12")]
    Twelve,
}

// CreateVideoBody in the spec
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(name = "CreateVideoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateVideoRequest {
    /// The video generation model to use (allowed values: sora-2, sora-2-pro). Defaults to `sora-2`.
    pub model: String,

    /// Text prompt that describes the video to generate.
    pub prompt: String,
    /// Output resolution formatted as width x height (allowed values: 720x1280, 1280x720, 1024x1792,
    /// 1792x1024). Defaults to 720x1280.
    pub size: Option<VideoSize>,
    /// Clip duration in seconds (allowed values: 4, 8, 12). Defaults to 4 seconds.
    pub seconds: Option<VideoSeconds>,
    /// Optional image reference that guides generation.
    pub input_reference: Option<ImageInput>,
    /// Character IDs to include in the generation.
    pub character_ids: Option<Vec<String>>,
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
pub struct VideoResourceError {
    pub code: String,
    pub message: String,
}

/// Structured information describing a generated video job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResource {
    /// Unix timestamp (seconds) for when the job completed, if finished.
    pub completed_at: Option<u64>,

    /// Unix timestamp (seconds) for when the job was created.
    pub created_at: u64,

    /// Error payload that explains why generation failed, if applicable.
    pub error: Option<VideoResourceError>,

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
    pub seconds: VideoSeconds,

    /// The resolution of the generated video.
    pub size: VideoSize,

    /// Current lifecycle status of the video job.
    pub status: VideoStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedVideoResource {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoListResource {
    pub data: Vec<VideoResource>,
    pub object: String,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VideoVariant {
    #[default]
    Video,
    Thumbnail,
    Spritesheet,
}
