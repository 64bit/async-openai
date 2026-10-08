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
        let type_name = name.strip_prefix("async_openai.").expect(name);
        assert!(!type_name.contains('.'), "internal module in {name}");
    }
}

#[test]
fn component_names_are_flat_for_derived_and_manual_schemas() {
    assert_eq!(
        CreateChatCompletionRequest::name(),
        "async_openai.CreateChatCompletionRequest"
    );
    assert_eq!(
        CreateCompletionRequest::name(),
        "async_openai.CreateCompletionRequest"
    );
    assert_eq!(ImageDetail::name(), "async_openai.ImageDetail");
    assert_eq!(Prompt::name(), "async_openai.Prompt");
    assert_eq!(
        ChatCompletionToolChoiceOption::name(),
        "async_openai.ChatCompletionToolChoiceOption"
    );
}

#[test]
fn upstream_components_coexist_with_same_named_downstream_types() {
    #[derive(ToSchema)]
    #[allow(dead_code)]
    struct CreateChatCompletionRequest {
        downstream_only: bool,
    }

    #[derive(OpenApi)]
    #[openapi(components(schemas(
        CreateChatCompletionRequest,
        async_openai::types::chat::CreateChatCompletionRequest
    )))]
    struct Combined;

    let doc = serde_json::to_value(Combined::openapi()).unwrap();
    let schemas = &doc["components"]["schemas"];
    assert!(schemas["CreateChatCompletionRequest"]["properties"]
        .get("downstream_only")
        .is_some());
    assert!(
        schemas["async_openai.CreateChatCompletionRequest"]["properties"]
            .get("messages")
            .is_some()
    );
    assert!(
        schemas["async_openai.CreateChatCompletionRequest"]["properties"]
            .get("downstream_only")
            .is_none()
    );
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

#[test]
fn documented_numeric_limits_are_schema_only() {
    let common = [
        ("n", 1, 128, true),
        ("temperature", 0, 2, false),
        ("top_p", 0, 1, false),
        ("frequency_penalty", -2, 2, false),
        ("presence_penalty", -2, 2, false),
    ];
    for chat in [true, false] {
        let (name, base, extra) = if chat {
            (
                CreateChatCompletionRequest::name(),
                json!({"model":"test","messages":[]}),
                vec![("top_logprobs", 0, 20, true)],
            )
        } else {
            (
                CreateCompletionRequest::name(),
                json!({"model":"test","prompt":"hi"}),
                vec![("logprobs", 0, 5, true), ("best_of", 0, 20, true)],
            )
        };
        let mut doc = document();
        doc["$ref"] = json!(format!("#/components/schemas/{name}"));
        let schema = jsonschema::validator_for(&doc).unwrap();
        assert!(
            schema.is_valid(&base),
            "optional fields must remain optional"
        );
        for (field, min, max, integer) in common.into_iter().chain(extra) {
            let property = &doc["components"]["schemas"][name.as_ref()]["properties"][field];
            assert_eq!(property["minimum"].as_f64(), Some(f64::from(min)));
            assert_eq!(property["maximum"].as_f64(), Some(f64::from(max)));
            for (number, expected) in [(min, true), (max, true), (min - 1, false), (max + 1, false)]
            {
                let mut value = base.clone();
                value[field] = if integer {
                    json!(number)
                } else {
                    json!(f64::from(number))
                };
                assert_eq!(
                    schema.is_valid(&value),
                    expected,
                    "{name}.{field}: {number}"
                );
            }
            let mut null = base.clone();
            null[field] = Value::Null;
            assert!(schema.is_valid(&null), "{name}.{field} remains nullable");
        }
    }
    // Documenting server limits does not introduce client-side range validation.
    assert!(serde_json::from_value::<CreateChatCompletionRequest>(
        json!({"model":"test","messages":[],"n":200,"top_logprobs":21,"temperature":3})
    )
    .is_ok());
    assert!(serde_json::from_value::<CreateCompletionRequest>(
        json!({"model":"test","prompt":"hi","n":200,"logprobs":6,"best_of":21})
    )
    .is_ok());
}

