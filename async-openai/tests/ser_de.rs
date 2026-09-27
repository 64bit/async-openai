use async_openai::types::chat::{
    ChatCompletionMessageToolCallChunk, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionStreamOptions, CreateChatCompletionRequest,
    CreateChatCompletionRequestArgs, FunctionCallStream, FunctionType,
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

#[test]
fn tool_call_chunk_none_fields_not_serialized() {
    // Streaming continuation deltas carry only the index and an arguments fragment;
    // OpenAI omits id and type there. openai-python's stream accumulator overwrites
    // `type` with each delta because it is a union tag, so an explicit null erases the
    // "function" sent in the first delta and fails its tool type assertion.
    let continuation = ChatCompletionMessageToolCallChunk {
        index: 0,
        id: None,
        r#type: None,
        function: Some(FunctionCallStream {
            name: None,
            arguments: Some("{\"ci".to_string()),
        }),
    };

    let serialized = serde_json::to_string(&continuation).unwrap();
    assert_eq!(
        serialized,
        r#"{"index":0,"function":{"arguments":"{\"ci"}}"#
    );

    // The first delta keeps every field that is set.
    let first = ChatCompletionMessageToolCallChunk {
        index: 0,
        id: Some("call_1".to_string()),
        r#type: Some(FunctionType::Function),
        function: Some(FunctionCallStream {
            name: Some("get_weather".to_string()),
            arguments: None,
        }),
    };

    let serialized_first = serde_json::to_string(&first).unwrap();
    assert_eq!(
        serialized_first,
        r#"{"index":0,"id":"call_1","type":"function","function":{"name":"get_weather"}}"#
    );

    // Test when all optional fields are None
    let empty = ChatCompletionMessageToolCallChunk {
        index: 0,
        id: None,
        r#type: None,
        function: None,
    };

    let serialized_empty = serde_json::to_string(&empty).unwrap();
    assert_eq!(serialized_empty, r#"{"index":0}"#);

    // Test roundtrip deserialization
    for chunk in [continuation, first, empty] {
        let serialized = serde_json::to_string(&chunk).unwrap();
        let deserialized: ChatCompletionMessageToolCallChunk =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(chunk, deserialized);
    }
}
