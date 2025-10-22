use std::io::{stdout, Write};

use async_openai::tools::{Tool, ToolManager};
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
};
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use futures::StreamExt;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut messages = vec![ChatCompletionRequestUserMessageArgs::default()
        .content("What's the weather like in Boston and Atlanta?")
        .build()?
        .into()];

    let weather_tool = WeatherTool;
    let mut tool_manager = ToolManager::new();
    tool_manager.add_tool(weather_tool);
    let tools = tool_manager.get_tools_for_chat();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4-1106-preview")
        .messages(messages.clone())
        .tools(tools)
        .build()?;

    let response_message = client
        .chat()
        .create(request)
        .await?
        .choices
        .first()
        .unwrap()
        .message
        .clone();

    if let Some(tool_calls) = response_message.tool_calls {
        let assistant_messages: ChatCompletionRequestMessage =
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls.clone())
                .build()?
                .into();

        let function_responses = tool_manager.call_for_chat(tool_calls.clone()).await;
        let tool_messages: Vec<ChatCompletionRequestMessage> = function_responses
            .into_iter()
            .map(|res| res.into())
            .collect();

        messages.push(assistant_messages);
        messages.extend(tool_messages);

        let subsequent_request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u32)
            .model("gpt-4-1106-preview")
            .messages(messages)
            .build()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let mut stream = client.chat().create_stream(subsequent_request).await?;

        let mut response_content = String::new();
        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    for chat_choice in response.choices.iter() {
                        if let Some(ref content) = chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                            response_content.push_str(content);
                        }
                    }
                }
                Err(err) => {
                    return Err(Box::new(err) as Box<dyn std::error::Error>);
                }
            }
        }
    }

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
        let mut rng = thread_rng();

        let temperature: i32 = rng.gen_range(20..=55);

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
