use async_openai::{
    traits::RequestOptionsBuilder,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-3.5-turbo")
        .store(true)
        .metadata(json!({
            "role": "manager",
            "department": "accounting",
            "source": "homepage",
        }))
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a corporate IT support expert.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("How can I hide the dock on my Mac?")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("Chat Completion Response:\n");
    println!("{:#?}", response);

    // api doesnt return the chat completion immediately, so retrieval doesnt work immediately, sleep
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // get chat completion object
    let chat_completion = client.chat().retrieve(&response.id).await?;

    println!("--------------------------------");
    println!("Retrieved chat completion:\n");
    println!("{:#?}", chat_completion);

    let chat_completion_messages = client
        .chat()
        .query(&[("limit", 10)])?
        .messages(&response.id)
        .await?;

    println!("--------------------------------");
    println!("Retrieved chat completion messages:\n");
    println!("{:#?}", chat_completion_messages);

    // list all chat completions
    let chat_completions = client.chat().query(&[("limit", 10)])?.list().await?;

    println!("--------------------------------");
    println!("Retrieved chat completions:\n");
    println!("{:#?}", chat_completions);

    let deleted = client.chat().delete(&response.id).await?;

    println!("--------------------------------");
    println!("Deleted chat completion:\n");
    println!("{:#?}", deleted);

    Ok(())
}
