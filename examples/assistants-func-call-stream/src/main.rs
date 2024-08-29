use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantStreamEvent, CreateAssistantRequestArgs, CreateMessageRequest, CreateRunRequest,
        CreateThreadRequest, FunctionObject, MessageDeltaContent, MessageRole, RunObject,
        SubmitToolOutputsRunRequest, ToolsOutputs,
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
            )),
            strict: None,
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
            )),
            strict: None,
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

    let _message = client
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

    let mut task_handle = None;

    while let Some(event) = event_stream.next().await {
        match event {
            Ok(event) => match event {
                AssistantStreamEvent::ThreadRunRequiresAction(run_object) => {
                    println!("thread.run.requires_action: run_id:{}", run_object.id);
                    let client = client.clone();
                    task_handle = Some(tokio::spawn(async move {
                        handle_requires_action(client, run_object).await
                    }));
                }
                _ => println!("\nEvent: {event:?}\n"),
            },
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }

    // wait for task to handle required action and submit tool outputs
    if let Some(task_handle) = task_handle {
        let _ = tokio::join!(task_handle);
    }

    // clean up
    client.threads().delete(&thread.id).await?;
    client.assistants().delete(&assistant.id).await?;

    Ok(())
}

async fn handle_requires_action(client: Client<OpenAIConfig>, run_object: RunObject) {
    let mut tool_outputs: Vec<ToolsOutputs> = vec![];
    if let Some(ref required_action) = run_object.required_action {
        for tool in &required_action.submit_tool_outputs.tool_calls {
            if tool.function.name == "get_current_temperature" {
                tool_outputs.push(ToolsOutputs {
                    tool_call_id: Some(tool.id.clone()),
                    output: Some("57".into()),
                })
            }

            if tool.function.name == "get_rain_probability" {
                tool_outputs.push(ToolsOutputs {
                    tool_call_id: Some(tool.id.clone()),
                    output: Some("0.06".into()),
                })
            }
        }

        if let Err(e) = submit_tool_outputs(client, run_object, tool_outputs).await {
            eprintln!("Error on submitting tool outputs: {e}");
        }
    }
}

async fn submit_tool_outputs(
    client: Client<OpenAIConfig>,
    run_object: RunObject,
    tool_outputs: Vec<ToolsOutputs>,
) -> Result<(), Box<dyn Error>> {
    let mut event_stream = client
        .threads()
        .runs(&run_object.thread_id)
        .submit_tool_outputs_stream(
            &run_object.id,
            SubmitToolOutputsRunRequest {
                tool_outputs,
                stream: Some(true),
            },
        )
        .await?;

    while let Some(event) = event_stream.next().await {
        match event {
            Ok(event) => {
                if let AssistantStreamEvent::ThreadMessageDelta(delta) = event {
                    if let Some(contents) = delta.delta.content {
                        for content in contents {
                            // only text is expected here and no images
                            if let MessageDeltaContent::Text(text) = content {
                                if let Some(text) = text.text {
                                    if let Some(text) = text.value {
                                        print!("{}", text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }

    Ok(())
}
