use serde::{Deserialize, Serialize};

/// Labels an `assistant` message as intermediate commentary or the final answer.
/// For models like `gpt-5.3-codex` and beyond, when sending follow-up requests, preserve and resend
/// phase on all assistant messages — dropping it can degrade performance.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessagePhase {
    Commentary,
    FinalAnswer,
}