#[test]
fn logit_bias_limits_apply_to_values_and_preserve_optional_nullable_maps() {
    for chat in [true, false] {
        let (name, base) = if chat {
            (
                CreateChatCompletionRequest::name(),
                json!({"model":"test","messages":[]}),
            )
        } else {
            (
                CreateCompletionRequest::name(),
                json!({"model":"test","prompt":"hi"}),
            )
        };
        let mut doc = document();
        doc["$ref"] = json!(format!("#/components/schemas/{name}"));
        let bias = &doc["components"]["schemas"][name.as_ref()]["properties"]["logit_bias"];
        assert!(bias["description"]
            .as_str()
            .unwrap()
            .contains("-100 to 100"));
        assert_eq!(bias["propertyNames"]["type"], "string");
        let schema = jsonschema::validator_for(&doc).unwrap();
        assert!(schema.is_valid(&base));
        for bias in [Value::Null, json!({}), json!({"1":-100,"2":100,"3":0})] {
            let mut value = base.clone();
            value["logit_bias"] = bias;
            assert!(schema.is_valid(&value), "{name}: {value}");
        }
        for bias in [
            json!({"1":-101}),
            json!({"1":101}),
            json!({"1":"100"}),
            json!({"1":0.5}),
        ] {
            let mut value = base.clone();
            value["logit_bias"] = bias;
            assert!(!schema.is_valid(&value), "{name}: {value}");
        }
    }
    assert!(serde_json::from_value::<CreateChatCompletionRequest>(
        json!({"model":"test","messages":[],"logit_bias":{"1":101}})
    )
    .is_ok());
    // Legacy completion biases remain arbitrary JSON values in the Rust type.
    assert!(serde_json::from_value::<CreateCompletionRequest>(
        json!({"model":"test","prompt":"hi","logit_bias":{"1":"unchanged"}})
    )
    .is_ok());
}

#[test]
fn downstream_can_overwrite_and_remove_bounds_through_a_flattened_reference() {
    use utoipa::openapi::{schema::AdditionalProperties, RefOr, Schema};
    use utoipa::Modify;

    #[derive(Serialize, ToSchema)]
    struct DownstreamRequest {
        #[serde(flatten)]
        inner: CreateChatCompletionRequest,
    }

    struct Widen;
    impl Modify for Widen {
        fn modify(&self, api: &mut utoipa::openapi::OpenApi) {
            let RefOr::T(Schema::Object(request)) = api
                .components
                .as_mut()
                .unwrap()
                .schemas
                .get_mut("async_openai.CreateChatCompletionRequest")
                .unwrap()
            else {
                panic!("expected request object")
            };
            let RefOr::T(Schema::Object(n)) = request.properties.get_mut("n").unwrap() else {
                panic!("expected n schema")
            };
            n.maximum = Some(255.into());
            let RefOr::T(Schema::Object(bias)) = request.properties.get_mut("logit_bias").unwrap()
            else {
                panic!("expected bias map")
            };
            let AdditionalProperties::RefOr(RefOr::T(Schema::Object(value))) =
                bias.additional_properties.as_deref_mut().unwrap()
            else {
                panic!("expected bias value schema")
            };
            value.minimum = Some((-128).into());
            value.maximum = Some(127.into());
        }
    }

    #[derive(OpenApi)]
    #[openapi(components(schemas(DownstreamRequest)), modifiers(&Widen))]
    struct DownstreamApi;

    let mut doc = serde_json::to_value(DownstreamApi::openapi()).unwrap();
    doc["$ref"] = json!("#/components/schemas/DownstreamRequest");
    let wider = json!({"model":"test","messages":[],"n":200,"logit_bias":{"1":127}});
    assert!(!validator::<CreateChatCompletionRequest>().is_valid(&wider));
    assert!(jsonschema::validator_for(&doc).unwrap().is_valid(&wider));

    // Raw OpenAPI document edits can remove the constraints altogether.
    let properties =
        &mut doc["components"]["schemas"]["async_openai.CreateChatCompletionRequest"]["properties"];
    for field in ["n", "logit_bias"] {
        let property = if field == "logit_bias" {
            &mut properties[field]["additionalProperties"]
        } else {
            &mut properties[field]
        };
        let object = property.as_object_mut().unwrap();
        object.remove("minimum");
        object.remove("maximum");
    }
    assert!(jsonschema::validator_for(&doc)
        .unwrap()
        .is_valid(&json!({"model":"test","messages":[],"n":0,"logit_bias":{"1":-101}})));
    // Overrides affect only this exported document, not upstream Rust/schema behavior.
    assert!(!validator::<CreateChatCompletionRequest>().is_valid(&wider));
}
