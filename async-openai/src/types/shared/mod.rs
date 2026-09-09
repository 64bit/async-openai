//! Shared types - these types are use by multiple type modules
//! and not exported directly, instead they are re-exported
//! by the modules that use them.

#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod completion_tokens_details;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod custom_grammar_format_param;
#[cfg(any(feature = "response-types", feature = "vectorstore-types"))]
mod filter;
#[cfg(feature = "chat-completion-types")]
mod function_call;
#[cfg(feature = "chat-completion-types")]
mod function_name;
#[cfg(feature = "chat-completion-types")]
mod function_object;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod image_detail;
#[cfg(any(feature = "image-types", feature = "video-types"))]
mod image_input;
#[cfg(feature = "chat-completion-types")]
mod image_url;
#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
mod log_prob_properties;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod prompt_tokens_details;
#[cfg(any(
    feature = "chat-completion-types",
    feature = "response-types",
    feature = "grader-types"
))]
mod reasoning_effort;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod response_format;
#[cfg(any(feature = "response-types", feature = "batch-types"))]
mod response_usage;
#[cfg(feature = "vectorstore-types")]
mod static_chunking_strategy;
#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
mod transcription_usage;

#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use completion_tokens_details::*;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use custom_grammar_format_param::*;
#[cfg(any(feature = "response-types", feature = "vectorstore-types"))]
pub use filter::*;
#[cfg(feature = "chat-completion-types")]
pub use function_call::*;
#[cfg(feature = "chat-completion-types")]
pub use function_name::*;
#[cfg(feature = "chat-completion-types")]
pub use function_object::*;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use image_detail::*;
#[cfg(any(feature = "image-types", feature = "video-types"))]
pub use image_input::*;
#[cfg(feature = "chat-completion-types")]
pub use image_url::*;
#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
pub use log_prob_properties::*;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use prompt_tokens_details::*;
#[cfg(any(
    feature = "chat-completion-types",
    feature = "response-types",
    feature = "grader-types"
))]
pub use reasoning_effort::*;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use response_format::*;
#[cfg(any(feature = "response-types", feature = "batch-types"))]
pub use response_usage::*;
#[cfg(feature = "vectorstore-types")]
pub use static_chunking_strategy::*;
#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
pub use transcription_usage::*;

#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod prompt_cache;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use prompt_cache::*;

#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
mod moderation;
#[cfg(any(feature = "chat-completion-types", feature = "response-types"))]
pub use moderation::*;

mod misalignment;
pub use misalignment::*;

#[cfg(feature = "administration-types")]
mod spend_alerts;
#[cfg(feature = "administration-types")]
pub use spend_alerts::*;

#[cfg(feature = "administration-types")]
mod spend_limit;
#[cfg(feature = "administration-types")]
pub use spend_limit::*;

#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
mod transcription_language;
#[cfg(any(feature = "audio-types", feature = "realtime-types"))]
pub use transcription_language::*;
