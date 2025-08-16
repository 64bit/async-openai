use async_openai::{
    Client,
    types::responses::{
        CreateResponseArgs, Input, InputContent, InputItem, InputMessageArgs, ResponseEvent, Role,
    },
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .stream(true)
        .input(Input::Items(vec![InputItem::Message(
            InputMessageArgs::default()
                .role(Role::User)
                .content(InputContent::TextInput(
                    "Write a haiku about programming.".to_string(),
                ))
                .build()?,
        )]))
        .build()?;

    let mut stream = client.responses().create_stream(request).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response_event) => match &response_event {
                ResponseEvent::ResponseOutputTextDelta(delta) => {
                    print!("{}", delta.delta);
                }
                ResponseEvent::ResponseCompleted(_)
                | ResponseEvent::ResponseIncomplete(_)
                | ResponseEvent::ResponseFailed(_) => {
                    break;
                }
                _ => { println!("{response_event:#?}"); }
            },
            Err(e) => {
                eprintln!("{e:#?}");
                // When a stream ends, it returns Err(OpenAIError::StreamError("Stream ended"))
                // Without this, the stream will never end
                break;
            }
        }
    }

    Ok(())
}
