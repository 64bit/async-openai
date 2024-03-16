use async_openai::{
    types::{AudioResponseFormat, CreateTranscriptionRequestArgs, TimestampGranularity},
    Client
};
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
        .response_format(AudioResponseFormat::VerboseJson)
        .timestamp_granularities(vec![TimestampGranularity::Word, TimestampGranularity::Segment])
        .build()?;

    let response = client.audio().transcribe(request).await?;

    println!("{}", response.text);
    if let Some(words) = &response.words {
        println!("- {} words", words.len());
    }
    if let Some(segments) = &response.segments {
        println!("- {} segments", segments.len());
    }

    Ok(())
}
