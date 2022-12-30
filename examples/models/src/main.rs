use std::error::Error;

use async_openai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model_list = client.models().list().await?;

    println!("List of models:\n {:#?}", model_list.data);

    let model = client.models().retrieve("text-davinci-003").await?;
    println!("text-davinci-003 model id: {}", model.id);

    Ok(())
}
