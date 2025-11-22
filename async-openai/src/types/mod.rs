//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)

mod common;
mod logprob;
mod mcp;

pub use common::*;
pub use logprob::*;
pub use mcp::*;

#[cfg(any(feature = "administration-types", feature = "administration"))]
pub mod admin;
#[cfg(any(feature = "assistant-types", feature = "assistants"))]
pub mod assistants;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod audio;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod batches;
#[cfg(any(feature = "chat-completion-types", feature = "chat-completion"))]
pub mod chat;
#[cfg(any(feature = "chatkit-types", feature = "chatkit"))]
pub mod chatkit;
#[cfg(any(feature = "completions-types", feature = "completions"))]
mod completion;
#[cfg(any(feature = "container-types", feature = "container"))]
pub mod containers;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod embeddings;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod evals;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod files;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod finetuning;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod graders;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod images;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod models;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod moderations;
#[cfg(any(feature = "realtime-types", feature = "realtime"))]
pub mod realtime;
#[cfg(any(feature = "response-types", feature = "responses"))]
pub mod responses;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod uploads;
#[cfg(any(feature = "vector-store-types", feature = "vector-stores"))]
pub mod vectorstores;
#[cfg(any(feature = "platform-types", feature = "platform"))]
pub mod videos;
#[cfg(any(feature = "webhook-types", feature = "webhook"))]
pub mod webhooks;

#[cfg(any(feature = "completions-types", feature = "completions"))]
pub use completion::*;

// Check if any type feature is enabled (which means derive_builder is available)
#[cfg(any(
    feature = "response-types",
    feature = "webhook-types",
    feature = "platform-types",
    feature = "vector-store-types",
    feature = "chatkit-types",
    feature = "container-types",
    feature = "realtime-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "administration-types",
    feature = "completions-types",
))]
mod impls;

#[cfg(any(
    feature = "response-types",
    feature = "webhook-types",
    feature = "platform-types",
    feature = "vector-store-types",
    feature = "chatkit-types",
    feature = "container-types",
    feature = "realtime-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "administration-types",
    feature = "completions-types",
))]
use derive_builder::UninitializedFieldError;

#[cfg(any(
    feature = "response-types",
    feature = "webhook-types",
    feature = "platform-types",
    feature = "vector-store-types",
    feature = "chatkit-types",
    feature = "container-types",
    feature = "realtime-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "administration-types",
    feature = "completions-types",
))]
use crate::error::OpenAIError;

#[cfg(any(
    feature = "response-types",
    feature = "webhook-types",
    feature = "platform-types",
    feature = "vector-store-types",
    feature = "chatkit-types",
    feature = "container-types",
    feature = "realtime-types",
    feature = "chat-completion-types",
    feature = "assistant-types",
    feature = "administration-types",
    feature = "completions-types",
))]
impl From<UninitializedFieldError> for OpenAIError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenAIError::InvalidArgument(value.to_string())
    }
}
