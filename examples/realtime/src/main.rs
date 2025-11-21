use std::process::exit;

use async_openai::types::realtime::{
    RealtimeClientEventConversationItemCreate, RealtimeClientEventResponseCreate,
    RealtimeConversationItem, RealtimeServerEvent,
};
use futures_util::{future, pin_mut, StreamExt};

use async_openai::traits::EventType;
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, protocol::Message},
};

#[tokio::main]
async fn main() {
    let url = "wss://api.openai.com/v1/realtime?model=gpt-realtime";
    let api_key = std::env::var("OPENAI_API_KEY").expect("Please provide OPENAPI_API_KEY env var");

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    // create request from url and add required headers
    let mut request = url.into_client_request().unwrap();
    request.headers_mut().insert(
        "Authorization",
        format!("Bearer {api_key}").parse().unwrap(),
    );

    // connect to WebSocket endpoint
    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");

    // output everything to stderr, for rest of the program stdin is used to send items of type "input_text"
    eprintln!("WebSocket handshake complete");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    let ws_to_stdout = {
        read.for_each(|message| async {
            let message = message.unwrap();

            match message {
                Message::Text(_) => {
                    let data = message.clone().into_data();
                    let server_event: Result<RealtimeServerEvent, serde_json::Error> =
                        serde_json::from_slice(&data);
                    match server_event {
                        Ok(server_event) => {
                            eprint!("{:32} | ", server_event.event_type());
                            match server_event {
                                RealtimeServerEvent::ResponseOutputItemDone(event) => {
                                    eprint!("{event:?}");
                                }
                                RealtimeServerEvent::Error(e) => {
                                    eprint!("{e:?}");
                                }
                                _ => {}
                            }
                        }
                        Err(error) => {
                            eprintln!("failed to deserialize: {error:?}");
                            eprintln!("{message:?}");
                        }
                    }
                }
                Message::Binary(_) => eprintln!("Binary"),
                Message::Frame(_) => eprintln!("Frame"),
                Message::Ping(_) => eprintln!("Ping"),
                Message::Pong(_) => eprintln!("Pong"),
                Message::Close(_) => {
                    eprintln!("Close");
                    exit(0);
                }
            }

            // after every message add newline
            eprint!("\n");
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Read from stdin and send "conversation.item.create" and "response.create" client events.
// type "quit" to stop
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);

        let text = String::from_utf8_lossy(&buf).into_owned();

        if text.trim() == "quit" {
            tx.close_channel();
            return;
        }

        // Create item from json representation
        let item = RealtimeConversationItem::try_from(serde_json::json!({
            "type": "message",
            "role": "user",
            "content": [
                {
                    "type": "input_text",
                    "text": String::from_utf8_lossy(&buf).into_owned()
                }
            ]
        }))
        .unwrap();

        // Create event of type "conversation.item.create"
        let event: RealtimeClientEventConversationItemCreate = item.into();
        // Create WebSocket message from client event
        let message: Message = event.into();
        // send WebSocket message containing event of type "conversation.item.create" to server
        tx.unbounded_send(message).unwrap();
        // send WebSocket message containing event of type "response.create" to server
        tx.unbounded_send(RealtimeClientEventResponseCreate::default().into())
            .unwrap();
    }
}
