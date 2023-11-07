use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a thread that contains [messages](https://platform.openai.com/docs/api-reference/messages).
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub struct ThreadObject {
    /// The identifier, which can be referenced in API endpoints.
    id: String,
    /// The object type, which is always `thread`.
    object: String,
    /// The Unix timestamp (in seconds) for when the thread was created.
    created_at: i32,

    metadata: Option<HashMap<String, serde_json::Value>>,
}
