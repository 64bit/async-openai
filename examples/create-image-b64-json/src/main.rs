use async_openai::{
    types::{CreateImageRequest, ImageSize, ResponseFormat},
    Client, Image,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequest {
        prompt: "Generate a logo for github repository async-openai".to_owned(),
        n: Some(2),
        response_format: Some(ResponseFormat::B64Json),
        size: Some(ImageSize::S256x256),
        user: Some("async-openai".to_owned()),
    };

    let response = Image::create(&client, request).await?;

    // Response already contains image data in base64 format.
    // Save images to ./data directory, each save happens in dedicated Tokio task
    // (creates directory when it doesn't exist)
    response.save("./data").await?;

    Ok(())
}
