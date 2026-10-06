use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

pub use crate::types::shared::ImageDetail;

/// A request to classify or score shared evidence against one or more questions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    name = "DecisionRequestArgs",
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "OpenAIError")
)]
pub struct DecisionRequest {
    pub model: String,
    pub input: DecisionInput,
    pub questions: Vec<QuestionParam>,
    /// Opaque caller-provided end-user identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub safety_identifier: Option<String>,
}

/// Shared evidence supplied as text or user messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInput {
    Text(String),
    Messages(Vec<DecisionInputItem>),
}

impl From<String> for DecisionInput {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for DecisionInput {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// A supported item in decision evidence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputItem {
    Message(DecisionInputMessage),
}

/// A user message containing text or inline images.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputMessage {
    #[serde(default = "user_role")]
    pub role: String,
    pub content: DecisionInputContent,
}

fn user_role() -> String {
    "user".to_owned()
}

/// Text evidence or an ordered list of text and inline image parts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputContent {
    Text(String),
    Parts(Vec<DecisionInputContentPart>),
}

/// A supported decision input content part.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DecisionInputContentPart {
    InputText(DecisionInputText),
    InputImage(DecisionInputImage),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputText {
    pub text: String,
}

/// An inline base64 image. External URLs and file IDs are not supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputImage {
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// A question about the request input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuestionParam {
    Predicate(QuestionPredicate),
    Choice(QuestionChoice),
    Score(QuestionScore),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionPredicate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: String,
    pub choices: Vec<ChoiceOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceOption {
    pub value: ChoiceValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChoiceValue {
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestionScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub instructions: String,
    pub levels: Vec<ScoreLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreLevel {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// The result of a Decisions API request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionResponse {
    pub model: String,
    pub answers: Vec<AnswerResource>,
    pub usage: DecisionUsage,
}

/// An answer to a decision question.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnswerResource {
    Predicate(PredicateAnswer),
    Choice(ChoiceAnswer),
    Score(ScoreAnswer),
    Refusal(RefusalAnswer),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PredicateAnswer {
    pub name: Option<String>,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceAnswer {
    pub name: Option<String>,
    pub choice: ChoiceValue,
    pub probabilities: Vec<ChoiceProbability>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceProbability {
    pub value: ChoiceValue,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreAnswer {
    pub name: Option<String>,
    pub score: f64,
    pub probabilities: Vec<ScoreProbability>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreProbability {
    pub value: i64,
    pub label: String,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefusalAnswer {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionUsage {
    pub input_tokens: i64,
    pub input_tokens_details: DecisionInputTokenDetails,
    pub output_tokens: i64,
    pub output_tokens_details: DecisionOutputTokenDetails,
    pub total_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionInputTokenDetails {
    pub cached_tokens: i64,
    pub cache_write_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionOutputTokenDetails {
    pub reasoning_tokens: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_union_uses_one_discriminator() {
        let question = QuestionParam::Predicate(QuestionPredicate {
            name: Some("damaged".into()),
            instructions: "Was the item damaged?".into(),
        });
        let value = serde_json::to_value(question).unwrap();
        assert_eq!(value["type"], "predicate");
        assert_eq!(
            value
                .as_object()
                .unwrap()
                .keys()
                .filter(|k| *k == "type")
                .count(),
            1
        );
    }
}
