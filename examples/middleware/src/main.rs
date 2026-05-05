use std::error::Error;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::middleware::HttpRequestFactory;
use async_openai::retry::OpenAIRetryLayer;
use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use futures::StreamExt;
use http::Response as HttpResponse;
use tower::{service_fn, util::BoxCloneSyncService, ServiceBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let service = build_service(request_count.clone());

    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base("http://example.test")
            .with_api_key("test-key"),
    )
    .with_http_service(service);

    let models = client.models().list().await?;
    println!("models via middleware: {}", models.data[0].id);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("Say hello through middleware")
            .build()?
            .into()])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;
    print!("stream via middleware:");
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        for choice in chunk.choices {
            if let Some(content) = choice.delta.content {
                print!(" {content}");
            }
        }
    }
    println!();

    println!(
        "requests observed by middleware: {}",
        request_count.load(Ordering::SeqCst)
    );

    Ok(())
}

fn build_service(
    request_count: Arc<AtomicUsize>,
) -> BoxCloneSyncService<HttpRequestFactory, reqwest::Response, OpenAIError> {
    // The tower stack is ordered intentionally:
    // - `concurrency_limit` is outermost so it governs the total number of
    //   in-flight requests seen by the example.
    // - `OpenAIRetryLayer` sits inside the limit so each retry attempt still goes through
    //   the same request factory and can rebuild the request safely.
    // - the leaf service actually builds the `reqwest::Request` and returns a
    //   `reqwest::Response`.
    //
    // This mirrors how users are expected to compose policies in their own
    // applications.
    let service = ServiceBuilder::new()
        .concurrency_limit(1)
        .layer(OpenAIRetryLayer::default())
        .service(service_fn(move |factory: HttpRequestFactory| {
            let request_count = request_count.clone();
            async move {
                // Rebuild the request on every attempt. This is what makes the
                // retry policy safe for requests whose body cannot be cloned
                // after construction.
                let request = factory.build().await?;
                let attempt = request_count.fetch_add(1, Ordering::SeqCst);

                let response = match request.url().path() {
                    "/models" if attempt == 0 => HttpResponse::builder()
                        .status(429)
                        .header("content-type", "application/json")
                        .body(reqwest::Body::from(
                            r#"{"error":{"message":"rate limited once for demo","type":"rate_limit_error","param":null,"code":null}}"#,
                        ))
                        .unwrap(),
                    "/models" => HttpResponse::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(reqwest::Body::from(
                            r#"{"object":"list","data":[{"id":"gpt-4o-mini","object":"model","created":0,"owned_by":"openai"}]}"#,
                        ))
                        .unwrap(),
                    "/chat/completions" => HttpResponse::builder()
                        .status(200)
                        .header("content-type", "text/event-stream")
                        .body(reqwest::Body::from(
                            // SSE is just bytes on the wire. The client keeps
                            // the parser local; middleware only controls the
                            // request open/retry phase.
                            concat!(
                                "data: {\"id\":\"chatcmpl-demo\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"hello\"},\"finish_reason\":null}],\"created\":0,\"model\":\"gpt-4o-mini\",\"object\":\"chat.completion.chunk\"}\n\n",
                                "data: {\"id\":\"chatcmpl-demo\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"world\"},\"finish_reason\":\"stop\"}],\"created\":0,\"model\":\"gpt-4o-mini\",\"object\":\"chat.completion.chunk\"}\n\n",
                                "data: [DONE]\n\n"
                            ),
                        ))
                        .unwrap(),
                    path => HttpResponse::builder()
                        .status(404)
                        .header("content-type", "application/json")
                        .body(reqwest::Body::from(format!(
                            r#"{{"error":{{"message":"unhandled path: {path}","type":"invalid_request_error","param":null,"code":null}}}}"#,
                        )))
                        .unwrap(),
                };

                Ok::<reqwest::Response, OpenAIError>(response.into())
            }
        }));

    BoxCloneSyncService::new(service)
}
