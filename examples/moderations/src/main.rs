use async_openai::{
    types::{CreateModerationRequestArgs, TextModerationModel},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // single
    let request = CreateModerationRequestArgs::default()
        .input("Lions want to kill")
        .model(TextModerationModel::Latest)
        .build()?;

    let response = client.moderations().create(request).await?;

    println!("Response (single): {response:#?}");

    // multiple
    let request = CreateModerationRequestArgs::default()
        .input(["Lions want to kill", "I hate them"])
        .model(TextModerationModel::Latest)
        .build()?;

    let response = client.moderations().create(request).await?;

    println!("Response (multiple): {response:#?}");

    Ok(())
}
