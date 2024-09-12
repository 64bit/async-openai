use async_openai::{
    types::{CreateImageVariationRequestArgs, DallE2ImageSize, ImageResponseFormat},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateImageVariationRequestArgs::default()
        .image("./images/cake.png")
        .n(1)
        .size(DallE2ImageSize::S512x512)
        .response_format(ImageResponseFormat::Url)
        .user("async-openai")
        .build()?;

    let response = client.images().create_variation(request).await?;

    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("image saved at {}", path.display()));

    Ok(())
}
