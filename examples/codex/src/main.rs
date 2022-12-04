use std::error::Error;

use async_openai as openai;
use openai::{types::CreateCompletionRequest, Client, Completion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "
    /// Rust language
    /// Download a url and save it to filesystem. No tests.";

    let client = Client::new();

    let request = CreateCompletionRequest {
        model: "code-davinci-002".to_owned(),
        prompt: Some(prompt.to_owned()),
        max_tokens: Some(256),
        top_p: Some(1.0),
        best_of: Some(1),
        logprobs: Some(1),
        echo: Some(true),
        ..Default::default()
    };

    let response = Completion::create(&client, request).await?;

    println!("{response:#?}");

    let choice = response.choices.iter().nth(0).unwrap();

    println!("{}", choice.text);

    Ok(())
}
