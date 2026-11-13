use async_openai::{
    traits::EventType,
    types::realtime::{
        Audio, AudioInput, AudioOutput, RealtimeAudioFormats,
        RealtimeClientEventInputAudioBufferAppend, RealtimeClientEventInputAudioBufferCommit,
        RealtimeClientEventResponseCreate, RealtimeClientEventSessionUpdate,
        RealtimeCreateClientSecretRequest, RealtimeResponseStatus, RealtimeServerEvent,
        RealtimeSession, RealtimeVoice, Session,
    },
};
use base64::{engine::general_purpose::STANDARD, Engine};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::{error::Error, path::Path};
use tokio::{
    net::TcpStream,
    sync::oneshot::{self, Receiver, Sender},
};
use tokio_tungstenite::{
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Use the API key to create a client secret.
    let secret_request = RealtimeCreateClientSecretRequest {
        session: Session::RealtimeSession(Box::default()),
        expires_after: None,
    };
    let secret = async_openai::Client::new()
        .realtime()
        .create_client_secret(secret_request)
        .await?;
    println!("Client secret created");

    // Connect using the client secret.
    let mut request =
        "wss://api.openai.com/v1/realtime?model=gpt-realtime".into_client_request()?;
    request
        .headers_mut()
        .insert("Authorization", format!("Bearer {}", secret.value).parse()?);
    let (ws, _) = tokio_tungstenite::connect_async(request).await?;
    let (writer, reader) = ws.split();
    let (ws_close_tx, ws_close_rx) = oneshot::channel();
    println!("Realtime WebSocket connected");

    tokio::try_join!(
        writer_task(writer, ws_close_rx),
        reader_task(reader, ws_close_tx)
    )?;

    Ok(())
}

/// Configure the session, send audio, and close after the response completes.
async fn writer_task(
    mut writer: WsWriter,
    ws_close_rx: Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    // Update session config; use PCM16 audio with manual turn control.
    let session_update = RealtimeClientEventSessionUpdate {
        event_id: Default::default(),
        session: Session::RealtimeSession(Box::new(RealtimeSession {
            audio: Some(Audio {
                input: AudioInput {
                    format: RealtimeAudioFormats::PCMAudioFormat { rate: 24000 },
                    noise_reduction: Default::default(),
                    transcription: Default::default(),
                    turn_detection: None, // Disable voice activity detection.
                },
                output: AudioOutput {
                    format: RealtimeAudioFormats::PCMAudioFormat { rate: 24000 },
                    speed: 1.0,
                    voice: RealtimeVoice::Marin,
                },
            }),
            ..Default::default()
        })),
    };
    writer.send(session_update.into()).await?;
    println!("writer: sent session.update");

    // Send raw audio from the WAV file.
    let audio_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("audio.wav");
    let audio_base64 = read_wav_base64(audio_path)?;
    let audio_buffer_append = RealtimeClientEventInputAudioBufferAppend {
        audio: audio_base64,
        ..Default::default()
    };
    writer.send(audio_buffer_append.into()).await?;
    println!("writer: sent input_audio_buffer.append");

    // Commit the audio as a user turn.
    writer
        .send(RealtimeClientEventInputAudioBufferCommit::default().into())
        .await?;
    println!("writer: sent input_audio_buffer.commit");

    // Request a response.
    writer
        .send(RealtimeClientEventResponseCreate::default().into())
        .await?;
    println!("writer: sent response.create");

    // Close after the reader confirms completion.
    ws_close_rx
        .await
        .map_err(|_| "Reader exited without confirming response completion")?;
    writer.send(Message::Close(None)).await?;

    Ok(())
}

/// Handle server events, save audio, and signal response completion.
async fn reader_task(mut reader: WsReader, ws_close_tx: Sender<()>) -> Result<(), Box<dyn Error>> {
    let mut audio_buffer = Vec::new();
    while let Some(result) = reader.next().await {
        let message = result?;
        match message {
            Message::Text(text) => {
                let Ok(event) = serde_json::from_str::<RealtimeServerEvent>(&text) else {
                    println!("reader: invalid server event: {text}");
                    continue;
                };

                match event {
                    RealtimeServerEvent::Error(event) => {
                        return Err(format!("Realtime API error: {:?}", event.error).into());
                    }
                    RealtimeServerEvent::ResponseOutputAudioTranscriptDelta(_) => {}
                    RealtimeServerEvent::ResponseOutputAudioTranscriptDone(event) => {
                        println!("reader: transcript: {}", event.transcript)
                    }
                    RealtimeServerEvent::ResponseOutputAudioDelta(event) => {
                        let audio = STANDARD.decode(&event.delta)?;
                        audio_buffer.extend_from_slice(&audio);
                    }
                    RealtimeServerEvent::ResponseOutputAudioDone(_) => {
                        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("response.wav");
                        save_pcm_wav(&path, &audio_buffer)?;
                        audio_buffer.clear();
                        println!("reader: saved {}", path.display());
                    }
                    RealtimeServerEvent::ResponseDone(event) => {
                        if !matches!(event.response.status, RealtimeResponseStatus::Completed) {
                            return Err(format!(
                                "Response did not complete: status={:?}, details={:?}",
                                event.response.status, event.response.status_details
                            )
                                .into());
                        }
                        println!("reader: response completed");
                        ws_close_tx.send(()).map_err(|_| "Writer already exited")?;
                        return Ok(());
                    }
                    other_event => println!("reader: received {}", other_event.event_type()),
                }
            }
            Message::Close(frame) => {
                return Err(
                    format!("WebSocket closed before response completion: {frame:?}").into(),
                );
            }
            _ => {}
        }
    }

    Err("WebSocket stream ended before response completion".into())
}

/// Save 24 kHz mono PCM16 little-endian audio as WAV.
fn save_pcm_wav(path: impl AsRef<Path>, audio: &[u8]) -> Result<(), hound::Error> {
    if audio.len() % 2 != 0 {
        return Err(hound::Error::FormatError(
            "PCM16 requires an even byte count",
        ));
    }
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 24_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for bytes in audio.chunks_exact(2) {
        writer.write_sample(i16::from_le_bytes([bytes[0], bytes[1]]))?;
    }
    writer.finalize()
}

/// Encode raw audio from a 24 kHz mono PCM16 WAV file as Base64.
fn read_wav_base64(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();

    if spec.channels != 1
        || spec.sample_rate != 24_000
        || spec.bits_per_sample != 16
        || spec.sample_format != hound::SampleFormat::Int
    {
        return Err("Expected a 24 kHz mono PCM16 WAV file".into());
    }

    let mut pcm = Vec::new();
    for sample in reader.samples::<i16>() {
        pcm.extend_from_slice(&sample?.to_le_bytes());
    }

    Ok(STANDARD.encode(&pcm))
}
