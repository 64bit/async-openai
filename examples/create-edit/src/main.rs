use async_openai::{types::CreateEditRequest, Client, Edit};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateEditRequest {
        model: "text-davinci-edit-001".to_string(),
        input: Some(
            "It's surely our responsibility to do everything within our power
            to create a planet that provides a home not just for us,
            but for all life on Earth."
                .into(),
        ),
        instruction: "Add a new paragraph in Sir David Attenborough voice".to_string(),
        n: Some(2),
        temperature: Some(0.9), // recommended altering this or top_p but not both.
        top_p: None,
    };

    let response = Edit::create(&client, request).await?;

    println!("response: {:#?}", response);

    Ok(())
}
