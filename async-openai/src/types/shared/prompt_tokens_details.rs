use serde::{Deserialize, Serialize};

/// Breakdown of tokens used in a completion.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PromptTokensDetails {
    /// Audio input tokens present in the prompt.
    pub audio_tokens: Option<u32>,
    /// Cached tokens present in the prompt.
    pub cached_tokens: Option<u32>,
    /// Text input tokens present in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<i64>,

    /// Image input tokens present in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tokens: Option<i64>,

    /// The unadjusted number of prompt tokens written to cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<i64>,
}
