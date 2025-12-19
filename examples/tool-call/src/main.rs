use std::collections::HashMap;
use std::io::{stdout, Write};

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestToolMessage,
    ChatCompletionRequestUserMessage, ChatCompletionTool, FunctionObjectArgs,
};
use async_openai::{types::chat::CreateChatCompletionRequestArgs, Client};
use futures::StreamExt;
use rand::seq::IndexedRandom;
use rand::{rng, Rng};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_prompt = "What's the weather like in Boston and Atlanta?";

    let request = CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(512u32)
        .model("gpt-5-mini")
        .messages(ChatCompletionRequestUserMessage::from(user_prompt))
        .tools(ChatCompletionTool {
            function: FunctionObjectArgs::default()
                .name("get_current_weather")
                .description("Get the current weather in a given location")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA",
                        },
                        "unit": { "type": "string", "enum": ["celsius", "fahrenheit"] },
                    },
                    "required": ["location", "unit"],
                    "additionalProperties": false
                }))
                .strict(true)
                .build()?,
        })
        .build()?;

    let response_message = client
        .chat()
        .create(request)
        .await?
        .choices
        .first()
        .ok_or("No choices")?
        .message
        .clone();

    if let Some(tool_calls) = response_message.tool_calls {
        let mut handles = Vec::new();
        for tool_call_enum in tool_calls {
            // Extract the function tool call from the enum
            if let ChatCompletionMessageToolCalls::Function(tool_call) = tool_call_enum {
                let name = tool_call.function.name.clone();
                let args = tool_call.function.arguments.clone();
                let tool_call_clone = tool_call.clone();

                let handle =
                    tokio::spawn(async move { call_fn(&name, &args).await.unwrap_or_default() });
                handles.push((handle, tool_call_clone));
            }
        }

        let mut function_responses = Vec::new();

        for (handle, tool_call_clone) in handles {
            if let Ok(response_content) = handle.await {
                function_responses.push((tool_call_clone, response_content));
            }
        }

        let mut messages: Vec<ChatCompletionRequestMessage> =
            ChatCompletionRequestUserMessage::from(user_prompt).into();

        // Convert ChatCompletionMessageToolCall to ChatCompletionMessageToolCalls enum
        let tool_calls: Vec<ChatCompletionMessageToolCalls> = function_responses
            .iter()
            .map(|(tool_call, _response_content)| {
                ChatCompletionMessageToolCalls::Function(tool_call.clone())
            })
            .collect();

        let assistant_messages: ChatCompletionRequestMessage =
            ChatCompletionRequestAssistantMessageArgs::default()
                .tool_calls(tool_calls)
                .build()?
                .into();

        let tool_messages: Vec<ChatCompletionRequestMessage> = function_responses
            .iter()
            .map(|(tool_call, response_content)| {
                ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
                    content: response_content.to_string().into(),
                    tool_call_id: tool_call.id.clone(),
                })
            })
            .collect();

        messages.push(assistant_messages);
        messages.extend(tool_messages);

        let subsequent_request = CreateChatCompletionRequestArgs::default()
            .max_completion_tokens(512u32)
            .model("gpt-5-mini")
            .messages(messages)
            .build()?;

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

async fn call_fn(name: &str, args: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut available_functions: HashMap<&str, fn(&str, &str) -> serde_json::Value> =
        HashMap::new();
    available_functions.insert("get_current_weather", get_current_weather);

    let function_args: serde_json::Value = args.parse().unwrap();

    let location = function_args["location"].as_str().unwrap();
    let unit = function_args["unit"].as_str().unwrap_or("fahrenheit");
    let function = available_functions.get(name).unwrap();
    let function_response = function(location, unit);
    Ok(function_response)
}

fn get_current_weather(location: &str, unit: &str) -> serde_json::Value {
    let mut rng = rng();

    let temperature: i32 = rng.random_range(20..=55);

    let forecasts = [
        "sunny", "cloudy", "overcast", "rainy", "windy", "foggy", "snowy",
    ];

    let forecast = forecasts.choose(&mut rng).unwrap_or(&"sunny");

    let weather_info = json!({
        "location": location,
        "temperature": temperature.to_string(),
        "unit": unit,
        "forecast": forecast
    });

    weather_info
}
