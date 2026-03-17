use std::error::Error;

use async_openai::{
    types::responses::{
        CreateResponseArgs, EasyInputMessage, FunctionCallOutput, FunctionCallOutputItemParam,
        FunctionToolArgs, InputItem, InputParam, Item, NamespaceToolArgs, OutputItem,
        ToolSearchToolArgs,
    },
    Client,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ListOpenOrdersArgs {
    customer_id: String,
}

fn list_open_orders(customer_id: &str) -> String {
    format!(
        "{{\"customer_id\":\"{customer_id}\",\"open_orders\":[{{\"order_id\":\"ORD-1001\",\"status\":\"processing\"}},{{\"order_id\":\"ORD-1002\",\"status\":\"awaiting_shipment\"}}]}}"
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let user_prompt = "List open orders for customer CUST-12345.";

    let get_customer_profile = FunctionToolArgs::default()
        .name("get_customer_profile")
        .description("Fetch a customer profile by customer ID.")
        .parameters(json!({
            "type": "object",
            "properties": {
                "customer_id": { "type": "string" }
            },
            "required": ["customer_id"],
            "additionalProperties": false
        }))
        .strict(true)
        .build()?;

    let list_open_orders_tool = FunctionToolArgs::default()
        .name("list_open_orders")
        .description("List open orders for a customer ID.")
        .defer_loading(true)
        .parameters(json!({
            "type": "object",
            "properties": {
                "customer_id": { "type": "string" }
            },
            "required": ["customer_id"],
            "additionalProperties": false
        }))
        .strict(true)
        .build()?;

    let crm_namespace = NamespaceToolArgs::default()
        .name("crm")
        .description("CRM tools for customer lookup and order management.")
        .tools(vec![
            get_customer_profile.clone().into(),
            list_open_orders_tool.clone().into(),
        ])
        .build()?;

    let tool_search = ToolSearchToolArgs::default().build()?;
    let tools = vec![crm_namespace.into(), tool_search.into()];

    let request = CreateResponseArgs::default()
        .model("gpt-5.4")
        .input(user_prompt)
        .parallel_tool_calls(false)
        .tools(tools.clone())
        .build()?;

    let first_response = client.responses().create(request).await?;
    let mut prior_items: Vec<InputItem> = vec![EasyInputMessage::from(user_prompt).into()];
    let mut function_call_request = None;

    for item in first_response.output {
        match item {
            OutputItem::ToolSearchCall(call) => {
                println!(
                    "Hosted tool_search_call: {}",
                    serde_json::to_string_pretty(&call)?
                );
                prior_items.push(Item::ToolSearchCall(call).into());
            }
            OutputItem::ToolSearchOutput(output) => {
                println!(
                    "Hosted tool_search_output: {}",
                    serde_json::to_string_pretty(&output)?
                );
                prior_items.push(Item::ToolSearchOutput(output).into());
            }
            OutputItem::FunctionCall(call) => {
                println!(
                    "Function call: namespace={:?} name={} arguments={}",
                    call.namespace, call.name, call.arguments
                );
                function_call_request = Some(call.clone());
                prior_items.push(Item::FunctionCall(call).into());
            }
            other => {
                println!("Other output item: {:?}", other);
            }
        }
    }

    let Some(function_call_request) = function_call_request else {
        println!("No function call was requested.");
        return Ok(());
    };

    let args: ListOpenOrdersArgs = serde_json::from_str(&function_call_request.arguments)?;
    let function_result = list_open_orders(&args.customer_id);
    println!("Function result: {}", function_result);

    prior_items.push(Item::FunctionCallOutput(FunctionCallOutputItemParam {
        call_id: function_call_request.call_id,
        output: FunctionCallOutput::Text(function_result),
        id: None,
        status: None,
    }).into());

    let final_request = CreateResponseArgs::default()
        .model("gpt-5.4")
        .input(InputParam::Items(prior_items))
        .parallel_tool_calls(false)
        .tools(tools)
        .build()?;

    let final_response = client.responses().create(final_request).await?;

    for item in final_response.output {
        if let OutputItem::Message(message) = item {
            println!("Final assistant message: {:?}", message);
        }
    }

    Ok(())
}
