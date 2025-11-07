use async_openai::{
    types::audio::{CreateTranslationRequestArgs, TranslationResponseFormat},
    Client,
};
use std::error::Error;

async fn translate_srt() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateTranslationRequestArgs::default()
        .file("./audio/koshish karne walon ki haar nahi hoti by amitabh bachchan_320kbps.mp3")
        .model("whisper-1")
        .response_format(TranslationResponseFormat::Srt)
        .build()?;

    let response = client.audio().translation().create_raw(request).await?;

    println!("translate_srt:");
    println!("{}", String::from_utf8_lossy(response.as_ref()));
    Ok(())
}

async fn translate_verbose_json() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // Credits and Source for audio: https://www.youtube.com/watch?v=bHWmzQ4HTS0
    let request = CreateTranslationRequestArgs::default()
        .file("./audio/koshish karne walon ki haar nahi hoti by amitabh bachchan_320kbps.mp3")
        .model("whisper-1")
        .build()?;

    let response = client.audio().translation().create(request).await?;

    println!("translate_verbose_json:");
    println!("{}", response.text);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    translate_verbose_json().await?;
    translate_srt().await?;
    Ok(())
}
