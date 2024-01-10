use async_openai::{
    types::{CreateSpeechRequestArgs, SpeechResponseFormat, SpeechModel, Voice},
    Client,
};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use futures::{future, StreamExt};
use bytes::{Buf, Bytes, BytesMut, BufMut};
use bytes::buf::Reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateSpeechRequestArgs::default()
        .input("Today is a wonderful day to stream something people love!")
        .voice(Voice::Alloy)
        .model(SpeechModel::Tts1)
        .response_format(SpeechResponseFormat::Mp3)
        .build()?;

    let mut stream = client.audio().speech_stream(request).await?;

    let mut file = File::create("./speech_stream.mp3")?; 
    
    stream.for_each(|item| {
        match item {
            Ok(response) => {
                let res_size = file.write(&response.bytes);
                match res_size {
                    Ok(size) => println!("Wrote {size} bytes to disk"),
                    Err(err) => println!("Could not write to disk: {err}")
                }
            },
            Err(err) => {
                println!("error: {err}");
            }
        }
        future::ready(())
    }).await; 

    Ok(())
}

