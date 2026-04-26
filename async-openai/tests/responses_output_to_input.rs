//! Round-trip tests for `From<OutputItem> for Item` / `InputItem`.
//!
//! Reasoning models emit `Reasoning` items in `response.output` that the
//! Responses API requires you to echo back into the next request's `input`
//! alongside the function calls they preceded. These tests cover the
//! conversion paths the bug-prone variants rely on; the headline scenarios
//! are reasoning + function_call, which is what the API errors on if the
//! pairing is dropped (see issue #492).

#![cfg(feature = "response-types")]

use async_openai::types::responses::{
    ApplyPatchCallOutputStatus, ApplyPatchCallStatus, ApplyPatchOperation, ApplyPatchToolCall,
    ApplyPatchToolCallOutput, ApplyPatchUpdateFileOperation, CompactionBody, FunctionCallOutput,
    FunctionCallOutputStatusEnum, FunctionToolCall, FunctionToolCallOutputResource, InputItem,
    Item, OutputItem, ReasoningItem,
};
use serde_json::json;

#[test]
fn reasoning_round_trips_to_item() {
    // Synthesize a ReasoningItem the way the API would deliver one: an id
    // and an empty summary list (`encrypted_content` is the typical
    // payload, populated when `include: ["reasoning.encrypted_content"]`
    // is set on the request).
    let reasoning: ReasoningItem = serde_json::from_value(json!({
        "id": "rs_abc123",
        "type": "reasoning",
        "summary": [],
        "encrypted_content": "opaque-bytes",
    }))
    .expect("deserialize reasoning item");

    let output = OutputItem::Reasoning(reasoning.clone());
    let as_item: Item = output.into();
    match as_item {
        Item::Reasoning(r) => {
            assert_eq!(r.id, reasoning.id);
            assert_eq!(r.encrypted_content.as_deref(), Some("opaque-bytes"));
        }
        other => panic!("expected Item::Reasoning, got {other:?}"),
    }
}

#[test]
fn reasoning_then_function_call_round_trip_for_input() {
    // The headline scenario: a reasoning model emits Reasoning + FunctionCall
    // in its output; the next request must echo BOTH back so the API's
    // "function_call without its required reasoning item" check passes.
    let reasoning: ReasoningItem = serde_json::from_value(json!({
        "id": "rs_pair",
        "type": "reasoning",
        "summary": [],
    }))
    .unwrap();
    let function_call = FunctionToolCall {
        arguments: r#"{"location":"Paris","units":"celsius"}"#.into(),
        call_id: "call_pair".into(),
        name: "get_weather".into(),
        namespace: None,
        id: Some("fc_pair".into()),
        status: None,
    };

    let output = vec![
        OutputItem::Reasoning(reasoning),
        OutputItem::FunctionCall(function_call.clone()),
    ];

    let echoed: Vec<InputItem> = output.into_iter().map(InputItem::from).collect();
    assert_eq!(echoed.len(), 2);
    assert!(matches!(echoed[0], InputItem::Item(Item::Reasoning(_))));
    assert!(matches!(echoed[1], InputItem::Item(Item::FunctionCall(_))));
}

#[test]
fn function_call_output_resource_drops_required_id_into_optional() {
    // Resource side has `id: String` + `status: FunctionCallOutputStatusEnum`;
    // the input-side `*ItemParam` has both as Option. Conversion should
    // wrap them in Some so an echoed-back item carries the same identity.
    let resource = FunctionToolCallOutputResource {
        call_id: "call_42".into(),
        output: FunctionCallOutput::Text("ok".into()),
        id: "fco_42".into(),
        status: FunctionCallOutputStatusEnum::Completed,
        created_by: Some("svc".into()),
    };

    let item: Item = OutputItem::FunctionCallOutput(resource).into();
    match item {
        Item::FunctionCallOutput(p) => {
            assert_eq!(p.call_id, "call_42");
            assert_eq!(p.id.as_deref(), Some("fco_42"));
            assert!(p.status.is_some());
        }
        other => panic!("expected FunctionCallOutput, got {other:?}"),
    }
}

#[test]
fn apply_patch_call_status_folds_through() {
    let call = ApplyPatchToolCall {
        id: "apc_1".into(),
        call_id: "call_apc".into(),
        status: ApplyPatchCallStatus::Completed,
        operation: ApplyPatchOperation::UpdateFile(ApplyPatchUpdateFileOperation {
            path: "src/main.rs".into(),
            diff: "@@ -1 +1 @@\n-old\n+new\n".into(),
        }),
        created_by: None,
    };
    let item: Item = OutputItem::ApplyPatchCall(call).into();
    let Item::ApplyPatchCall(p) = item else {
        panic!("expected ApplyPatchCall");
    };
    assert_eq!(p.id.as_deref(), Some("apc_1"));
}

#[test]
fn apply_patch_call_output_status_failed_folds_through() {
    let out = ApplyPatchToolCallOutput {
        id: "apco_1".into(),
        call_id: "call_apco".into(),
        status: ApplyPatchCallOutputStatus::Failed,
        output: Some("patch did not apply cleanly".into()),
        created_by: None,
    };
    let item: Item = OutputItem::ApplyPatchCallOutput(out).into();
    let Item::ApplyPatchCallOutput(p) = item else {
        panic!("expected ApplyPatchCallOutput");
    };
    assert_eq!(p.output.as_deref(), Some("patch did not apply cleanly"));
}

#[test]
fn compaction_body_to_param() {
    let body = CompactionBody {
        id: "cmp_1".into(),
        encrypted_content: "encrypted-blob".into(),
        created_by: None,
    };
    let item: Item = OutputItem::Compaction(body).into();
    let Item::Compaction(p) = item else {
        panic!("expected Compaction");
    };
    assert_eq!(p.id.as_deref(), Some("cmp_1"));
    assert_eq!(p.encrypted_content, "encrypted-blob");
}
