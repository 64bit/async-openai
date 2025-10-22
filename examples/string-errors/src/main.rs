//! This example demonstrates how errors from OpenRouter can be parsed by the library consumer.
//! It uses the `string-errors` feature to receive API errors as raw strings instead of parsed structs.

use async_openai::{config::OpenAIConfig, error::OpenAIError, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct OpenRouterError {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct ErrorWrapper {
    error: OpenRouterError,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new().with_api_base("https://openrouter.ai/api/v1");
    let client = Client::with_config(config);

    let result: Result<serde_json::Value, OpenAIError> = client
        .chat()
        .create_byot(json!({
            "model": "invalid-model",
            "messages": [{"role": "user", "content": "Hello"}]
        }))
        .await;

    match result.unwrap_err() {
        OpenAIError::ApiError(error_string) => {
            let error = serde_json::from_str::<ErrorWrapper>(&error_string).unwrap();
            println!("Code: {}", error.error.code);
            println!("Message: {}", error.error.message);
        }
        _ => panic!("Expected OpenAIError::ApiError"),
    }

    Ok(())
}
