use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, Write};

use async_openai::types::{
    ChatCompletionRequestFunctionMessageArgs, ChatCompletionRequestUserMessageArgs, FinishReason,
};
use async_openai::{
    types::{ChatCompletionFunctionsArgs, CreateChatCompletionRequestArgs},
    Client,
};

use async_openai::config::OpenAIConfig;
use futures::StreamExt;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model = "gpt-4o-mini";

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("What's the weather like in Boston?")
            .build()?
            .into()])
        .functions([ChatCompletionFunctionsArgs::default()
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
            .build()?])
        .function_call("auto")
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    let mut fn_name = String::new();
    let mut fn_args = String::new();

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                for chat_choice in response.choices {
                    if let Some(fn_call) = &chat_choice.delta.function_call {
                        writeln!(lock, "function_call: {:?}", fn_call).unwrap();
                        if let Some(name) = &fn_call.name {
                            fn_name.clone_from(name);
                        }
                        if let Some(args) = &fn_call.arguments {
                            fn_args.push_str(args);
                        }
                    }
                    if let Some(finish_reason) = &chat_choice.finish_reason {
                        if matches!(finish_reason, FinishReason::FunctionCall) {
                            call_fn(&client, &fn_name, &fn_args).await?;
                        }
                    } else if let Some(content) = &chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                }
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}

async fn call_fn(
    client: &Client<OpenAIConfig>,
    name: &str,
    args: &str,
) -> Result<(), Box<dyn Error>> {
    let mut available_functions: HashMap<&str, fn(&str, &str) -> serde_json::Value> =
        HashMap::new();
    available_functions.insert("get_current_weather", get_current_weather);

    let function_args: serde_json::Value = args.parse().unwrap();

    let model = "gpt-4o-mini";
    let location = function_args["location"].as_str().unwrap();
    let unit = function_args["unit"].as_str().unwrap_or("fahrenheit");
    let function = available_functions.get(name).unwrap();
    let function_response = function(location, unit); // call the function

    let message = vec![
        ChatCompletionRequestUserMessageArgs::default()
            .content("What's the weather like in Boston?")
            .build()?
            .into(),
        ChatCompletionRequestFunctionMessageArgs::default()
            .content(function_response.to_string())
            .name(name)
            .build()?
            .into(),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model(model)
        .messages(message)
        .build()?;

    // Now stream received response from model, which essentially formats the function response
    let mut stream = client.chat().create_stream(request).await?;

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }
    println!("\n");
    Ok(())
}

fn get_current_weather(location: &str, unit: &str) -> serde_json::Value {
    let weather_info = json!({
            "location": location,
            "temperature": "72",
            "unit": unit,
            "forecast": ["sunny", "windy"]
    });

    weather_info
}
