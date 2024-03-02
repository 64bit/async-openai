//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
mod assistant;
mod assistant_file;
mod assistant_impls;
mod audio;
mod chat;
mod common;
mod completion;
mod embedding;
mod file;
mod fine_tuning;
mod image;
mod message;
mod message_file;
mod model;
mod moderation;
mod run;
mod step;
mod thread;

pub use assistant::*;
pub use assistant_file::*;
pub use audio::*;
pub use chat::*;
pub use common::*;
pub use completion::*;
pub use embedding::*;
pub use file::*;
pub use fine_tuning::*;
pub use image::*;
pub use message::*;
pub use message_file::*;
pub use model::*;
pub use moderation::*;
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
