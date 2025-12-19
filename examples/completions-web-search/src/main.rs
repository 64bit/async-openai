use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, WebSearchContextSize, WebSearchLocation,
    WebSearchOptions, WebSearchUserLocation, WebSearchUserLocationType,
};
use async_openai::{types::chat::CreateChatCompletionRequestArgs, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_prompt = "What is the weather like today? Be concise.";

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(256u32)
        .model("gpt-4o-mini-search-preview")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(user_prompt)
            .build()?
            .into()])
        .web_search_options(WebSearchOptions {
            search_context_size: Some(WebSearchContextSize::Low),
            user_location: Some(WebSearchUserLocation {
                r#type: WebSearchUserLocationType::Approximate,
                approximate: WebSearchLocation {
                    city: Some("Paris".to_string()),
                    ..Default::default()
                },
            }),
        })
        .build()?;

    let response_message = client
        .chat()
        .create(request)
        .await?
        .choices
        .first()
        .unwrap()
        .message
        .clone();

    if let Some(content) = response_message.content {
        println!("Response: {}", content);
    }

    Ok(())
}
