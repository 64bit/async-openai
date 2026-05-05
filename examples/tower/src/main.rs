use std::error::Error;
use std::time::Duration;

use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::middleware::ReqwestService;
use async_openai::retry::{OpenAIRetryLayer, SimpleRetryPolicy};
use async_openai::types::chat::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use async_openai::Client;
use tower::{util::BoxCloneSyncService, ServiceBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = ReqwestService::new(reqwest::Client::new());
    let service = ServiceBuilder::new()
        .concurrency_limit(1)
        .timeout(Duration::from_millis(700))
        // .timeout(Duration::from_millis(700))
        .layer(OpenAIRetryLayer::default())
        // .retry(SimpleRetryPolicy::default())
        .service(base);
    let service = BoxCloneSyncService::new(service);

    let client = Client::with_config(OpenAIConfig::default()).with_http_service(service);

    // Spawn several concurrent tasks to test the concurrency_limit
    let mut handles = vec![];
    for i in 0..1 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            // Build a simple chat completion request
            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-3.5-turbo")
                .messages([ChatCompletionRequestMessage::User("hello".into())])
                .build()
                .unwrap();

            let res: Result<CreateChatCompletionResponse, OpenAIError> =
                client.chat().create_byot(&request).await;
            match res {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            println!("Task {i}: Chat completion = {}", content);
                        } else {
                            println!("Task {i}: No content in completion.");
                        }
                    } else {
                        println!("Task {i}: No choices in completion response.");
                    }
                }
                Err(e) => {
                    println!("Task {i} error: {e} \n(debug: {e:?})");
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        // Wait for all tasks to complete
        let _ = handle.await;
    }
    // let models = client.models().list().await?;
    // println!("first model: {}", models.data[0].id);

    Ok(())
}
