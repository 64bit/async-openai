use async_openai::{
    types::{CreateImageEditRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateImageEditRequestArgs::default()
        .image("./images/sunlit_lounge.png")
        .mask("./images/mask.png")
        .prompt("A sunlit indoor lounge area with a duck in the pool")
        .n(1)
        .size(ImageSize::S1024x1024)
        .response_format(ResponseFormat::Url)
        .user("async-openai")
        .build()?;

    let response = client.images().create_edit(request).await?;

    response.save("./data").await?;

    Ok(())
}
