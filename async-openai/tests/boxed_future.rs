use futures::future::{BoxFuture, FutureExt};
use futures::StreamExt;

use async_openai::types::{CompletionResponseStream, CreateCompletionRequestArgs};
use async_openai::Client;

#[tokio::test]
async fn boxed_future_test() {
    fn interpret_bool(token_stream: &mut CompletionResponseStream) -> BoxFuture<'_, bool> {
        async move {
            // collect all responses from the stream
            let mut response = String::new();
            while let Some(next_token) = token_stream.next().await {
                match next_token {
                    Ok(next_token) => {
                        let token_str = &next_token.choices[0].text.trim();
                        if !token_str.is_empty() {
                            response.push_str(token_str);
                            response.push_str(" ");
                        }
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            println!("Response: {}", response);
            return response.contains("yes") || response.contains("Yes");
        }
        .boxed()
    }

    let client = Client::new();

    let mut request = CreateCompletionRequestArgs::default()
        .model("davinci-002")
        .n(1)
        .prompt("yes no yes no yes no")
        .stream(true)
        .logprobs(3)
        .max_tokens(64_u16)
        .build()
        .unwrap();

    let mut stream = client
        .completions()
        .create_stream(&mut request)
        .await
        .unwrap();

    let result = interpret_bool(&mut stream).await;
    assert!(result);
}
