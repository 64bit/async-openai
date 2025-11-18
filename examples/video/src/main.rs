use async_openai::{
    config::OpenAIConfig,
    traits::RequestOptionsBuilder,
    types::videos::{CreateVideoRequestArgs, VideoJob, VideoSize, VideoVariant},
    Client,
};
use bytes::Bytes;
use std::error::Error;

pub async fn save<P: AsRef<std::path::Path>>(
    bytes: Bytes,
    file_path: P,
) -> Result<(), Box<dyn Error>> {
    let dir = file_path.as_ref().parent();
    if let Some(dir) = dir {
        tokio::fs::create_dir_all(dir).await?;
    }

    tokio::fs::write(file_path, &bytes).await?;

    Ok(())
}

async fn create_video(client: &Client<OpenAIConfig>) -> Result<VideoJob, Box<dyn Error>> {
    let request = CreateVideoRequestArgs::default()
        .model("sora-2")
        .prompt("Fridge opens, cat walks out, and celebrates a birthday party")
        .input_reference("./input/monster_original_720p.jpeg")
        .size(VideoSize::S1280x720) // size of input image
        .build()?;

    println!("Generating video...");
    let response = client.videos().create(request).await?;

    println!("Video generation started!");
    println!("Video ID: {}", response.id);
    println!("Status: {}", response.status);
    println!("Model: {}", response.model);
    println!("Size: {}", response.size);
    println!("Duration: {} seconds", response.seconds);

    // Poll for completion
    let video_id = &response.id;
    loop {
        let status_response = client.videos().retrieve(video_id).await?;
        println!("Current status: {}", status_response.status);

        match status_response.status.as_str() {
            "completed" => {
                println!("Video generation completed!");
                break;
            }
            "failed" => {
                println!("Video generation failed!");
                if let Some(error) = status_response.error {
                    println!("Error: {:?}", error);
                }
                return Err("Video generation failed".into());
            }
            _ => {
                println!("Progress: {}%", status_response.progress);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let video = create_video(&client).await?;
    // wait for above video to be "completed"
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let videos = client.videos().query(&[("limit", "100")])?.list().await?;

    for video in &videos.data {
        println!("Video: {:#?}", video);

        if video.status == "completed" {
            let content = client
                .videos()
                .query(&[("variant", VideoVariant::Video)])?
                .download_content(&video.id)
                .await;
            if let Ok(content) = content {
                let output_path = &format!("./data/{}.mp4", video.id);
                save(content, output_path).await?;
                println!("Video saved to {}", output_path);
            } else {
                println!("cannot download video: {:?}", content);
            }
        }
    }

    println!(
        "\nVideo deleted: {:?}",
        client.videos().delete(&video.id).await?
    );

    Ok(())
}
