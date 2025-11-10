//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
mod assistant;
mod assistant_impls;
mod assistant_stream;
pub mod audio;
mod audit_log;
pub mod batches;
mod chat;
mod common;
mod completion;
mod containers;
pub mod embeddings;
pub mod evals;
pub mod files;
pub mod finetuning;
pub mod graders;
pub mod images;
mod invites;
mod logprob;
mod mcp;
mod message;
pub mod models;
pub mod moderations;
mod project_api_key;
mod project_service_account;
mod project_users;
mod projects;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime")))]
#[cfg(feature = "realtime")]
pub mod realtime;
pub mod responses;
mod run;
mod step;
mod thread;
pub mod uploads;
mod users;
mod vector_store;
mod video;
#[cfg_attr(docsrs, doc(cfg(feature = "webhook")))]
#[cfg(feature = "webhook")]
pub mod webhooks;

pub use assistant::*;
pub use assistant_stream::*;
pub use audit_log::*;
pub use chat::*;
pub use common::*;
pub use completion::*;
pub use containers::*;
pub use invites::*;
pub use logprob::*;
pub use mcp::*;
pub use message::*;
pub use project_api_key::*;
pub use project_service_account::*;
pub use project_users::*;
pub use projects::*;
pub use run::*;
pub use step::*;
pub use thread::*;
pub use users::*;
pub use vector_store::*;
pub use video::*;

mod impls;
use derive_builder::UninitializedFieldError;

use crate::error::OpenAIError;

impl From<UninitializedFieldError> for OpenAIError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenAIError::InvalidArgument(value.to_string())
    }
}
