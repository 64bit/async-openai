use actix_web::{web, App, HttpServer};
use async_openai::{config::OpenAIConfig, types::RequestForStream, Client};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct Request {
    param: String,
    stream: Option<bool>,
}

impl RequestForStream for Request {
    fn is_request_for_stream(&self) -> bool {
        self.stream.unwrap_or(false)
    }

    fn set_request_for_stream(&mut self, stream: bool) {
        self.stream = Some(stream)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Response {
    len: usize,
}

#[tokio::test]
async fn extensible() {
    async fn handle(request: web::Json<Request>) -> web::Json<Response> {
        web::Json(Response {
            len: request.param.len(),
        })
    }

    let server = HttpServer::new(move || {
        App::new().configure(|cfg| {
            cfg.service(web::resource("/chat/completions").route(web::post().to(handle)));
        })
    })
    .disable_signals()
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();

    tokio::spawn(server);

    let client = Client::with_config(OpenAIConfig::new().with_api_base("http://127.0.0.1:8080"));

    let mut request = Request {
        param: "foo".to_string(),
        stream: None,
    };

    let response: Response = client.chat().create_ext(request.clone()).await.unwrap();
    assert_eq!(response, Response { len: 3 });

    request.stream = Some(true);
    let response = client
        .chat()
        .create_stream_ext::<_, Response>(request)
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(
        response.to_string(),
        "stream failed: Invalid header value: \"application/json\""
    )
}
