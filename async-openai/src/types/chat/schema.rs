//! Schema-only implementations for unions not represented faithfully by utoipa's derive.
//!
//! Keep Serde on the public types unchanged. In particular, per-variant
//! `serde(untagged)` must not introduce a wrapper named after the Rust variant.

use super::*;
use crate::types::shared::schema::schema;
use utoipa::openapi::{schema::AnyOfBuilder, schema::ObjectBuilder, Type};

// Empty arrays match several branches, so Serde's untagged union needs anyOf,
// not oneOf. Do not add constraints from API prose that Serde does not enforce.
schema!(
    Prompt,
    "async_openai.Prompt",
    AnyOfBuilder::new()
        .item(String::schema())
        .item(Vec::<String>::schema())
        .item(Vec::<u32>::schema())
        .item(Vec::<Vec<u32>>::schema()),
    []
);

schema!(
    ChatCompletionFunctionCall,
    "async_openai.ChatCompletionFunctionCall",
    AnyOfBuilder::new()
        .item(
            ObjectBuilder::new()
                .schema_type(Type::String)
                .enum_values(Some(["none", "auto"]))
        )
        .item(FunctionName::schema()),
    [FunctionName]
);

// The fallback accepts any string, including the named voices. Enumerating the
// names alongside a string branch in oneOf would incorrectly reject them.
schema!(
    ChatCompletionAudioVoice,
    "async_openai.ChatCompletionAudioVoice",
    String::schema(),
    []
);

// Only the object alternatives carry the "type" discriminator.
#[derive(utoipa::ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
enum TaggedToolChoice {
    AllowedTools(ChatCompletionAllowedToolsChoice),
    Function(ChatCompletionNamedToolChoice),
    Custom(ChatCompletionNamedToolChoiceCustom),
}

schema!(
    ChatCompletionToolChoiceOption,
    "async_openai.ChatCompletionToolChoiceOption",
    AnyOfBuilder::new()
        .item(TaggedToolChoice::schema())
        .item(ToolChoiceOptions::schema()),
    [TaggedToolChoice, ToolChoiceOptions]
);
