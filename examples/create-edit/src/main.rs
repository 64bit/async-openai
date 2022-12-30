use async_openai::{types::CreateEditRequestArgs, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateEditRequestArgs::default()
        .model("text-davinci-edit-001")
        .input(concat!(
            "It's surely our responsibility to do everything within our power ",
            "to create a planet that provides a home not just for us, ",
            "but for all life on Earth."
        ))
        .instruction("Add a new paragraph in Sir David Attenborough voice")
        .n(2)
        .temperature(0.9)
        .build()?;

    let response = client.edits().create(request).await?;

    for choice in response.choices {
        println!("{} \n----", choice.text)
    }

    Ok(())
}
