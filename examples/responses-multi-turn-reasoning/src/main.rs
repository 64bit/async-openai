//! Multi-turn Responses API with a reasoning model.
//!
//! Reasoning models (gpt-5 and friends) emit `Reasoning` items in
//! `response.output` alongside any `FunctionCall` items they produce. The
//! Responses API requires those reasoning items to be echoed back into the
//! next request's `input` together with the function calls they preceded —
//! otherwise the server rejects the request with:
//!
//! ```text
//! Item 'fc_...' of type 'function_call' was provided without its required 'reasoning' item: 'rs_...'
//! ```
//!
//! In the official Python and Node SDKs the canonical pattern is just
//! `input_list += response.output`. In Rust the input/output schemas are
//! split (`Item` vs `OutputItem`), so this example uses the
//! `From<OutputItem> for InputItem` conversion to do the same thing.
//!
//! Run with: `OPENAI_API_KEY=... cargo run -p responses-multi-turn-reasoning`

use async_openai::types::responses::{
    CreateResponseArgs, EasyInputMessage, FunctionCallOutput, FunctionCallOutputItemParam,
    FunctionTool, FunctionToolCall, InputItem, InputParam, Item, OutputItem, Tool,
};
use async_openai::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct WeatherFunctionArgs {
    location: String,
    units: String,
}

fn check_weather(location: String, units: String) -> String {
    format!("The weather in {location} is 25 {units}")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let tools = vec![Tool::Function(FunctionTool {
        defer_loading: None,
        name: "get_weather".to_string(),
        description: Some("Retrieves current weather for the given location".to_string()),
        parameters: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City and country e.g. Bogotá, Colombia"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Units the temperature will be returned in."
                }
            },
            "required": ["location", "units"],
            "additionalProperties": false
        })),
        strict: Some(true),
    })];

    let mut input_items: Vec<InputItem> =
        vec![EasyInputMessage::from("What's the weather like in Paris today?").into()];

    // Turn 1: ask the model. With a reasoning model, response.output will
    // typically contain one or more Reasoning items followed by a
    // FunctionCall item.
    let request = CreateResponseArgs::default()
        .model("gpt-5")
        .input(InputParam::Items(input_items.clone()))
        .tools(tools.clone())
        .build()?;
    let response = client.responses().create(request).await?;

    // Pull out the function call we need to execute, but DO NOT consume
    // `response.output` yet — we need to round-trip the whole thing
    // (reasoning items included) into the next request.
    let function_call: FunctionToolCall = response
        .output
        .iter()
        .find_map(|item| match item {
            OutputItem::FunctionCall(fc) => Some(fc.clone()),
            _ => None,
        })
        .ok_or("model did not request a function call")?;

    let function_result = match function_call.name.as_str() {
        "get_weather" => {
            let args: WeatherFunctionArgs = serde_json::from_str(&function_call.arguments)?;
            check_weather(args.location, args.units)
        }
        other => return Err(format!("unknown function {other}").into()),
    };

    // The fix: append the entire `response.output` into our running input,
    // converted from OutputItem to InputItem. This carries the Reasoning
    // items along with the FunctionCall, satisfying the API's pairing
    // requirement on the next turn.
    //
    // Without this — for instance, pushing only the FunctionCall back —
    // the next request fails with:
    //   "Item 'fc_...' was provided without its required 'reasoning' item"
    input_items.extend(response.output.into_iter().map(InputItem::from));

    // And then the function call's result.
    input_items.push(InputItem::Item(Item::FunctionCallOutput(
        FunctionCallOutputItemParam {
            call_id: function_call.call_id.clone(),
            output: FunctionCallOutput::Text(function_result),
            id: None,
            status: None,
        },
    )));

    // Turn 2: the model now sees its own reasoning trail and the tool
    // result, and can compose a final answer.
    let request = CreateResponseArgs::default()
        .model("gpt-5")
        .input(InputParam::Items(input_items))
        .tools(tools)
        .build()?;
    let response = client.responses().create(request).await?;

    if let Some(text) = response.output_text() {
        println!("{text}");
    } else {
        println!("(no text in final response)");
    }

    Ok(())
}
