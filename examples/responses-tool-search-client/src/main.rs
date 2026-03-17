use std::error::Error;

use async_openai::{
    types::responses::{
        CreateResponseArgs, EasyInputMessage, FunctionToolArgs, InputParam, Item, OutputItem,
        OutputStatus, Tool, ToolSearchExecution, ToolSearchOutput, ToolSearchToolArgs,
        FunctionCallOutput, FunctionCallOutputItemParam,
    },
    Client,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ShippingEtaArgs {
    order_id: String,
}

fn get_shipping_eta(order_id: &str) -> String {
    format!(
        "{{\"order_id\":\"{order_id}\",\"estimated_delivery\":\"2026-03-19\",\"status\":\"in_transit\"}}"
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let user_prompt = "Find the shipping ETA tool, load it, and call it for order_42.";

    let tool_search = ToolSearchToolArgs::default()
        .execution(ToolSearchExecution::Client)
        .description("Find the project-specific tools needed to continue the task.")
        .parameters(json!({
            "type": "object",
            "properties": {
                "goal": { "type": "string" }
            },
            "required": ["goal"],
            "additionalProperties": false
        }))
        .build()?;

    let first_request = CreateResponseArgs::default()
        .model("gpt-5.4")
        .input(user_prompt)
        .parallel_tool_calls(false)
        .tools(vec![tool_search.into()])
        .build()?;

    let first_response = client.responses().create(first_request).await?;

    let mut prior_items = Vec::new();
    let mut search_call = None;

    for item in first_response.output {
        match item {
            OutputItem::ToolSearchCall(call) => {
                search_call = Some(call.clone());
                prior_items.push(Item::ToolSearchCall(call));
            }
            OutputItem::Message(message) => {
                prior_items.push(Item::Message(message.into()));
            }
            _ => {}
        }
    }

    let search_call = search_call.expect("expected tool_search_call in the first response");

    println!(
        "Client tool_search_call: {}",
        serde_json::to_string_pretty(&search_call)?
    );

    let loaded_tool = FunctionToolArgs::default()
        .name("get_shipping_eta")
        .description("Look up shipping ETA details for a provided order ID.")
        .defer_loading(true)
        .parameters(json!({
            "type": "object",
            "properties": {
                "order_id": { "type": "string" }
            },
            "required": ["order_id"],
            "additionalProperties": false
        }))
        .strict(true)
        .build()?;

    let tool_search_output = ToolSearchOutput {
        call_id: search_call.call_id.clone(),
        execution: Some(ToolSearchExecution::Client),
        id: None,
        status: Some(OutputStatus::Completed),
        tools: vec![Tool::Function(loaded_tool)],
        created_by: None,
    };

    let mut second_input = vec![EasyInputMessage::from(user_prompt).into()];
    second_input.extend(prior_items.into_iter().map(Into::into));
    second_input.push(Item::ToolSearchOutput(tool_search_output.clone()).into());
    let mut third_input = second_input.clone();

    let second_request = CreateResponseArgs::default()
        .model("gpt-5.4")
        .input(InputParam::Items(second_input))
        .build()?;

    let second_response = client.responses().create(second_request).await?;
    let mut function_call_request = None;

    for item in second_response.output {
        match item {
            OutputItem::Message(message) => {
                println!("Intermediate assistant message: {:?}", message);
                third_input.push(Item::Message(message.into()).into());
            }
            OutputItem::FunctionCall(call) => {
                println!(
                    "Loaded function call: namespace={:?} name={} arguments={}",
                    call.namespace, call.name, call.arguments
                );
                function_call_request = Some(call.clone());
                third_input.push(Item::FunctionCall(call).into());
            }
            other => {
                println!("Other output item: {:?}", other);
            }
        }
    }

    let Some(function_call_request) = function_call_request else {
        println!("No function call was requested after tool search.");
        return Ok(());
    };

    let args: ShippingEtaArgs = serde_json::from_str(&function_call_request.arguments)?;
    let function_result = get_shipping_eta(&args.order_id);
    println!("Function result: {}", function_result);

    third_input.push(
        Item::FunctionCallOutput(FunctionCallOutputItemParam {
            call_id: function_call_request.call_id,
            output: FunctionCallOutput::Text(function_result),
            id: None,
            status: None,
        })
        .into(),
    );

    let third_request = CreateResponseArgs::default()
        .model("gpt-5.4")
        .input(InputParam::Items(third_input))
        .build()?;

    let third_response = client.responses().create(third_request).await?;

    for item in third_response.output {
        if let OutputItem::Message(message) = item {
            println!("Final assistant message: {:?}", message);
        }
    }

    Ok(())
}
