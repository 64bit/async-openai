//! Integration test: verify `OutputMessage.id` is optional when echoed back.
//!
//! OpenAI's Responses API sometimes omits the `id` field on `output_text`
//! items when they appear in the input array (multi-turn conversations
//! passing assistant messages back). This test exercises the real API to
//! confirm the `Option<String>` change works end-to-end.
//!
//! Requires `OPENAI_API_KEY` environment variable. Skipped when absent.

#![cfg(feature = "responses")]

use async_openai::types::responses::{
    CreateResponseArgs, EasyInputMessage, InputItem, InputParam,
};
use async_openai::Client;

/// Smoke test: create a response, echo its output back as input, verify
/// the second request succeeds without deserialization errors.
#[tokio::test]
async fn output_message_id_round_trip() {
    let _key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("skipping: OPENAI_API_KEY not set");
            return;
        }
    };

    let client = Client::new();

    // Turn 1: ask a simple question
    let req1 = CreateResponseArgs::default()
        .model("gpt-4o-mini")
        .max_output_tokens(64u32)
        .input([EasyInputMessage::from("Say only the word 'hello'")])
        .build()
        .expect("build turn-1 request");

    let resp1 = client
        .responses()
        .create(req1)
        .await
        .expect("turn-1 API call");

    // Collect output items to echo back as input (the multi-turn pattern)
    let echoed: Vec<InputItem> = resp1
        .output
        .into_iter()
        .map(InputItem::from)
        .collect();

    assert!(!echoed.is_empty(), "expected at least one output item");

    // Verify every OutputMessage in the echoed items has an optional id
    // (some may have it, some may not — both should work)
    for item in &echoed {
        if let InputItem::Item(async_openai::types::responses::Item::Message(
            async_openai::types::responses::MessageItem::Output(msg),
        )) = item
        {
            // id is Option<String> — this line compiles only if the
            // struct field is indeed Option<String> (the fix).
            let _id: &Option<String> = &msg.id;
        }
    }

    // Turn 2: echo the output back as input
    let mut input_items: Vec<InputItem> = echoed;
    input_items.push(EasyInputMessage::from("What did I just say?").into());

    let req2 = CreateResponseArgs::default()
        .model("gpt-4o-mini")
        .max_output_tokens(64u32)
        .input(InputParam::Items(input_items))
        .build()
        .expect("build turn-2 request");

    let resp2 = client
        .responses()
        .create(req2)
        .await
        .expect("turn-2 API call (output_message echoed back)");

    // If we got here, deserialization succeeded — the fix works.
    // The second response should contain text output.
    let text = resp2.output_text();
    assert!(
        text.is_some() || resp2.output.is_empty(),
        "expected output text or empty output"
    );
}

/// Verify that constructing an InputItem from an OutputMessage with a
/// missing id field round-trips through the API without error.
#[tokio::test]
async fn output_message_without_id_in_input() {
    let _key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("skipping: OPENAI_API_KEY not set");
            return;
        }
    };

    let client = Client::new();

    // Build an input that includes an OutputMessage *without* an id,
    // simulating what the API sometimes returns.
    let input_item: InputItem = serde_json::from_value(serde_json::json!({
        "type": "message",
        "role": "assistant",
        "status": "completed",
        "content": [
            {"type": "output_text", "text": "The capital of France is Paris.", "annotations": []}
        ]
    }))
    .expect("deserialize OutputMessage without id as InputItem");

    let req = CreateResponseArgs::default()
        .model("gpt-4o-mini")
        .max_output_tokens(64u32)
        .input(InputParam::Items(vec![input_item, EasyInputMessage::from("Is that correct?").into()]))
        .build()
        .expect("build request with id-less output message in input");

    let resp = client
        .responses()
        .create(req)
        .await
        .expect("API call with OutputMessage missing id in input");

    // Success = no deserialization panic
    let _text = resp.output_text();
}
