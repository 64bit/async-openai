use serde::{Deserialize, Serialize};

use crate::types::evals::EvalItem;
use crate::types::graders::ReasoningEffort;

/// String check operation.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GraderStringCheckOperation {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Like.
    Like,
    /// Case-insensitive like.
    Ilike,
}

/// String check grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderStringCheck {
    /// The name of the grader.
    pub name: String,
    /// The input text. This may include template strings.
    pub input: String,
    /// The reference text. This may include template strings.
    pub reference: String,
    /// The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`.
    pub operation: GraderStringCheckOperation,
}

/// Text similarity grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderTextSimilarity {
    /// The name of the grader.
    pub name: String,
    /// The text being graded.
    pub input: String,
    /// The text being graded against.
    pub reference: String,
    /// The evaluation metric to use.
    pub evaluation_metric: GraderTextSimilarityEvaluationMetric,
}

/// Text similarity metric.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GraderTextSimilarityEvaluationMetric {
    Cosine,
    FuzzyMatch,
    Bleu,
    Gleu,
    Meteor,
    Rouge1,
    Rouge2,
    Rouge3,
    Rouge4,
    Rouge5,
    RougeL,
}

/// Python grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderPython {
    /// The name of the grader.
    pub name: String,
    /// The source code of the python script.
    pub source: String,
    /// The image tag to use for the python script.
    pub image_tag: Option<String>,
}

/// Score model grader.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderScoreModel {
    /// The name of the grader.
    pub name: String,
    /// The model to use for the evaluation.
    pub model: String,
    /// A list of chat messages forming the prompt or context.
    pub input: Vec<EvalItem>,

    /// Optional sampling parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_params: Option<GraderScoreModelSamplingParams>,
    /// The range of the score. Defaults to [0, 1].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderScoreModelSamplingParams {
    /// A seed value to initialize the randomness, during sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// An alternative to temperature for nucleus sampling; 1.0 includes all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// A higher temperature increases randomness in the outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// The maximum number of tokens the grader model may generate in its response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    /// Optional reasoning effort parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderLabelModel {
    /// The name of the grader.
    pub name: String,
    /// The model to use for the evaluation. Must support structured outputs.
    pub model: String,
    /// A list of chat messages forming the prompt or context.
    pub input: Vec<EvalItem>,
    /// The labels to classify to each item in the evaluation.
    pub labels: Vec<String>,
    /// The labels that indicate a passing result. Must be a subset of labels.
    pub passing_labels: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Graders {
    StringCheck(GraderStringCheck),
    TextSimilarity(GraderTextSimilarity),
    Python(GraderPython),
    ScoreModel(GraderScoreModel),
    LabelModel(GraderLabelModel),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GraderMulti {
    /// The name of the grader.
    pub name: String,
    pub graders: Graders,
    /// A formula to calculate the output based on grader results.
    pub calculate_output: String,
}
