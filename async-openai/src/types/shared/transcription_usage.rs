use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUsageInputTokenDetails {
    /// Number of audio tokens billed for this request.
    pub audio_tokens: u32,
    /// Number of text tokens billed for this request.
    pub text_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptTextUsageTokens {
    /// Number of input tokens billed for this request.
    pub input_tokens: u32,
    /// Number of output tokens generated.
    pub output_tokens: u32,
    /// Total number of tokens used (input + output).
    pub total_tokens: u32,
    /// Details about the input tokens billed for this request.
    pub input_token_details: Option<TokenUsageInputTokenDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptTextUsageDuration {
    ///Duration of the input audio in seconds.
    pub seconds: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TranscriptionUsage {
    #[serde(rename = "tokens")]
    Tokens(TranscriptTextUsageTokens),
    #[serde(rename = "duration")]
    Duration(TranscriptTextUsageDuration),
}
