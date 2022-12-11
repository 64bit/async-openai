use async_openai as openai;
use openai::{error::OpenAIError, types::CreateCompletionRequest, Client, Completion};

async fn joke(client: &Client) -> Result<String, OpenAIError> {
    let request = CreateCompletionRequest {
        model: "text-davinci-003".to_owned(),
        prompt: Some("Tell me a joke".to_owned()),
        max_tokens: Some(30),
        ..Default::default()
    };

    let response = Completion::create(&client, request).await?;

    Ok(response.choices.first().unwrap().text.to_string())
}

#[tokio::main]
async fn main() {
    let backoff = backoff::ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
        .build();

    let client = Client::new().with_backoff(backoff);
    let mut count = 100;

    // Make back to back requests in a loop to trigger rate limits
    // which will be retried by exponential backoff
    while count > 0 {
        match joke(&client).await {
            Ok(joke) => println!("{joke}"),
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
        count -= 1;
    }
}
