use std::error::Error;

use async_openai::{
    traits::RequestOptionsBuilder,
    types::chat::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-3.5-turbo")
        .messages([
            // Can also use ChatCompletionRequest<Role>MessageArgs for builder pattern
            ChatCompletionRequestSystemMessage::from("You are a helpful assistant.").into(),
            ChatCompletionRequestUserMessage::from("Who won the world series in 2020?").into(),
            ChatCompletionRequestAssistantMessage::from(
                "The Los Angeles Dodgers won the World Series in 2020.",
            )
            .into(),
            ChatCompletionRequestUserMessage::from("Where was it played?").into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client
        .chat()
        .query(&vec![("limit", 10)])?
        .create(request)
        .await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}
