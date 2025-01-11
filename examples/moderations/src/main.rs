use async_openai::{types::CreateModerationRequestArgs, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model = "omni-moderation-latest";

    // single
    let request = CreateModerationRequestArgs::default()
        .input("Lions want to kill")
        .model(model)
        .build()?;

    let response = client.moderations().create(request).await?;

    println!("Response (single): {response:#?}");

    // multiple
    let request = CreateModerationRequestArgs::default()
        .input(["Lions want to kill", "I hate them"])
        .model(model)
        .build()?;

    let response = client.moderations().create(request).await?;

    println!("Response (multiple): {response:#?}");

    Ok(())
}
