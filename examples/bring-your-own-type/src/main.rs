use std::{
    error::Error,
    io::{stdout, Write},
    pin::Pin,
};

use async_openai::{config::OpenAIConfig, error::OpenAIError, Client};
use futures::StreamExt;
use futures_core::Stream;

use serde_json::{json, Value};

async fn chat(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "developer",
                    "content": "You are a helpful assistant"
                },
                {
                    "role": "user",
                    "content": "What do you think about life?"
                }
            ],
            "model": "gpt-4o",
            "store": false
        }))
        .await?;

    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}

type MyStreamingType = Pin<Box<dyn Stream<Item = Result<Value, OpenAIError>> + Send>>;

async fn chat_stream(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    let mut stream: MyStreamingType = client
        .chat()
        .create_stream_byot(json!({
            "messages": [
                {
                    "role": "developer",
                    "content": "You are a helpful assistant"
                },
                {
                    "role": "user",
                    "content": "What do you think about life?"
                }
            ],
            "model": "gpt-4o",
            "stream": true
        }))
        .await?;

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        if let Ok(chunk) = result {
            if let Some(content) = chunk["choices"][0]["delta"]["content"].as_str() {
                write!(lock, "{}", content).unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    chat(&client).await?;
    chat_stream(&client).await?;
    Ok(())
}
