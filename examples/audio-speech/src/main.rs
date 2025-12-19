use async_openai::{
    types::audio::{CreateSpeechRequestArgs, SpeechModel, Voice},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateSpeechRequestArgs::default()
        .input("Today is a wonderful day to build something people love!")
        .voice(Voice::Alloy)
        .model(SpeechModel::Tts1)
        .build()?;

    let response = client.audio().speech().create(request).await?;

    response.save("./data/audio.mp3").await?;

    Ok(())
}
