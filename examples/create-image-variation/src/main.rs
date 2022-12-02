use std::error::Error;

use async_openai as openai;
use openai::{
    types::{CreateImageVariationRequest, ImageInput, ImageSize, ResponseFormat},
    Client, Image,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateImageVariationRequest {
        image: ImageInput::new("./images/cake.png"),
        n: Some(1),
        size: Some(ImageSize::S512x512),
        response_format: Some(ResponseFormat::Url),
        user: Some("async-openai".to_string()),
    };

    let response = Image::create_variation(&client, request).await?;

    response.save("./data").await?;

    Ok(())
}
