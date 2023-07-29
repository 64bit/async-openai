use std::io::{stdout, Write};
use std::collections::HashMap;
use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionFunctionsArgs, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

use futures::StreamExt;
use serde_json::json;


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

    // the first response from GPT is just the json response containing the function that was called
		// and the model-generated arguments for that function (don't stream this)
    let response = client
				.chat()
				.create(request)
				.await?
				.choices
				.get(0)
				.unwrap()
				.message
				.clone();
		
		if let Some(function_call) = response.function_call {
				let mut available_functions: HashMap<&str, fn(&str, &str) -> serde_json::Value> =
						HashMap::new();
				available_functions.insert("get_current_weather", get_current_weather);
				
				let function_name = function_call.name;
				let function_args: serde_json::Value = function_call.arguments.parse().unwrap();

				let location = function_args["location"].as_str().unwrap();
				let unit = "fahrenheit"; // why doesn't the model return a unit argument?
				let function = available_functions.get(function_name.as_str()).unwrap();
        let function_response = function(location, unit); // call the function

				let message = vec![
						ChatCompletionRequestMessageArgs::default()
								.role(Role::User)
								.content("What's the weather like in Boston?")
								.build()?,
						ChatCompletionRequestMessageArgs::default()
								.role(Role::Function)
								.content(function_response.to_string())
								.name(function_name)
								.build()?
				];

				let request = CreateChatCompletionRequestArgs::default()
						.max_tokens(512u16)
						.model("gpt-3.5-turbo-0613")
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
				println!("{}", "\n");
		}

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
