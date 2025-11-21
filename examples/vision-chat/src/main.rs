use std::error::Error;

use async_openai::{
    types::chat::{
        ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ImageDetail,
        ImageUrl,
    },
    Client,
};

/// https://platform.openai.com/docs/guides/vision - quickstart
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Image Credit: https://unsplash.com/photos/pride-of-lion-on-field-L4-BDd01wmM
    let image_url =
        "https://images.unsplash.com/photo-1554990772-0bea55d510d5?q=80&w=512&auto=format";

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .max_tokens(300_u32)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartText::from("What is this image?").into(),
                ChatCompletionRequestMessageContentPartImage::from(ImageUrl {
                    url: image_url.to_string(),
                    detail: Some(ImageDetail::High),
                })
                .into(),
            ])
            .build()?
            .into()])
        .build()?;

    println!("{}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index,
            choice.message.role,
            choice.message.content.unwrap_or_default()
        );
    }

    Ok(())
}
