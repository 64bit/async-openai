use std::error::Error;

use async_openai::{types::CreateCompletionRequestArgs, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // single
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt("Tell me a joke about the universe")
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;

    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

    // multiple
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt([
            "How old is the human civilization?",
            "How old is the Earth?",
        ])
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;

    println!("\nResponse (multiple):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
