use async_openai::{
    tools::{Tool, ToolManager},
    types::responses::{
        CreateResponseArgs, FunctionCall, Input, InputItem, InputMessageArgs, OutputContent, Role,
    },
    Client,
};
use rand::{rng, seq::IndexedRandom, Rng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let weather_tool = WeatherTool;
    let mut tool_manager = ToolManager::new();
    tool_manager.add_tool(weather_tool);
    let tools = tool_manager.get_tools_for_responses();

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

    for output_content in response.output.clone() {
        input_messages.push(InputItem::Custom(serde_json::to_value(output_content)?));
    }

    // the model might ask for us to do a function call
    let function_call_request: Vec<FunctionCall> = response
        .output
        .into_iter()
        .filter_map(|output_content| {
            if let OutputContent::FunctionCall(inner) = output_content {
                Some(inner)
            } else {
                None
            }
        })
        .collect();

    if function_call_request.is_empty() {
        println!("No function_call request found");
        return Ok(());
    };

    let function_result = tool_manager
        .call_for_responses(function_call_request.clone())
        .await;

    input_messages.extend(function_result);

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

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
enum Unit {
    Fahrenheit,
    Celsius,
}

#[derive(Debug, JsonSchema, Deserialize)]
struct WeatherRequest {
    /// The city and state, e.g. San Francisco, CA
    location: String,
    unit: Unit,
}

#[derive(Debug, Serialize)]
struct WeatherResponse {
    location: String,
    temperature: String,
    unit: Unit,
    forecast: String,
}

struct WeatherTool;

impl Tool for WeatherTool {
    type Args = WeatherRequest;
    type Output = WeatherResponse;
    type Error = String;

    fn name() -> String {
        "get_current_weather".to_string()
    }

    fn description() -> Option<String> {
        Some("Get the current weather in a given location".to_string())
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut rng = rng();

        let temperature: i32 = rng.random_range(20..=55);

        let forecasts = [
            "sunny", "cloudy", "overcast", "rainy", "windy", "foggy", "snowy",
        ];

        let forecast = forecasts.choose(&mut rng).unwrap_or(&"sunny");

        let weather_info = WeatherResponse {
            location: args.location,
            temperature: temperature.to_string(),
            unit: args.unit,
            forecast: forecast.to_string(),
        };

        Ok(weather_info)
    }
}
