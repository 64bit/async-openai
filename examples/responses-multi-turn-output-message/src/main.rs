//! Multi-turn Responses API passing assistant output messages back as input.
//!
//! The Responses API supports passing prior assistant messages back in the
//! `input` array of a follow-up request so the model sees its own prior turn.
//! In practice the `OutputMessage` returned in `response.output` is fed back
//! via the `MessageItem::Output` variant of the input `Item` enum.
//!
//! The OpenAI API may return `OutputMessage` items with or without an `id`
//! field (especially for items reconstructed from previous turns). The Rust
//! SDK models `OutputMessage.id` as `Option<String>` for exactly this reason
//! — this example round-trips the output through the input and runs a second
//! turn to confirm that the API still responds normally.
//!
//! Run with: `OPENAI_API_KEY=... cargo run -p responses-multi-turn-output-message`

use std::error::Error;

use async_openai::types::responses::{
    CreateResponseArgs, EasyInputMessage, InputItem, InputParam, Item, MessageItem, OutputItem,
    OutputMessageContent,
};
use async_openai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Turn 1: ask the model something that produces an assistant message.
    let request = CreateResponseArgs::default()
        .model("gpt-4.1-mini")
        .input("Reply with a single short sentence acknowledging you understand JSON.")
        .build()?;
    let response = client.responses().create(request).await?;

    // Capture the assistant's reply text and the full OutputMessage we got
    // back. The latter is what we'll echo into the next request as input.
    let assistant_text = response
        .output
        .iter()
        .find_map(|item| match item {
            OutputItem::Message(msg) => msg
                .content
                .iter()
                .find_map(|c| match c {
                    OutputMessageContent::OutputText(ot) => Some(ot.text.clone()),
                    _ => None,
                }),
            _ => None,
        })
        .ok_or("model did not return a text message")?;

    println!("Turn 1 reply: {assistant_text}");

    // Pull the OutputMessage(s) back out. `From<OutputItem> for InputItem`
    // routes Message variants through `MessageItem::Output`, so this is the
    // canonical way to feed assistant messages back in.
    let echoed_items: Vec<InputItem> = response
        .output
        .into_iter()
        .map(InputItem::from)
        .collect();

    // Quick check: did the SDK actually deserialize the OutputMessage we got
    // back from the API? This is the property the `OutputMessage.id` fix
    // protects — if the API ever returns an item without `id`, deserialization
    // still succeeds and we still hand a valid input item back to the server.
    for item in &echoed_items {
        if let InputItem::Item(Item::Message(MessageItem::Output(m))) = item {
            println!(
                "Echoed OutputMessage: id={:?} content_count={}",
                m.id,
                m.content.len()
            );
        }
    }

    // Turn 2: build a fresh input list — a user question plus the echoed
    // assistant output — and ask the model to follow up. The echoed items
    // go through the input array untouched, exercising the OutputMessage
    // round-trip in real API traffic.
    let mut input_items: Vec<InputItem> =
        vec![EasyInputMessage::from("Summarise your previous reply in 6 words or fewer.").into()];
    input_items.extend(echoed_items);

    let request = CreateResponseArgs::default()
        .model("gpt-4.1-mini")
        .input(InputParam::Items(input_items))
        .build()?;
    let response = client.responses().create(request).await?;

    if let Some(text) = response.output_text() {
        println!("Turn 2 reply: {text}");
    } else {
        println!("(no text in turn 2 reply)");
    }

    Ok(())
}