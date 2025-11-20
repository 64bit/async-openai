use async_openai::{
    Client,
    traits::EventType,
    types::responses::{CreateResponseArgs, ResponseStreamEvent},
};
use futures::StreamExt;
use std::io::{Write, stdout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .stream(true)
        .input("Write a haiku about programming.")
        .build()?;

    let mut stream = client.responses().create_stream(request).await?;

    let mut lock = stdout().lock();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response_event) => match &response_event {
                ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                    write!(lock, "{}", delta.delta)?;
                }
                _ => {
                    writeln!(lock, "\n{}: skipping\n", response_event.event_type())?;
                }
            },
            Err(e) => {
                eprintln!("\n{e:#?}");
            }
        }
        stdout().flush()?;
    }

    Ok(())
}
