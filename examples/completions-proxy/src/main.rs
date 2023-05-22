use actix_web::web::Json;
use actix_web_lab::sse;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use async_openai::{
    types::{CreateChatCompletionRequest, CreateCompletionRequestArgs},
    Client,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionRequest {
    prompt: String,
    max_tokens: u16,
    temperature: f32,
    stream: bool,
}

#[post("/completions")]
async fn post_completions(payload: Json<CompletionRequest>) -> impl Responder {
    println!("Received completion request: {:?}", payload.0);

    let client = Client::new();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .n(1)
        .prompt(payload.prompt.as_str())
        .stream(true)
        .max_tokens(payload.max_tokens)
        .temperature(payload.temperature)
        .build()
        .unwrap();

    let stream = client.completions().create_stream(request).await.unwrap();

    let response_stream = stream.map(|response| match response {
        Ok(ccr) => {
            println!("Response: {:?}", ccr);
            Ok(sse::Data::new(ccr.choices[0].text.as_str()).into())
        }
        Err(e) => {
            eprintln!("OpenAIError: {:?}", e);
            Err(e)
        }
    });

    sse::Sse::from_stream(response_stream)
}

#[post("/chat/completions")]
async fn post_chat_completions(payload: Json<CreateChatCompletionRequest>) -> impl Responder {
    println!("Received chat completion request: {:?}", payload.0);

    let client = Client::new();

    let request = payload.0;

    let stream = client.chat().create_stream(request).await.unwrap();

    let response_stream = stream.map(|response| match response {
        Ok(cccr) => {
            println!("Response: {:?}", cccr);
            Ok(sse::Data::new(match &cccr.choices[0].delta.content {
                Some(content) => content.as_str(),
                None => "",
            })
            .into())
        }
        Err(e) => {
            eprintln!("OpenAIError: {:?}", e);
            Err(e)
        }
    });

    sse::Sse::from_stream(response_stream)
}

#[tokio::main]
async fn main() {
    let port = 3333;
    let server = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(post_completions)
            .service(post_chat_completions)
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    // .disable_signals()
    .run();
    println!("Server started on port {port}");
    server.await.unwrap();
}
