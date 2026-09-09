/// The moderation policy for the response input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ModerationConfigParam {
    pub mode: ModerationMode,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ModerationInputType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ModerationMode {
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "block")]
    Block,
}

/// Configuration for running moderation on the input and output of this response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ModerationParam {
    /// The moderation model to use for moderated completions, e.g. 'omni-moderation-latest'.
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ModerationPolicyParam>,
}

/// The policy to apply to moderated response input and output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ModerationPolicyParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ModerationConfigParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ModerationConfigParam>,
}

/// A moderation result produced for the response input or output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ModerationResultBody {
    /// The moderation model that produced this result.
    pub model: String,
    /// A boolean indicating whether the content was flagged by any category.
    pub flagged: bool,
    /// A dictionary of moderation categories to booleans, True if the input is flagged under this category.
    pub categories: std::collections::HashMap<String, bool>,
    /// A dictionary of moderation categories to scores.
    pub category_scores: std::collections::HashMap<String, f64>,
    /// Which modalities of input are reflected by the score for each category.
    pub category_applied_input_types: std::collections::HashMap<String, Vec<ModerationInputType>>,
}
