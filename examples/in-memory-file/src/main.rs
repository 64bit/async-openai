use async_openai::{types::CreateTranscriptionRequestArgs, Client};
use std::error::Error;
use std::fs;
use async_openai::types::AudioInput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read("./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3")?;
    let bytes = bytes::Bytes::from(file_contents);

    let client = Client::new();
    let filename = "A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3".to_string();
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(AudioInput::from_bytes(filename, bytes))
        .model("whisper-1")
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{}", response.text);

    Ok(())
}
