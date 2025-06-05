use async_openai::{
    types::responses::{
        CreateResponseArgs, FunctionArgs, FunctionCall, Input, InputItem, InputMessageArgs,
        OutputContent, Role, ToolDefinition,
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

    let tools = vec![ToolDefinition::Function(
        FunctionArgs::default()
            .name("get_weather")
            .description("Retrieves current weather for the given location")
            .parameters(serde_json::json!(
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
            ))
            .build()?,
    )];

    let mut input_messages = vec![InputItem::Message(
        InputMessageArgs::default()
            .role(Role::User)
            .content("What's the weather like in Paris today?")
            .build()?,
    )];

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .input(Input::Items(input_messages.clone()))
        .tools(tools.clone())
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    // the model might ask for us to do a function call
    let function_call_request: Option<FunctionCall> =
        response.output.into_iter().find_map(|output_content| {
            if let OutputContent::FunctionCall(inner) = output_content {
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

    input_messages.push(InputItem::Custom(serde_json::to_value(
        &OutputContent::FunctionCall(function_call_request.clone()),
    )?));
    input_messages.push(InputItem::Custom(serde_json::json!({
        "type": "function_call_output",
        "call_id": function_call_request.call_id,
        "output": function_result,
    })));

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .input(Input::Items(input_messages))
        .tools(tools)
        .build()?;

    println!("request 2 {}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    println!("{}", serde_json::to_string(&response).unwrap());

    Ok(())
}
