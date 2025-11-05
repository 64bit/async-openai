use async_openai::{
    Client,
    types::responses::{
        CreateResponseArgs, EasyInputContent, EasyInputMessage, InputItem, InputParam, MessageType,
        ResponseStreamEvent, Role,
    },
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .stream(true)
        .input(InputParam::Items(vec![InputItem::EasyMessage(
            EasyInputMessage {
                r#type: MessageType::Message,
                role: Role::User,
                content: EasyInputContent::Text("Write a haiku about programming.".to_string()),
            },
        )]))
        .build()?;

    let mut stream = client.responses().create_stream(request).await?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response_event) => match &response_event {
                ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                    print!("{}", delta.delta);
                }
                ResponseStreamEvent::ResponseCompleted(_)
                | ResponseStreamEvent::ResponseIncomplete(_)
                | ResponseStreamEvent::ResponseFailed(_) => {
                    break;
                }
                _ => {
                    println!("{response_event:#?}");
                }
            },
            Err(e) => {
                eprintln!("{e:#?}");
            }
        }
    }

    Ok(())
}
