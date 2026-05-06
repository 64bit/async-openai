use std::error::Error;
use std::time::Duration;

use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
#[allow(unused_imports)]
use async_openai::middleware::{
    retry::{OpenAIRetryLayer, SimpleRetryPolicy},
    ReqwestService,
};
use async_openai::types::chat::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use async_openai::Client;
use tower::{util::BoxCloneSyncService, ServiceBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = ReqwestService::new(reqwest::Client::new());
    let service = ServiceBuilder::new()
        .concurrency_limit(10)
        .timeout(Duration::from_secs(1))
        .layer(OpenAIRetryLayer::default())
        // .retry(SimpleRetryPolicy::default()) / use this or the layer above but not both
        .service(base);
    let service = BoxCloneSyncService::new(service);

    let client = Client::with_config(OpenAIConfig::default()).with_http_service(service);

    // Spawn several concurrent tasks to test the concurrency_limit
    let mut handles = vec![];
    for i in 0..10 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            // Build a simple chat completion request
            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-5.4-mini")
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

    Ok(())
}
