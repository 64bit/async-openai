use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::{Choice, CompletionUsage};

#[derive(Debug, Clone, Serialize, Default, Builder, PartialEq)]
#[builder(name = "CreateEditRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateEditRequest {
    /// ID of the model to use. You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: String,

    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>, // default ''

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,

    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1 max: 20 default:1

    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min:0 ,max: 2, default: 1,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///  We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateEditResponse {
    pub object: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: CompletionUsage,
}
