use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModerationInput {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Default, Clone, Copy, PartialEq)]
pub enum TextModerationModel {
    #[default]
    #[serde(rename = "text-moderation-latest")]
    Latest,
    #[serde(rename = "text-moderation-stable")]
    Stable,
}

#[derive(Debug, Default, Clone, Serialize, Builder, PartialEq)]
#[builder(name = "CreateModerationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateModerationRequest {
    /// The input text to classify
    pub input: ModerationInput,

    /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.
    ///
    /// The default is `text-moderation-latest` which will be automatically upgraded over time. This ensures you are always using our most accurate model. If you use `text-moderation-stable`, we will provide advanced notice before updating the model. Accuracy of `text-moderation-stable` may be slightly lower than for `text-moderation-latest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<TextModerationModel>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Category {
    /// Content that expresses, incites, or promotes hate based on race, gender,
    /// ethnicity, religion, nationality, sexual orientation, disability status, or
    /// caste. Hateful content aimed at non-protected groups (e.g., chess players)
    /// is harrassment.
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    /// Hateful content that also includes violence or serious harm towards the
    /// targeted group based on race, gender, ethnicity, religion, nationality,
    /// sexual orientation, disability status, or caste.
    pub hate_threatening: bool,
    /// Content that expresses, incites, or promotes harassing language towards any target.
    pub harassment: bool,
    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    /// Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    /// Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    /// Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
    pub sexual: bool,
    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    /// Content that depicts death, violence, or physical injury.
    pub violence: bool,
    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

/// A list of the categories along with their scores as predicted by model.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CategoryScore {
    /// The score for the category 'hate'.
    pub hate: f32,
    /// The score for the category 'hate/threatening'.
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f32,
    /// The score for the category 'harassment'.
    pub harassment: f32,
    /// The score for the category 'harassment/threatening'.
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f32,
    /// The score for the category 'self-harm'.
    #[serde(rename = "self-harm")]
    pub self_harm: f32,
    /// The score for the category 'self-harm/intent'.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f32,
    /// The score for the category 'self-harm/instructions'.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f32,
    /// The score for the category 'sexual'.
    pub sexual: f32,
    /// The score for the category 'sexual/minors'.
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f32,
    /// The score for the category 'violence'.
    pub violence: f32,
    /// The score for the category 'violence/graphic'.
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ContentModerationResult {
    /// Whether the content violates [OpenAI's usage policies](https://platform.openai.com/policies/usage-policies).
    pub flagged: bool,
    /// A list of the categories, and whether they are flagged or not.
    pub categories: Category,
    /// A list of the categories along with their scores as predicted by model.
    pub category_scores: CategoryScore,
}

/// Represents policy compliance report by OpenAI's content moderation model against a given input.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreateModerationResponse {
    /// The unique identifier for the moderation request.
    pub id: String,
    /// The model used to generate the moderation results.
    pub model: String,
    /// A list of moderation objects.
    pub results: Vec<ContentModerationResult>,
}
