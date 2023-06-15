use async_openai::{types::CreateTranscriptionRequestArgs, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{}", response.text);

    Ok(())
}
