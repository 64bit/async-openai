use std::error::Error;

use async_openai::{
    types::{
        AssistantStreamEvent, CreateAssistantRequestArgs, CreateMessageRequest, CreateRunRequest,
        CreateThreadRequest, FunctionObject, MessageRole,
    },
    Client,
};
use futures::StreamExt;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "ERROR");

    // Setup tracing subscriber so that library can log the errors
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new();

    //
    // Step 1: Define functions
    //

    let create_assistant_request = CreateAssistantRequestArgs::default()
    .instructions("You are a weather bot. Use the provided functions to answer questions.")
    .model("gpt-4o")
    .tools(vec![
        FunctionObject {
            name: "get_current_temperature".into(),
            description: Some("Get the current temperature for a specific location".into()),
            parameters: Some(serde_json::json!(
                {
                    "type": "object",
                    "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g., San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["Celsius", "Fahrenheit"],
                        "description": "The temperature unit to use. Infer this from the user's location."
                    }
                    },
                    "required": ["location", "unit"]
                }
            ))
        }.into(),

        FunctionObject {
            name: "get_rain_probability".into(),
            description: Some("Get the probability of rain for a specific location".into()),
            parameters: Some(serde_json::json!(
                {
                    "type": "object",
                    "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g., San Francisco, CA"
                    }
                    },
                    "required": ["location"]
                }
            ))
        }.into()
    ]).build()?;

    let assistant = client.assistants().create(create_assistant_request).await?;

    //
    // Step 2: Create a Thread and add Messages
    //
    let thread = client
        .threads()
        .create(CreateThreadRequest::default())
        .await?;

    let message = client
        .threads()
        .messages(&thread.id)
        .create(CreateMessageRequest {
            role: MessageRole::User,
            content: "What's the weather in San Francisco today and the likelihood it'll rain?"
                .into(),
            ..Default::default()
        })
        .await?;

    //
    // Step 3: Initiate a Run
    //
    let mut event_stream = client
        .threads()
        .runs(&thread.id)
        .create_stream(CreateRunRequest {
            assistant_id: assistant.id.clone(),
            stream: Some(true),
            ..Default::default()
        })
        .await?;

    while let Some(event) = event_stream.next().await {
        match event {
            Ok(event) => match event {
                AssistantStreamEvent::ThreadRunRequiresAction(action) => {
                    println!("thread.run.requires_action: {action:?}");
                }
                _ => println!("Event: {event:?}"),
            },
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }

    // clean up
    client.threads().delete(&thread.id).await?;
    client.assistants().delete(&assistant.id).await?;

    Ok(())
}
