use async_openai::types::{chat::*, completions::*};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(components(schemas(
    CreateChatCompletionRequest,
    CreateChatCompletionResponse,
    CreateChatCompletionStreamResponse,
    CreateCompletionRequest,
    CreateCompletionResponse
)))]
struct Contract;

fn document() -> Value {
    serde_json::to_value(Contract::openapi()).unwrap()
}

fn validator<T: ToSchema>() -> jsonschema::Validator {
    let mut doc = document();
    doc["$ref"] = json!(format!("#/components/schemas/{}", T::name()));
    jsonschema::validator_for(&doc).unwrap()
}

fn accepts<T: ToSchema + DeserializeOwned + Serialize>(values: &[Value]) {
    let schema = validator::<T>();
    for value in values {
        let parsed: T = serde_json::from_value(value.clone()).unwrap();
        assert!(schema.is_valid(value), "input rejected by schema: {value}");
        let encoded = serde_json::to_value(parsed).unwrap();
        assert!(
            schema.is_valid(&encoded),
            "output rejected by schema: {encoded}"
        );
    }
}

#[test]
fn complete_component_graph_has_no_dangling_references() {
    fn visit(node: &Value, root: &Value) {
        match node {
            Value::Object(fields) => {
                if let Some(reference) = fields.get("$ref").and_then(Value::as_str) {
                    assert!(reference.starts_with("#/components/schemas/"));
                    assert!(root.pointer(&reference[1..]).is_some(), "{reference}");
                }
                for value in fields.values() {
                    visit(value, root);
                }
            }
            Value::Array(values) => {
                for value in values {
                    visit(value, root);
                }
            }
            _ => {}
        }
    }
    let doc = document();
    visit(&doc, &doc);
    for name in doc["components"]["schemas"].as_object().unwrap().keys() {
        assert!(name.starts_with("async_openai."), "{name}");
    }
}

#[test]
fn request_schema_preserves_required_and_nullable_fields() {
    accepts::<CreateChatCompletionRequest>(&[
        json!({"model":"test","messages":[{"role":"user","content":"hello"}]}),
        json!({"model":"test","messages":[],"temperature":null,"stream":false,
            "response_format":{"type":"json_schema","json_schema":{
                "name":"answer","schema":{"type":"object"}}}}),
    ]);
    let schema = validator::<CreateChatCompletionRequest>();
    for value in [
        json!({"model":"test"}),
        json!({"messages":[]}),
        json!({"model":"test","messages":[],"stream":"yes"}),
        json!({"model":"test","messages":[{"role":"invalid","content":"hello"}]}),
    ] {
        assert!(!schema.is_valid(&value), "{value}");
        assert!(serde_json::from_value::<CreateChatCompletionRequest>(value).is_err());
    }
}

#[test]
fn prompt_untagged_arrays_include_the_overlapping_empty_case() {
    accepts::<Prompt>(&[
        json!("hi"),
        json!(["hi"]),
        json!([1, 2]),
        json!([[1], [2]]),
        json!([]),
    ]);
    let schema = validator::<Prompt>();
    assert!(!schema.is_valid(&json!(true)));
    assert!(!schema.is_valid(&json!(["hi", 1])));
}

#[test]
fn mixed_tool_choices_are_strings_or_direct_objects() {
    accepts::<ChatCompletionToolChoiceOption>(&[
        json!("none"),
        json!("auto"),
        json!("required"),
        json!({"type":"function","function":{"name":"lookup"}}),
        json!({"type":"custom","custom":{"name":"lookup"}}),
        json!({"type":"allowed_tools","allowed_tools":[]}),
    ]);
    let schema = validator::<ChatCompletionToolChoiceOption>();
    for value in [
        json!({"Mode":"auto"}),
        json!("invalid"),
        json!({"type":"function"}),
    ] {
        assert!(!schema.is_valid(&value), "{value}");
    }
}

#[test]
fn legacy_function_choice_and_extensible_voice_match_serde() {
    accepts::<ChatCompletionFunctionCall>(&[
        json!("none"),
        json!("auto"),
        json!({"name":"lookup"}),
    ]);
    assert!(
        !validator::<ChatCompletionFunctionCall>().is_valid(&json!({"Function":{"name":"lookup"}}))
    );
    accepts::<ChatCompletionAudioVoice>(&[json!("alloy"), json!("new-voice")]);
    assert!(!validator::<ChatCompletionAudioVoice>().is_valid(&json!({"Other":"new-voice"})));
}

#[test]
fn response_and_stream_schemas_accept_serialized_payloads() {
    accepts::<CreateChatCompletionResponse>(&[json!({
        "id":"chat-1","object":"chat.completion","created":1,"model":"test",
        "choices":[{"index":0,"message":{"role":"assistant","content":"hi"},
                    "finish_reason":"stop","logprobs":null}]
    })]);
    accepts::<CreateChatCompletionStreamResponse>(&[json!({
        "id":"chat-1","object":"chat.completion.chunk","created":1,"model":"test",
        "choices":[{"index":0,"delta":{"content":"hi"},"finish_reason":null,"logprobs":null}]
    })]);
    accepts::<CreateCompletionRequest>(&[json!({"model":"test","prompt":[]})]);
    accepts::<CreateCompletionResponse>(&[json!({
        "id":"cmpl-1","object":"text_completion","created":1,"model":"test",
        "choices":[{"index":0,"text":"hi","finish_reason":"stop","logprobs":null}]
    })]);
}
