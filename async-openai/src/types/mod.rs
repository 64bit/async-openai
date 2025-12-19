//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
#[cfg(feature = "administration-types")]
pub mod admin;
#[cfg(feature = "assistant-types")]
pub mod assistants;
#[cfg(feature = "audio-types")]
pub mod audio;
#[cfg(feature = "batch-types")]
pub mod batches;
#[cfg(feature = "chat-completion-types")]
pub mod chat;
#[cfg(feature = "chatkit-types")]
pub mod chatkit;
#[cfg(feature = "completion-types")]
pub mod completions;
#[cfg(feature = "container-types")]
pub mod containers;
#[cfg(feature = "embedding-types")]
pub mod embeddings;
#[cfg(feature = "eval-types")]
pub mod evals;
#[cfg(feature = "file-types")]
pub mod files;
#[cfg(feature = "finetuning-types")]
pub mod finetuning;
#[cfg(feature = "grader-types")]
pub mod graders;
#[cfg(feature = "image-types")]
pub mod images;
#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
mod input_source;
#[cfg(any(feature = "response-types", feature = "realtime-types"))]
pub mod mcp;
#[cfg(any(
    feature = "response-types",
    feature = "audio-types",
    feature = "video-types",
    feature = "image-types",
    feature = "batch-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "vectorstore-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
mod metadata;
#[cfg(feature = "model-types")]
pub mod models;
#[cfg(feature = "moderation-types")]
pub mod moderations;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime-types")))]
#[cfg(feature = "realtime-types")]
pub mod realtime;
#[cfg(feature = "response-types")]
pub mod responses;
#[cfg(any(
    feature = "response-types",
    feature = "video-types",
    feature = "vectorstore-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "batch-types",
    feature = "audio-types",
    feature = "realtime-types",
    feature = "image-types"
))]
mod shared;
#[cfg(feature = "upload-types")]
pub mod uploads;
#[cfg(feature = "vectorstore-types")]
pub mod vectorstores;
#[cfg(feature = "video-types")]
pub mod videos;
#[cfg_attr(docsrs, doc(cfg(feature = "webhook-types")))]
#[cfg(feature = "webhook-types")]
pub mod webhooks;

#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "container-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
pub use input_source::*;

#[cfg(any(
    feature = "audio-types",
    feature = "batch-types",
    feature = "file-types",
    feature = "upload-types",
    feature = "image-types",
    feature = "video-types",
    feature = "vectorstore-types",
    feature = "container-types",
    feature = "response-types",
    feature = "chat-completion-types",
    feature = "realtime-types"
))]
pub use metadata::*;

#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types",
    feature = "video-types"
))]
mod impls;

#[cfg(any(
    feature = "response-types",
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types",
    feature = "administration-types",
))]
impl From<derive_builder::UninitializedFieldError> for crate::error::OpenAIError {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        crate::error::OpenAIError::InvalidArgument(value.to_string())
    }
}
