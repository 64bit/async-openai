use async_openai::{
    types::{CreateCompletionRequest, Prompt},
    Client, Completion,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let completion_request = CreateCompletionRequest {
        model: "text-davinci-003".to_owned(),
        n: Some(1),
        prompt: Some(Prompt::String(
            "Tell me a bedtime story about Optimus Prime and Bumblebee".to_owned(),
        )),
        max_tokens: Some(1024),
        stream: Some(true),
        ..Default::default()
    };

    let mut stream = Completion::create_stream(&client, completion_request).await?;

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
