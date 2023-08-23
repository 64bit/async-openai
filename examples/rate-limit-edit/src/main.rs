use std::error::Error;

use async_openai::{types::CreateEditRequestArgs, Client};

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This should come from env var outside the program
    std::env::set_var("RUST_LOG", "warn");

    // Setup tracing subscriber so that library can log the rate limited message
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let backoff = backoff::ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
        .build();

    let client = Client::new().with_backoff(backoff);

    let request = CreateEditRequestArgs::default()
        .model("text-davinci-edit-001")
        .input("The food was delicious and the waiter...")
        .instruction("make it 20 words long")
        .build()?;

    // Make back to back requests in a loop to trigger rate limits
    // which will be retried by exponential backoff

    // Limit: 20 RPM

    for idx in 0..100 {
        let response = client.edits().create(request.clone()).await?;
        println!("[{idx}] {:?}", response.usage);
    }

    Ok(())
}
