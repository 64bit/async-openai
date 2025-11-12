//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
pub mod admin;
mod assistant;
mod assistant_impls;
mod assistant_stream;
pub mod audio;
pub mod batches;
pub mod chat;
pub mod chatkit;
mod common;
mod completion;
pub mod containers;
pub mod embeddings;
pub mod evals;
pub mod files;
pub mod finetuning;
pub mod graders;
pub mod images;
mod logprob;
mod mcp;
mod message;
pub mod models;
pub mod moderations;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime")))]
#[cfg(feature = "realtime")]
pub mod realtime;
pub mod responses;
mod run;
mod step;
mod thread;
pub mod uploads;
pub mod vectorstores;
pub mod videos;
#[cfg_attr(docsrs, doc(cfg(feature = "webhook")))]
#[cfg(feature = "webhook")]
pub mod webhooks;

pub use assistant::*;
pub use assistant_stream::*;
pub use common::*;
pub use completion::*;
pub use logprob::*;
pub use mcp::*;
pub use message::*;
pub use run::*;
pub use step::*;
pub use thread::*;

mod impls;
use derive_builder::UninitializedFieldError;

use crate::error::OpenAIError;

impl From<UninitializedFieldError> for OpenAIError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenAIError::InvalidArgument(value.to_string())
    }
}
