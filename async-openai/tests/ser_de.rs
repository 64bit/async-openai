use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionStreamOptions, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    FunctionCallStream,
};

#[test]
fn chat_types_serde() {
    let request: CreateChatCompletionRequest = CreateChatCompletionRequestArgs::default()
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("your are a calculator")
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("what is the result of 1+1")
                .build()
                .unwrap()
                .into(),
        ])
        .build()
        .unwrap();
    // serialize the request
    let serialized = serde_json::to_string(&request).unwrap();
    // deserialize the request
    let deserialized: CreateChatCompletionRequest = serde_json::from_str(&serialized).unwrap();
    assert_eq!(request, deserialized);
}

#[test]
fn stream_options_none_fields_not_serialized() {
    // When include_obfuscation is None, it should not appear in the serialized JSON.
    // This is important for OpenAI-compatible providers (like NVIDIA NIM) that reject unknown fields.
    let stream_options = ChatCompletionStreamOptions {
        include_usage: Some(true),
        include_obfuscation: None,
    };

    let serialized = serde_json::to_string(&stream_options).unwrap();

    // Verify include_usage is present
    assert!(serialized.contains("include_usage"));
    // Verify include_obfuscation is NOT present (not even as null)
    assert!(
        !serialized.contains("include_obfuscation"),
        "include_obfuscation should not be serialized when None, but got: {}",
        serialized
    );

    // Test when both are None
    let stream_options_empty = ChatCompletionStreamOptions {
        include_usage: None,
        include_obfuscation: None,
    };

    let serialized_empty = serde_json::to_string(&stream_options_empty).unwrap();
    assert_eq!(serialized_empty, "{}");

    // Test roundtrip deserialization
    let deserialized: ChatCompletionStreamOptions = serde_json::from_str(&serialized).unwrap();
    assert_eq!(stream_options, deserialized);
}

#[test]
fn function_call_stream_none_fields_not_serialized() {
    // When name or arguments is None, it should not appear in the serialized JSON.
    // Streaming consumers that read function.arguments with a string default
    // (e.g. `dict.get('arguments', '')`) crash on explicit JSON null because the
    // key is present-but-null rather than absent.
    let fcs = FunctionCallStream {
        name: Some("get_weather".to_string()),
        arguments: None,
    };

    let serialized = serde_json::to_string(&fcs).unwrap();

    // Verify name is present
    assert!(serialized.contains("name"));
    // Verify arguments is NOT present (not even as null)
    assert!(
        !serialized.contains("arguments"),
        "arguments should not be serialized when None, but got: {}",
        serialized
    );

    // Test when both are None
    let fcs_empty = FunctionCallStream {
        name: None,
        arguments: None,
    };

    let serialized_empty = serde_json::to_string(&fcs_empty).unwrap();
    assert_eq!(serialized_empty, "{}");

    // Test roundtrip deserialization
    let deserialized: FunctionCallStream = serde_json::from_str(&serialized).unwrap();
    assert_eq!(fcs, deserialized);
}
