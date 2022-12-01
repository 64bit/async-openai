use std::error::Error;

use async_openai as openai;
use openai::{
    types::{CreateImageEditRequest, ImageInput, ImageSize, ResponseFormat},
    Client, Image,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateImageEditRequest {
        image: ImageInput::new("./images/sunlit_lounge.png"),
        mask: ImageInput::new("./images/mask.png"),
        prompt: "A sunlit indoor lounge area with a duck in the pool".to_string(),
        n: Some(1),
        size: Some(ImageSize::S1024x1024),
        response_format: Some(ResponseFormat::Url),
        user: Some("async-openai".to_string()),
    };

    let response = Image::create_edit(&client, request).await?;

    response.save("./data").await?;

    Ok(())
}
