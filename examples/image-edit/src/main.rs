use async_openai::{
    types::images::{CreateImageEditRequestArgs, ImageResponseFormat, ImageSize},
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
        .response_format(ImageResponseFormat::Url)
        .user("async-openai")
        .build()?;

    let response = client.images().edit(request).await?;

    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("image saved at {}", path.display()));

    Ok(())
}
