use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    middleware::{retry::SimpleRetryPolicy, HttpRequestFactory, ReqwestService},
    types::responses::CreateResponseArgs,
    Client,
};
use dioxus::prelude::*;
use tower::{Layer, Service, ServiceBuilder};

type LogFuture = Pin<Box<dyn Future<Output = Result<reqwest::Response, OpenAIError>> + 'static>>;

#[derive(Clone, Debug)]
struct LogLayer;

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService { inner }
    }
}

#[derive(Clone, Debug)]
struct LogService<S> {
    inner: S,
}

impl<S> Service<HttpRequestFactory> for LogService<S>
where
    S: Service<HttpRequestFactory, Response = reqwest::Response, Error = OpenAIError>
        + Clone
        + 'static,
    S::Future: 'static,
{
    type Response = reqwest::Response;
    type Error = OpenAIError;
    type Future = LogFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: HttpRequestFactory) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move {
            web_sys::console::log_1(&"tower-wasm middleware: sending request".into());

            let result = inner.call(request).await;

            web_sys::console::log_1(&"tower-wasm middleware: received response".into());

            result
        })
    }
}

fn build_client(api_key: String) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new().with_api_key(api_key);
    let service = ServiceBuilder::new()
        .layer(LogLayer)
        .retry(SimpleRetryPolicy::default())
        .service(ReqwestService::default());

    Client::with_config(config).with_http_service(service)
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut user_input = use_signal(|| "tell me two dad jokes".to_string());
    let mut response_text = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut api_key = use_signal(String::new);
    let mut client = use_signal(|| None::<Client<OpenAIConfig>>);

    let configure_client = move |_| {
        let key = api_key();

        if key.is_empty() {
            response_text.set("Enter an API key before configuring the client.".to_string());
            return;
        }

        client.set(Some(build_client(key)));
        response_text
            .set("Client configured. Requests will reuse the same tower service.".to_string());
    };

    let submit = move |_| {
        let input = user_input();
        let configured_client = client();

        if input.is_empty() {
            return;
        }

        let Some(client) = configured_client else {
            response_text.set("Configure the client before sending a request.".to_string());
            return;
        };

        is_loading.set(true);
        response_text.set(String::new());

        spawn(async move {
            let request = CreateResponseArgs::default()
                .model("gpt-5-mini")
                .input(input)
                .max_output_tokens(1024u32)
                .build()
                .unwrap();

            match client.responses().create(request).await {
                Ok(response) => {
                    response_text.set(format!("{:#?}", response.output_text()));
                }
                Err(error) => {
                    response_text.set(format!("Error: {error}"));
                }
            }

            is_loading.set(false);
        });
    };

    rsx! {
        div {
            style: "padding: 20px; max-width: 700px; margin: 0 auto; font-family: sans-serif;",

            h1 { "async-openai Tower WASM demo" }
            p {
                "This example routes requests through tower retry middleware before reqwest's wasm transport."
            }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "API Key: "
                    input {
                        r#type: "password",
                        value: "{api_key}",
                        oninput: move |event| api_key.set(event.value()),
                        placeholder: "sk-...",
                        style: "width: 100%; padding: 8px; margin-top: 5px;"
                    }
                }
            }

            button {
                onclick: configure_client,
                disabled: is_loading(),
                style: "padding: 10px 20px; cursor: pointer; margin-bottom: 20px;",
                "Configure reusable client"
            }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "Your message: "
                    textarea {
                        value: "{user_input}",
                        oninput: move |event| user_input.set(event.value()),
                        placeholder: "Enter your message here...",
                        style: "width: 100%; padding: 8px; margin-top: 5px; min-height: 100px;"
                    }
                }
            }

            button {
                onclick: submit,
                disabled: is_loading() || client().is_none(),
                style: "padding: 10px 20px; cursor: pointer;",
                if is_loading() { "Loading..." } else { "Send through tower" }
            }

            if !response_text().is_empty() {
                div {
                    style: "margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 5px;",
                    h3 { "Response:" }
                    p { style: "white-space: pre-wrap;", "{response_text}" }
                }
            }
        }
    }
}
