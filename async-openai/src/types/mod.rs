//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
mod assistant;
mod assistant_file;
mod assistant_impls;
mod audio;
mod common;
mod file;
mod fine_tune;
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
pub use common::*;
pub use file::*;
pub use fine_tune::*;
pub use image::*;
pub use message::*;
pub use message_file::*;
pub use model::*;
pub use moderation::*;
pub use run::*;
pub use step::*;
pub use thread::*;

mod impls;
mod types;
use derive_builder::UninitializedFieldError;
pub use types::*;

use crate::error::OpenAIError;

impl From<UninitializedFieldError> for OpenAIError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenAIError::InvalidArgument(value.to_string())
    }
}
