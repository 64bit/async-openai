use async_openai::{
    traits::EventType,
    types::responses::{
        CreateResponseArgs, EasyInputMessage, FunctionCallOutput, FunctionCallOutputItemParam,
        FunctionTool, FunctionToolCall, InputItem, InputParam, Item, OutputItem,
        ResponseStreamEvent, Tool,
    },
    Client,
};
use clap::Parser;
use futures::StreamExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, Write};

#[derive(Debug, Deserialize)]
struct WeatherFunctionArgs {
    location: String,
    units: String,
}

fn check_weather(location: String, units: String) -> String {
    format!("The weather in {location} is 25 {units}")
}

#[derive(Parser, Debug)]
#[command(name = "responses-function-call")]
#[command(about = "Example demonstrating function calls with the Responses API")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Run non-streaming function call example
    NonStreaming,
    /// Run streaming function call example
    Streaming,
    /// Run both streaming and non-streaming examples
    All,
}

async fn run_non_streaming() -> Result<(), Box<dyn Error>> {
    println!("=== Non-Streaming Function Call Example ===\n");

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

    println!("Request: {}", serde_json::to_string(&request)?);
    println!("\n---\n");

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

    println!(
        "Function call requested: {} with arguments: {}",
        function_call_request.name, function_call_request.arguments
    );

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

    println!("Function result: {}\n", function_result);

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

    println!("Second request: {}", serde_json::to_string(&request)?);
    println!("\n---\n");

    let response = client.responses().create(request).await?;

    println!("Final response: {}", serde_json::to_string(&response)?);

    Ok(())
}

async fn run_streaming() -> Result<(), Box<dyn Error>> {
    println!("=== Streaming Function Call Example ===\n");

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
        .stream(true)
        .input(InputParam::Items(input_items.clone()))
        .tools(tools.clone())
        .build()?;

    println!("Request: {}", serde_json::to_string(&request)?);
    println!("\n---\n");

    let mut stream = client.responses().create_stream(request).await?;

    // Track function call arguments as they stream in
    let mut function_call_args: HashMap<String, String> = HashMap::new();
    // Track function call metadata (name, call_id) by item_id
    let mut function_call_metadata: HashMap<String, (String, String)> = HashMap::new();
    let mut function_call_request: Option<FunctionToolCall> = None;
    let mut stdout_lock = stdout().lock();

    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => {
                match &event {
                    ResponseStreamEvent::ResponseOutputItemAdded(added) => {
                        // When a function call item is added, extract the call_id
                        if let OutputItem::FunctionCall(fc) = &added.item {
                            let item_id = fc.id.clone().unwrap_or_default();
                            function_call_metadata
                                .insert(item_id.clone(), (fc.name.clone(), fc.call_id.clone()));
                            writeln!(stdout_lock, "{}: {}\n", added.event_type(), fc.name)?;
                        }
                    }
                    ResponseStreamEvent::ResponseFunctionCallArgumentsDelta(delta) => {
                        // Accumulate function call arguments
                        let args = function_call_args
                            .entry(delta.item_id.clone())
                            .or_insert_with(String::new);
                        args.push_str(&delta.delta);
                        write!(stdout_lock, "{}: {}\n", delta.event_type(), delta.delta)?;
                        stdout().flush()?;
                    }
                    ResponseStreamEvent::ResponseFunctionCallArgumentsDone(done) => {
                        // Function call arguments are complete
                        if let Some((name, call_id)) = function_call_metadata.get(&done.item_id) {
                            let arguments = function_call_args
                                .remove(&done.item_id)
                                .unwrap_or_else(|| done.arguments.clone());

                            writeln!(
                                stdout_lock,
                                "{}: [Function call complete: {}]",
                                done.event_type(),
                                name
                            )?;
                            writeln!(
                                stdout_lock,
                                "{}: Arguments: {}\n",
                                done.event_type(),
                                arguments
                            )?;

                            // Create the function call request
                            function_call_request = Some(FunctionToolCall {
                                name: name.clone(),
                                arguments: arguments,
                                call_id: call_id.clone(),
                                id: Some(done.item_id.clone()),
                                status: None,
                            });
                        }
                    }
                    ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                        write!(stdout_lock, "{}: {}\n", delta.event_type(), delta.delta)?;
                        stdout().flush()?;
                    }
                    _ => {
                        writeln!(stdout_lock, "{}: skipping\n", event.event_type())?;
                    }
                }
            }
            Err(e) => {
                writeln!(stdout_lock, "\nError: {:?}", e)?;
                return Err(Box::new(e));
            }
        }
    }

    // Execute the function call if we have one
    let Some(function_call_request) = function_call_request else {
        println!("\nNo function_call request found");
        return Ok(());
    };

    println!("\n---\n");

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

    println!("Function result: {}\n", function_result);

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
        .stream(true)
        .input(InputParam::Items(input_items))
        .tools(tools)
        .build()?;

    println!("Second request: {}", serde_json::to_string(&request)?);
    println!("\n---\n");
    println!("Final response (streaming):\n");

    let mut stream = client.responses().create_stream(request).await?;
    let mut stdout_lock = stdout().lock();

    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => match &event {
                ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                    write!(stdout_lock, "{}: {}\n", delta.event_type(), delta.delta)?;
                    stdout().flush()?;
                }
                _ => {
                    writeln!(stdout_lock, "{}: skipping\n", event.event_type())?;
                }
            },
            Err(e) => {
                writeln!(stdout_lock, "\nError: {:?}", e)?;
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::NonStreaming => run_non_streaming().await,
        Command::Streaming => run_streaming().await,
        Command::All => {
            run_non_streaming().await?;
            println!("\n\n");
            run_streaming().await
        }
    }
}
