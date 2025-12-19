use async_openai::{
    types::responses::{CreateResponseArgs, ResponseStreamEvent},
    Client,
};
use futures::StreamExt;
use std::error::Error;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // First, create a response with background=true and stream=true
    println!("Creating a respose with background=true and stream=true ...");
    let create_request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .background(true)
        .stream(true)
        .input("Write a function in Rust that adds two u32 and returns u64'")
        .build()?;

    let mut response = client.responses().create_stream(create_request).await?;
    let mut response_id = None;
    let mut lock = stdout().lock();

    while let Some(result) = response.next().await {
        if let Ok(ResponseStreamEvent::ResponseCreated(event)) = result {
            writeln!(lock, "Response created with ID: {}", event.response.id).unwrap();
            response_id = Some(event.response.id.clone());
            break;
        }
    }

    if let Some(response_id) = response_id {
        writeln!(lock, "\nRetrieving {} with streaming...\n", &response_id).unwrap();
        let mut retrieve_stream = client.responses().retrieve_stream(&response_id).await?;
        while let Some(result) = retrieve_stream.next().await {
            if let Ok(ResponseStreamEvent::ResponseOutputTextDelta(delta)) = result {
                write!(lock, "{}", delta.delta).unwrap();
            }
        }
    }

    Ok(())
}
