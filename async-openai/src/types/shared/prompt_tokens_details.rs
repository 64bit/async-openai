use serde::{Deserialize, Serialize};

/// Breakdown of tokens used in a completion.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct PromptTokensDetails {
    /// Audio input tokens present in the prompt.
    pub audio_tokens: Option<u32>,
    /// Cached tokens present in the prompt.
    pub cached_tokens: Option<u32>,
}
