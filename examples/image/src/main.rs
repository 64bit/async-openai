use std::error::Error;

use async_openai as openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = openai::Client::new();

    let create_image_request = openai::types::CreateImageRequest {
        prompt: "cats on sofa and carpet in living room".to_string(),
        n: Some(2),
        response_format: Some(openai::types::ResponseFormat::Url),
        size: Some(openai::types::ImageSize::S256x256),
        user: Some("async-openai".to_string()),
    };

    println!(
        "Sending request for prompt: {}",
        create_image_request.prompt
    );
    let create_image_response = openai::Image::create(&client, create_image_request).await?;

    println!("Response: {:#?}", create_image_response);

    println!("Saving images ...");
    create_image_response.save("./data").await?;

    Ok(())
}
