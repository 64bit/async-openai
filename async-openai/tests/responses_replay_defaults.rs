#![cfg(feature = "response-types")]

use async_openai::types::responses::{ComputerScreenContent, OutputMessage, OutputStatus};
use serde_json::json;

#[test]
fn replayed_message_defaults_missing_status_and_annotations() {
    let message: OutputMessage = serde_json::from_value(json!({
        "id": "msg_1",
        "role": "assistant",
        "content": [{"type": "output_text", "text": "hello"}]
    }))
    .unwrap();

    assert_eq!(message.status, OutputStatus::Completed);
    let value = serde_json::to_value(message).unwrap();
    assert_eq!(value["status"], "completed");
    assert_eq!(value["content"][0]["annotations"], json!([]));
}

#[test]
fn replayed_message_preserves_explicit_status_and_annotations() {
    let message: OutputMessage = serde_json::from_value(json!({
        "id": "msg_1",
        "role": "assistant",
        "status": "incomplete",
        "content": [{
            "type": "output_text",
            "text": "hello",
            "annotations": [{"type": "file_citation", "file_id": "file_1", "filename": "notes.txt", "index": 0}]
        }]
    }))
    .unwrap();

    assert_eq!(message.status, OutputStatus::Incomplete);
    let value = serde_json::to_value(message).unwrap();
    assert_eq!(value["content"][0]["annotations"][0]["file_id"], "file_1");
}

#[test]
fn screenshot_cache_breakpoint_round_trips_and_is_omitted_when_absent() {
    let value = json!({
        "image_url": "https://example.com/screenshot.png",
        "file_id": null,
        "prompt_cache_breakpoint": {"mode": "explicit"}
    });
    let screen: ComputerScreenContent = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(screen).unwrap(), value);

    let screen: ComputerScreenContent =
        serde_json::from_value(json!({"file_id": "file_1"})).unwrap();
    assert!(serde_json::to_value(screen)
        .unwrap()
        .get("prompt_cache_breakpoint")
        .is_none());
}
