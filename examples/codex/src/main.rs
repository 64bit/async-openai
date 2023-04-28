use std::error::Error;

use async_openai::{types::CreateCompletionRequestArgs, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "/// Rust language
/// Function to download a url and save it to disk.";

    let client = Client::openai();

    let request = CreateCompletionRequestArgs::default()
        .model("code-davinci-002")
        .prompt(prompt)
        .max_tokens(256_u16)
        .temperature(0.0)
        .top_p(1.0)
        .best_of(1)
        .logprobs(1)
        .echo(true)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let response = client.completions().create(request).await?;

    let choice = response.choices.iter().nth(0).unwrap();

    println!("{}", choice.text);

    Ok(())
}
