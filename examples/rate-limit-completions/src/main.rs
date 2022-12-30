use std::error::Error;

use async_openai::{types::CreateCompletionRequestArgs, Client};

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

    let request = CreateCompletionRequestArgs::default()
        .model("text-ada-001")
        .prompt("Tell me a joke")
        .max_tokens(30_u16)
        .build()?;

    // Make back to back requests in a loop to trigger rate limits
    // which will be retried by exponential backoff
    for _ in 0..100 {
        let response = client.completions().create(request.clone()).await?;
        println!("{}", response.choices[0].text);
    }

    Ok(())
}
