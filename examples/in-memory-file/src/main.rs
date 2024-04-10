use async_openai::types::AudioInput;
use async_openai::{types::CreateTranscriptionRequestArgs, Client};
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filename =
        "A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3".to_string();
    let file_contents = fs::read(format!("./audio/{}", filename))?;

    let bytes = bytes::Bytes::from(file_contents);

    // To pass in in-memory files, you can pass either bytes::Bytes or vec[u8] to AudioInputs, FileInputs, and ImageInputs.
    let audio_input = AudioInput::from_bytes(filename, bytes);

    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(audio_input)
        .model("whisper-1")
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{}", response.text);

    Ok(())
}
