use std::error::Error;

use async_openai as openai;
use openai::{types::CreateCompletionRequest, Client, Completion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "/// Rust language
/// Function to download a url and save it disk.";

    let client = Client::new();

    let request = CreateCompletionRequest {
        model: "code-davinci-002".to_owned(),
        prompt: Some(prompt.to_owned()),
        max_tokens: Some(256),
        temperature: Some(0.0),
        top_p: Some(1.0),
        best_of: Some(1),
        logprobs: Some(1),
        echo: Some(true),
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        ..Default::default()
    };

    let response = Completion::create(&client, request).await?;

    let choice = response.choices.iter().nth(0).unwrap();

    println!("{}", choice.text);

    Ok(())
}
