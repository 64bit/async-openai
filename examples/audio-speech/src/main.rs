use async_openai::{
    types::{CreateSpeechRequestArgs, Voice},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateSpeechRequestArgs::default()
        .input("Today is a wonderful day to build something people love!".to_string())
        .voice(Voice::Alloy)
        .model("tts-1")
        .build()?;

    let response = client.audio().speech(request).await?;

    std::fs::write("audio.mp3", response)?;

    Ok(())
}
