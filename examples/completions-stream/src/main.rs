use async_openai::{types::CreateCompletionRequestArgs, Client};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .n(1)
        .prompt("Tell me a bedtime story about Optimus Prime and Bumblebee")
        .stream(true)
        .max_tokens(1024_u16)
        .build()?;

    let mut stream = client.completions().create_stream(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                print!("{}", c.text);
            }),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}
