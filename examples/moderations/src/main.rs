use async_openai::{
    types::{CreateModerationRequest, ModerationInput, TextModerationModel},
    Client, Moderation,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // single
    let request = CreateModerationRequest {
        input: ModerationInput::String("Lions want to kill".to_owned()),
        model: Some(TextModerationModel::Latest),
    };

    let response = Moderation::create(&client, request).await?;

    println!("Response (single): {response:#?}");

    // multiple
    let request = CreateModerationRequest {
        input: ModerationInput::StringArray(vec![
            "Lions want to kill".to_owned(),
            "I hate them".to_owned(),
        ]),
        model: Some(TextModerationModel::Latest),
    };

    let response = Moderation::create(&client, request).await?;

    println!("Response (multiple): {response:#?}");

    Ok(())
}
