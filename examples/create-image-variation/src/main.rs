use async_openai::{
    types::{CreateImageVariationRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateImageVariationRequestArgs::default()
        .image("./images/cake.png")
        .n(1)
        .size(ImageSize::S512x512)
        .response_format(ResponseFormat::Url)
        .user("async-openai")
        .build()?;

    let response = client.images().create_variation(request).await?;

    response.save("./data").await?;

    Ok(())
}
