use std::error::Error;

use async_openai as openai;
use openai::{
    types::{CreateModerationRequest, Input, TextModerationModel},
    Client, Moderation,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // single
    let request = CreateModerationRequest {
        input: Input::Single("Lions want to kill".to_string()),
        model: Some(TextModerationModel::Latest),
    };

    let response = Moderation::create(&client, request).await?;

    println!("Response (single): {response:#?}");

    // multiple
    let request = CreateModerationRequest {
        input: Input::Array(vec![
            "Lions want to kill".to_string(),
            "I hate them".to_string(),
        ]),
        model: Some(TextModerationModel::Latest),
    };

    let response = Moderation::create(&client, request).await?;

    println!("Response (multiple): {response:#?}");

    Ok(())
}
