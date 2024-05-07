use std::error::Error;

use async_openai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model_list = client.models().list().await?;

    println!("List of models:\n {:#?}", model_list.data);

    let model = client.models().retrieve("gpt-3.5-turbo-instruct").await?;
    println!("gpt-3.5-turbo-instruct model id: {}", model.id);

    Ok(())
}
