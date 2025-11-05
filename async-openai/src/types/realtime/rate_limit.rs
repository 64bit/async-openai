use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeRateLimitName {
    Requests,
    Tokens,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeRateLimit {
    /// The name of the rate limit (requests, tokens).
    pub name: RealtimeRateLimitName,
    /// The maximum allowed value for the rate limit.
    pub limit: u32,
    /// The remaining value before the limit is reached.
    pub remaining: u32,
    /// Seconds until the rate limit resets.
    pub reset_seconds: f32,
}
