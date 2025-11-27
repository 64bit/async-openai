use std::io::{stdout, Write};

use async_openai::types::chat::{
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestToolMessage, ChatCompletionRequestUserMessage, ChatCompletionTool,
    FinishReason, FunctionObjectArgs,
};
use async_openai::{types::chat::CreateChatCompletionRequestArgs, Client};
use futures::StreamExt;
use rand::seq::IndexedRandom;
use rand::{rng, Rng};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_prompt = "What's the weather like in Boston and Atlanta?";

    // Create the initial request using ergonomic From traits
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
                    "required": ["location"],
                }))
                .build()?,
        })
        .build()?;

    // Stream the initial response and collect tool calls
    let mut stream = client.chat().create_stream(request).await?;
    let mut tool_calls = Vec::new();
    let mut execution_handles = Vec::new();
    let mut stdout_lock = stdout().lock();

    // First stream: collect tool calls, print content, and start executing tool calls as soon as they're complete
    while let Some(result) = stream.next().await {
        let response = result?;

        for choice in response.choices {
            // Print any content deltas
            if let Some(content) = &choice.delta.content {
                write!(stdout_lock, "{}", content)?;
            }

            // Collect tool call chunks
            if let Some(tool_call_chunks) = choice.delta.tool_calls {
                for chunk in tool_call_chunks {
                    let index = chunk.index as usize;

                    // Ensure we have enough space in the vector
                    while tool_calls.len() <= index {
                        tool_calls.push(ChatCompletionMessageToolCall {
                            id: String::new(),
                            function: Default::default(),
                        });
                    }

                    // Update the tool call with chunk data
                    let tool_call = &mut tool_calls[index];
                    if let Some(id) = chunk.id {
                        tool_call.id = id;
                    }
                    if let Some(function_chunk) = chunk.function {
                        if let Some(name) = function_chunk.name {
                            tool_call.function.name = name;
                        }
                        if let Some(arguments) = function_chunk.arguments {
                            tool_call.function.arguments.push_str(&arguments);
                        }
                    }
                }
            }

            // When tool calls are complete, start executing them immediately
            if matches!(choice.finish_reason, Some(FinishReason::ToolCalls)) {
                // Spawn execution tasks for all collected tool calls
                for tool_call in tool_calls.iter() {
                    let name = tool_call.function.name.clone();
                    let args = tool_call.function.arguments.clone();
                    let tool_call_id = tool_call.id.clone();

                    let handle = tokio::spawn(async move {
                        let result = call_function(&name, &args).await;
                        (tool_call_id, result)
                    });
                    execution_handles.push(handle);
                }
            }
        }
        stdout_lock.flush()?;
    }

    // Wait for all tool call executions to complete (outside the stream loop)
    if !execution_handles.is_empty() {
        let mut tool_responses = Vec::new();
        for handle in execution_handles {
            let (tool_call_id, response) = handle.await?;
            tool_responses.push((tool_call_id, response));
        }

        // Build the follow-up request using ergonomic From traits
        let mut messages: Vec<ChatCompletionRequestMessage> =
            vec![ChatCompletionRequestUserMessage::from(user_prompt).into()];

        // Add assistant message with tool calls
        let assistant_tool_calls: Vec<ChatCompletionMessageToolCalls> = tool_calls
            .iter()
            .map(|tc| tc.clone().into()) // From<ChatCompletionMessageToolCall>
            .collect();
        messages.push(
            ChatCompletionRequestAssistantMessage {
                content: None,
                tool_calls: Some(assistant_tool_calls),
                ..Default::default()
            }
            .into(),
        );

        // Add tool response messages
        for (tool_call_id, response) in tool_responses {
            messages.push(
                ChatCompletionRequestToolMessage {
                    content: response.to_string().into(),
                    tool_call_id,
                }
                .into(),
            );
        }

        // Second stream: get the final response
        let follow_up_request = CreateChatCompletionRequestArgs::default()
            .max_completion_tokens(512u32)
            .model("gpt-5-mini")
            .messages(messages)
            .build()?;

        let mut follow_up_stream = client.chat().create_stream(follow_up_request).await?;

        while let Some(result) = follow_up_stream.next().await {
            let response = result?;
            for choice in response.choices {
                if let Some(content) = &choice.delta.content {
                    write!(stdout_lock, "{}", content)?;
                }
            }
            stdout_lock.flush()?;
        }
    }

    Ok(())
}

async fn call_function(name: &str, args: &str) -> serde_json::Value {
    match name {
        "get_current_weather" => get_current_weather(args),
        _ => json!({"error": format!("Unknown function: {}", name)}),
    }
}

fn get_current_weather(args: &str) -> serde_json::Value {
    let args: serde_json::Value = args.parse().unwrap_or(json!({}));
    let location = args["location"]
        .as_str()
        .unwrap_or("unknown location")
        .to_string();
    let unit = args["unit"].as_str().unwrap_or("fahrenheit");

    let mut rng = rng();
    let temperature: i32 = rng.random_range(20..=55);
    let forecasts = [
        "sunny", "cloudy", "overcast", "rainy", "windy", "foggy", "snowy",
    ];
    let forecast = forecasts.choose(&mut rng).unwrap_or(&"sunny");

    json!({
        "location": location,
        "temperature": temperature.to_string(),
        "unit": unit,
        "forecast": forecast
    })
}
