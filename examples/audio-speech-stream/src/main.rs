use async_openai::traits::EventType;
use async_openai::{
    types::audio::{
        CreateSpeechRequestArgs, CreateSpeechResponseStreamEvent, SpeechModel, StreamFormat, Voice,
    },
    Client,
};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use futures::StreamExt;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateSpeechRequestArgs::default()
        .input("Today is a wonderful day to build something people love!")
        .voice(Voice::Alloy)
        .model(SpeechModel::Gpt4oMiniTts)
        .stream_format(StreamFormat::SSE)
        .build()?;

    let mut response = client.audio().speech().create_stream(request).await?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./data/sse-audio.mp3")?;

    while let Some(event) = response.next().await {
        match event {
            Ok(event) => match event {
                CreateSpeechResponseStreamEvent::SpeechAudioDelta(delta) => {
                    let decoded = BASE64_STANDARD.decode(&delta.audio)?;
                    println!(
                        "[{}] audio base64-decoded size: {:?}",
                        delta.event_type(),
                        decoded.len()
                    );
                    file.write_all(&decoded)?;
                }
                CreateSpeechResponseStreamEvent::SpeechAudioDone(done) => {
                    println!("[{}] usage: {:?}", done.event_type(), done.usage);
                }
            },
            Err(e) => eprintln!("{e:?}"),
        }
    }

    Ok(())
}
