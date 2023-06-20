use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, FunctionsArgs, Role,
    },
    Client,
};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-0613")
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("What's the weather like in Boston?")
            .build()?])
        .functions([FunctionsArgs::default()
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
        .function_call(json!("auto"))
        .build()?;

    println!("request = {:#?}", request);

    let response_message = client.chat().create(request).await?;
    // .choices
    // .get(0)
    // .unwrap()
    // .message
    // .clone();
    println!("resonse message: {:#?}", response_message);

    // if let Some(value) = response_message.function_call {
    //     let mut available_functions: HashMap<&str, fn(&str, &str) -> serde_json::Value> =
    //         HashMap::new();
    //     available_functions.insert("get_current_weather", get_current_weather);
    //     let function_name = value["function_call"]["name"].as_str().unwrap();
    //     println!("function name: {:?}", function_name);
    //     let function_args = &value["function_call"]["arguments"];
    //     let location = function_args["location"].as_str().unwrap();
    //     let unit = "fahrenheit";
    //     let function = available_functions.get(function_name).unwrap();
    //     let _function_response = function(location, unit);
    // }

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

#[test]
fn test_json() {
    // let json_value = json!({
    //     "type": "object",
    //     "properties": {
    //         "location": {
    //             "type": "string",
    //             "description": "The city and state, e.g. San Francisco, CA",
    //         },
    //         "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
    //     },
    //     "required": ["location"],
    // });

    // println!("json_value: {:?}", json_value);

    let ret = FunctionsArgs::default()
        .name("get_current_weather")
        .description("Get the current weather in a given location")
        .parameters(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA",
                },
                "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
            },
            "required": ["location"],
        }))
        .build();
    println!("ret = {:?}", ret);
}
