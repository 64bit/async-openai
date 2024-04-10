use async_openai::{
    types::{AudioResponseFormat, CreateTranscriptionRequestArgs, TimestampGranularity},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    transcribe_json().await?;
    transcribe_verbose_json().await?;
    transcribe_srt().await?;
    Ok(())
}

async fn transcribe_json() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=oQnDVqGIv4s
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .response_format(AudioResponseFormat::Json)
        .build()?;

    let response = client.audio().transcribe(request).await?;
    println!("{}", response.text);
    Ok(())
}

async fn transcribe_verbose_json() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .response_format(AudioResponseFormat::VerboseJson)
        .timestamp_granularities(vec![
            TimestampGranularity::Word,
            TimestampGranularity::Segment,
        ])
        .build()?;

    let response = client.audio().transcribe_verbose_json(request).await?;

    println!("{}", response.text);
    if let Some(words) = &response.words {
        println!("- {} words", words.len());
    }
    if let Some(segments) = &response.segments {
        println!("- {} segments", segments.len());
    }

    Ok(())
}

async fn transcribe_srt() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateTranscriptionRequestArgs::default()
        .file(
            "./audio/A Message From Sir David Attenborough A Perfect Planet BBC Earth_320kbps.mp3",
        )
        .model("whisper-1")
        .response_format(AudioResponseFormat::Srt)
        .build()?;

    let response = client.audio().transcribe_raw(request).await?;
    println!("{}", String::from_utf8_lossy(response.as_ref()));
    Ok(())
}
