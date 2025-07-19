use async_openai::types::responses::*;
use serde_json;

#[test]
fn test_response_event_deserialization() {
    // Test basic streaming events - using actual JSON from responses API
    let created_json = r#"{
        "type": "response.created",
        "sequence_number": 0,
        "response": {
            "id": "resp_68819584a96881a082f70cbb524d3b6c00c0da7b27b3d5bd",
            "object": "response",
            "created_at": 1753322884,
            "status": "in_progress",
            "model": "o3-2025-04-16",
            "background": false,
            "service_tier": "auto",
            "top_logprobs": 0,
            "output": [],
            "parallel_tool_calls": true,
            "reasoning": {
                "effort": "medium"
            },
            "store": true,
            "temperature": 1.0,
            "text": {
                "format": {
                    "type": "text"
                }
            },
            "tool_choice": "auto",
            "tools": [],
            "top_p": 1.0,
            "truncation": "disabled",
            "metadata": {}
        }
    }"#;

    let delta_json = r#"{
        "type": "response.output_text.delta",
        "sequence_number": 6,
        "item_id": "msg_688195877b9081a088b67ef1d8707db800c0da7b27b3d5bd",
        "output_index": 1,
        "content_index": 0,
        "delta": "Silent",
        "logprobs": []
    }"#;

    let completed_json = r#"{
        "type": "response.completed",
        "sequence_number": 26,
        "response": {
            "id": "resp_68819584a96881a082f70cbb524d3b6c00c0da7b27b3d5bd",
            "object": "response",
            "created_at": 1753322884,
            "status": "completed",
            "model": "o3-2025-04-16",
            "usage": {
                "input_tokens": 13,
                "input_tokens_details": {
                    "audio_tokens": null,
                    "cached_tokens": 0
                },
                "output_tokens": 151,
                "output_tokens_details": {
                    "accepted_prediction_tokens": null,
                    "audio_tokens": null,
                    "reasoning_tokens": 128,
                    "rejected_prediction_tokens": null
                },
                "total_tokens": 164
            },
            "background": false,
            "service_tier": "auto",
            "top_logprobs": 0,
            "output": [
                {
                    "id": "rs_68819585260c81a0b001a62df6c4164000c0da7b27b3d5bd",
                    "type": "reasoning",
                    "summary": []
                },
                {
                    "id": "msg_688195877b9081a088b67ef1d8707db800c0da7b27b3d5bd",
                    "type": "message",
                    "status": "completed",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "Silent lines of code  \nLogic weaves through glowing night  \nBugs flee with sunrise",
                            "annotations": [],
                            "logprobs": []
                        }
                    ],
                    "role": "assistant"
                }
            ],
            "parallel_tool_calls": true,
            "reasoning": {
                "effort": "medium"
            },
            "store": true,
            "temperature": 1.0,
            "text": {
                "format": {
                    "type": "text"
                }
            },
            "tool_choice": "auto",
            "tools": [],
            "top_p": 1.0,
            "truncation": "disabled",
            "metadata": {}
        }
    }"#;

    // Test deserialization
    let created: ResponseEvent = serde_json::from_str(created_json).unwrap();
    let delta: ResponseEvent = serde_json::from_str(delta_json).unwrap();
    let completed: ResponseEvent = serde_json::from_str(completed_json).unwrap();

    assert!(matches!(created, ResponseEvent::ResponseCreated(_)));
    assert!(matches!(delta, ResponseEvent::ResponseOutputTextDelta(_)));
    assert!(matches!(completed, ResponseEvent::ResponseCompleted(_)));

    // Test serialization round-trip
    let created_serialized = serde_json::to_string(&created).unwrap();
    let _: ResponseEvent = serde_json::from_str(&created_serialized).unwrap();
}

#[test]
fn test_response_event_unknown() {
    // Test Unknown event handling for completely unknown event types
    let unknown_json = r#"{
        "type": "response.future_feature",
        "sequence_number": 42,
        "some_new_field": "value"
    }"#;

    let event: ResponseEvent = serde_json::from_str(unknown_json).unwrap();
    match event {
        ResponseEvent::Unknown(value) => {
            assert_eq!(value.get("type").unwrap().as_str().unwrap(), "response.future_feature");
            assert_eq!(value.get("sequence_number").unwrap().as_u64().unwrap(), 42);
            assert_eq!(value.get("some_new_field").unwrap().as_str().unwrap(), "value");
        }
        _ => panic!("Expected Unknown event"),
    }
}
