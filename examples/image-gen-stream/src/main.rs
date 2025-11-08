use async_openai::{
    traits::EventType,
    types::images::{
        CreateImageRequestArgs, ImageGenStreamEvent, ImageModel, ImageOutputFormat, ImageSize,
    },
    Client,
};
use base64::{engine::general_purpose, Engine as _};
use futures::StreamExt;
use std::error::Error;
use std::fs;
use std::path::Path;

fn save_image(event: ImageGenStreamEvent) -> Result<(), Box<dyn Error>> {
    // Create data directory if it doesn't exist
    let data_dir = Path::new("./data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }

    // Extract b64_json and output_format from the event
    let (b64_json, output_format, created_at, filename_suffix) = match event {
        ImageGenStreamEvent::PartialImage(event) => (
            event.b64_json,
            event.output_format,
            event.created_at,
            format!("partial_{}", event.partial_image_index),
        ),
        ImageGenStreamEvent::Completed(event) => (
            event.b64_json,
            event.output_format,
            event.created_at,
            "completed".to_string(),
        ),
    };

    // Determine file extension from output_format
    let extension = match output_format {
        ImageOutputFormat::Png => "png",
        ImageOutputFormat::Jpeg => "jpeg",
        ImageOutputFormat::Webp => "webp",
    };

    // Create unique filename
    let filename = format!("image_{}_{}.{}", created_at, filename_suffix, extension);
    let filepath = data_dir.join(&filename);

    // Decode base64
    let image_data = general_purpose::STANDARD.decode(b64_json)?;

    // Write to file (create or overwrite)
    fs::write(&filepath, image_data)?;

    println!("Saved image to: {}", filepath.display());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .model(ImageModel::GptImage1)
        .prompt("humans dancing a victory dance")
        .size(ImageSize::S1024x1024)
        .partial_images(2)
        .stream(true)
        .build()?;

    let mut stream = client.images().generate_stream(request).await?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                println!("Saving image from event: {:?}", event.event_type());
                save_image(event)?;
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(())
}
