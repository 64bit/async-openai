use std::fmt::Display;

use crate::types::chat::{
    ChatCompletionFunctionCall, ChatCompletionMessageCustomToolCall, ChatCompletionMessageToolCall,
    ChatCompletionMessageToolCalls, ChatCompletionNamedToolChoice,
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent,
    ChatCompletionRequestDeveloperMessage, ChatCompletionRequestDeveloperMessageContent,
    ChatCompletionRequestFunctionMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestMessageContentPartAudio, ChatCompletionRequestMessageContentPartImage,
    ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestToolMessage,
    ChatCompletionRequestToolMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
    ChatCompletionTool, ChatCompletionTools, CustomToolChatCompletions, FunctionName, ImageUrl,
    Role,
};

impl From<ChatCompletionRequestUserMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestUserMessage) -> Self {
        Self::User(value)
    }
}

impl From<ChatCompletionRequestSystemMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestSystemMessage) -> Self {
        Self::System(value)
    }
}

impl From<ChatCompletionRequestDeveloperMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestDeveloperMessage) -> Self {
        Self::Developer(value)
    }
}

impl From<ChatCompletionRequestAssistantMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestAssistantMessage) -> Self {
        Self::Assistant(value)
    }
}

impl From<ChatCompletionRequestFunctionMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestFunctionMessage) -> Self {
        Self::Function(value)
    }
}

impl From<ChatCompletionRequestToolMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestToolMessage) -> Self {
        Self::Tool(value)
    }
}

impl From<ChatCompletionRequestUserMessageContent> for ChatCompletionRequestUserMessage {
    fn from(value: ChatCompletionRequestUserMessageContent) -> Self {
        Self {
            content: value,
            name: None,
        }
    }
}

impl From<ChatCompletionRequestSystemMessageContent> for ChatCompletionRequestSystemMessage {
    fn from(value: ChatCompletionRequestSystemMessageContent) -> Self {
        Self {
            content: value,
            name: None,
        }
    }
}

impl From<ChatCompletionRequestDeveloperMessageContent> for ChatCompletionRequestDeveloperMessage {
    fn from(value: ChatCompletionRequestDeveloperMessageContent) -> Self {
        Self {
            content: value,
            name: None,
        }
    }
}

impl From<ChatCompletionRequestAssistantMessageContent> for ChatCompletionRequestAssistantMessage {
    fn from(value: ChatCompletionRequestAssistantMessageContent) -> Self {
        Self {
            content: Some(value),
            ..Default::default()
        }
    }
}

impl From<&str> for ChatCompletionRequestUserMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestUserMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestSystemMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestSystemMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestDeveloperMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestDeveloperMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestDeveloperMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestDeveloperMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestAssistantMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestToolMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestToolMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestToolMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestToolMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestUserMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestUserMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for ChatCompletionRequestSystemMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value.into()).into()
    }
}

impl From<&str> for ChatCompletionRequestDeveloperMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestDeveloperMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestSystemMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<String> for ChatCompletionRequestDeveloperMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestAssistantMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<Vec<ChatCompletionRequestUserMessageContentPart>>
    for ChatCompletionRequestUserMessageContent
{
    fn from(value: Vec<ChatCompletionRequestUserMessageContentPart>) -> Self {
        ChatCompletionRequestUserMessageContent::Array(value)
    }
}

impl From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        ChatCompletionRequestUserMessageContentPart::Text(value)
    }
}

impl From<ChatCompletionRequestMessageContentPartImage>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartImage) -> Self {
        ChatCompletionRequestUserMessageContentPart::ImageUrl(value)
    }
}

impl From<ChatCompletionRequestMessageContentPartAudio>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartAudio) -> Self {
        ChatCompletionRequestUserMessageContentPart::InputAudio(value)
    }
}

impl From<&str> for ChatCompletionRequestMessageContentPartText {
    fn from(value: &str) -> Self {
        ChatCompletionRequestMessageContentPartText { text: value.into() }
    }
}

impl From<String> for ChatCompletionRequestMessageContentPartText {
    fn from(value: String) -> Self {
        ChatCompletionRequestMessageContentPartText { text: value }
    }
}

impl From<&str> for ChatCompletionFunctionCall {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "none" => Self::None,
            _ => Self::Function { name: value.into() },
        }
    }
}

impl From<&str> for FunctionName {
    fn from(value: &str) -> Self {
        Self { name: value.into() }
    }
}

impl From<String> for FunctionName {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl From<&str> for ChatCompletionNamedToolChoice {
    fn from(value: &str) -> Self {
        Self {
            function: value.into(),
        }
    }
}

impl From<String> for ChatCompletionNamedToolChoice {
    fn from(value: String) -> Self {
        Self {
            function: value.into(),
        }
    }
}

impl Default for ChatCompletionRequestDeveloperMessageContent {
    fn default() -> Self {
        ChatCompletionRequestDeveloperMessageContent::Text("".into())
    }
}

impl Default for ChatCompletionRequestSystemMessageContent {
    fn default() -> Self {
        ChatCompletionRequestSystemMessageContent::Text("".into())
    }
}

impl Default for ChatCompletionRequestToolMessageContent {
    fn default() -> Self {
        ChatCompletionRequestToolMessageContent::Text("".into())
    }
}

impl Default for ChatCompletionRequestUserMessageContent {
    fn default() -> Self {
        ChatCompletionRequestUserMessageContent::Text("".into())
    }
}

impl From<&str> for ImageUrl {
    fn from(value: &str) -> Self {
        Self {
            url: value.into(),
            detail: Default::default(),
        }
    }
}

impl From<String> for ImageUrl {
    fn from(value: String) -> Self {
        Self {
            url: value,
            detail: Default::default(),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::System => "system",
                Role::Assistant => "assistant",
                Role::Function => "function",
                Role::Tool => "tool",
            }
        )
    }
}

impl From<ChatCompletionTool> for Vec<ChatCompletionTools> {
    fn from(value: ChatCompletionTool) -> Self {
        vec![ChatCompletionTools::Function(value)]
    }
}

impl From<CustomToolChatCompletions> for Vec<ChatCompletionTools> {
    fn from(value: CustomToolChatCompletions) -> Self {
        vec![ChatCompletionTools::Custom(value)]
    }
}

impl From<ChatCompletionRequestUserMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestUserMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionRequestSystemMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestSystemMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionRequestDeveloperMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestDeveloperMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionRequestAssistantMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestAssistantMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionRequestFunctionMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestFunctionMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionRequestToolMessage> for Vec<ChatCompletionRequestMessage> {
    fn from(value: ChatCompletionRequestToolMessage) -> Self {
        vec![value.into()]
    }
}

impl From<ChatCompletionMessageToolCall> for ChatCompletionMessageToolCalls {
    fn from(value: ChatCompletionMessageToolCall) -> Self {
        ChatCompletionMessageToolCalls::Function(value)
    }
}

impl From<ChatCompletionMessageCustomToolCall> for ChatCompletionMessageToolCalls {
    fn from(value: ChatCompletionMessageCustomToolCall) -> Self {
        ChatCompletionMessageToolCalls::Custom(value)
    }
}

impl From<ImageUrl> for ChatCompletionRequestMessageContentPartImage {
    fn from(value: ImageUrl) -> Self {
        ChatCompletionRequestMessageContentPartImage { image_url: value }
    }
}
