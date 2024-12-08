use std::{error::Error, io::Write};

use std::fs::File;
use std::io::Read;
use base64;

use async_openai::{
    config::OpenAIConfig,
    types::{
        Audio,
        AudioArgs,
        AudioFormat,
        AudioParams,
        ChatCompletionRequestAssistantMessageArgs,
        ChatCompletionRequestSystemMessageContentPart,
        ChatCompletionRequestMessageContentPartAudioArgs,
        ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestMessageContentPartTextArgs,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessageArgs,
        ChatCompletionRequestUserMessageContent,
        ChatCompletionRequestUserMessageContentPart,
        CreateChatCompletionRequestArgs,
        Modalitie,
        VoiceType
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let file_path = "./audio/test.mp3";

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let audio_data = base64::encode(&buffer);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-audio-preview")

        .modalities(vec![Modalitie::Text, Modalitie::Audio])
        .audio(AudioArgs::default().voice(VoiceType::Alloy).format(AudioFormat::Mp3).build()?)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(ChatCompletionRequestSystemMessageContent::Array(vec![
                    ChatCompletionRequestSystemMessageContentPart::Text(
                        ChatCompletionRequestMessageContentPartTextArgs::default().text(
                            "You are a helpful assistant.".to_string(),
                        ).build()?
                    ),
                ]))
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestUserMessageContentPart::Text(
                        ChatCompletionRequestMessageContentPartTextArgs::default()
                            .text("You are a helpful assistant.".to_string()).build()?
                    ),
                    ChatCompletionRequestUserMessageContentPart::InputAudio(
                        ChatCompletionRequestMessageContentPartAudioArgs::default()
                            .input_audio(
                                Audio::default()
                                    .data(audio_data.clone())
                                    .format(AudioFormat::Mp3)
                                    .build()?
                            )
                            .build()?
                    )
                ]))
                .build()?
                .into(),
        ])
        .build()?;


    let response = client.chat().create(request).await?;

    let audio_data = response.choices[0].message.audio.as_ref().unwrap().data.clone();

    let audio_data = base64::decode(audio_data)?;
    let file_path = "./audio/test_response.mp3";
    let mut file = File::create(file_path)?;
    file.write_all(&audio_data)?;
    Ok(())
}
