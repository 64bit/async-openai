use serde::{Deserialize, Serialize};

use crate::types::chat::CreateChatCompletionRequest;
use crate::types::completion::CreateCompletionRequest;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CreateTokenizeRequest {
    Chat(CreateChatCompletionRequest),
    Completion(CreateCompletionRequest),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateTokenizeResponse {
    count: u32,
    max_model_len: u32,
    tokens: Vec<u32>,
}
