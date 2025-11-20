use async_openai::{
    types::responses::{
        CreateResponseArgs, EasyInputMessage, FunctionCallOutput, FunctionCallOutputItemParam,
        FunctionTool, FunctionToolCall, InputItem, InputParam, Item, OutputItem, Tool,
    },
    Client,
};
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
        name: "get_weather".to_string(),
        description: Some("Retrieves current weather for the given location".to_string()),
        parameters: Some(serde_json::json!(
            {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "City and country e.g. Bogotá, Colombia"
                    },
                    "units": {
                        "type": "string",
                        "enum": [
                            "celsius",
                            "fahrenheit"
                        ],
                        "description": "Units the temperature will be returned in."
                    }
                },
                "required": [
                    "location",
                    "units"
                ],
                "additionalProperties": false
            }
        )),
        strict: None,
    })];

    let mut input_items: Vec<InputItem> =
        vec![EasyInputMessage::from("What's the weather like in Paris today?").into()];

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .input(InputParam::Items(input_items.clone()))
        .tools(tools.clone())
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    // the model might ask for us to do a function call
    let function_call_request: Option<FunctionToolCall> =
        response.output.into_iter().find_map(|output_item| {
            if let OutputItem::FunctionCall(inner) = output_item {
                Some(inner)
            } else {
                None
            }
        });

    let Some(function_call_request) = function_call_request else {
        println!("No function_call request found");
        return Ok(());
    };

    let function_result = match function_call_request.name.as_str() {
        "get_weather" => {
            let args: WeatherFunctionArgs = serde_json::from_str(&function_call_request.arguments)?;
            check_weather(args.location, args.units)
        }
        _ => {
            println!("Unknown function {}", function_call_request.name);
            return Ok(());
        }
    };

    // Add the function call from the assistant back to the conversation
    input_items.push(InputItem::Item(Item::FunctionCall(
        function_call_request.clone(),
    )));

    // Add the function call output back to the conversation
    input_items.push(InputItem::Item(Item::FunctionCallOutput(
        FunctionCallOutputItemParam {
            call_id: function_call_request.call_id.clone(),
            output: FunctionCallOutput::Text(function_result),
            id: None,
            status: None,
        },
    )));

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .input(InputParam::Items(input_items))
        .tools(tools)
        .build()?;

    println!("request 2 {}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    println!("{}", serde_json::to_string(&response).unwrap());

    Ok(())
}
