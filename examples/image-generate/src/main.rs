use async_openai::{
    types::images::{CreateImageRequestArgs, ImageModel, ImageSize},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .model(ImageModel::GptImage2)
        .prompt("cats on sofa and carpet in living room")
        .n(2)
        .size(ImageSize::Auto)
        .user("async-openai")
        .build()?;

    let response = client.images().generate(request).await?;

    // Concurrently save each image in its own Tokio task.
    // Create directory if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
