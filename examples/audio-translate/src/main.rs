use async_openai::{types::CreateTranslationRequestArgs, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=bHWmzQ4HTS0
    let request = CreateTranslationRequestArgs::default()
        .file("./audio/koshish karne walon ki haar nahi hoti by amitabh bachchan_320kbps.mp3")
        .model("whisper-1")
        .build()?;

    let response = client.audio().translate(request).await?;

    println!("{}", response.text);

    Ok(())
}
