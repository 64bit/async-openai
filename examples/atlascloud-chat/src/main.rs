use std::{env, error::Error};

use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

const ATLASCLOUD_API_BASE: &str = "https://api.atlascloud.ai/v1";
const DEFAULT_MODEL: &str = "qwen/qwen3.5-flash";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("ATLASCLOUD_API_KEY")
        .map_err(|_| "ATLASCLOUD_API_KEY must be set to call Atlas Cloud")?;
    let model = env::var("ATLASCLOUD_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base(ATLASCLOUD_API_BASE)
            .with_api_key(api_key),
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(256_u32)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a concise assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Write one sentence about Rust async clients.")
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            println!("{content}");
        }
    }

    Ok(())
}
