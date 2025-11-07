use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Log probability information for a transcribed token.
pub struct LogProbProperties {
    /// The bytes that were used to generate the log probability.
    pub bytes: Vec<u8>,
    /// The log probability of the token.
    pub logprob: f64,
    /// The token that was used to generate the log probability.
    pub token: String,
}
