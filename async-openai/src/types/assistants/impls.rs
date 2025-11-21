use crate::types::assistants::CreateMessageRequestContent;

impl From<String> for CreateMessageRequestContent {
    fn from(value: String) -> Self {
        Self::Content(value)
    }
}

impl From<&str> for CreateMessageRequestContent {
    fn from(value: &str) -> Self {
        Self::Content(value.to_string())
    }
}

impl Default for CreateMessageRequestContent {
    fn default() -> Self {
        Self::Content("".into())
    }
}
