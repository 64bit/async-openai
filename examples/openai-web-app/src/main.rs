use dioxus::prelude::*;
use futures::stream::StreamExt;
use log::Level;

use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, Role};

const USE_AZURE: bool = false;

const API_BASE: &str = "...";
const API_KEY: &str = "...";
const API_VERSION: &str = "...";
const DEPLOYMENT_ID: &str = "...";

pub fn app(cx: Scope) -> Element {
    const GREETING: &str = "Hello! How are you?";
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-0613")
        .messages([
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(GREETING)
                    .build()
                    .unwrap()
            )
        ])
        .build().unwrap();
    let response_string: &UseRef<String> = use_ref(cx, String::new);
    let fetch_completion_chunks: &Coroutine<()> = use_coroutine(cx, |rx| {
        let response_string = response_string.to_owned();
        async move {
            let mut stream = if USE_AZURE {
                let config = async_openai::config::AzureConfig::new()
                    .with_api_base(API_BASE)
                    .with_api_key(API_KEY)
                    .with_api_version(API_VERSION)
                    .with_deployment_id(DEPLOYMENT_ID);
                let client = async_openai::Client::with_config(config);
                client.chat().create_stream(request).await.unwrap()
            } else {
                let config = async_openai::config::OpenAIConfig::new()
                    .with_api_key(API_KEY);
                let config = if API_BASE != "..." {
                    config.with_api_base(API_BASE)
                } else {
                    config
                };
                let client = async_openai::Client::with_config(config);
                client.chat().create_stream(request).await.unwrap()
            };
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(response) => {
                        if response.choices.is_empty() {
                            // azure openai service returns empty response on first call
                            continue;
                        }
                        response_string.with_mut(|string| {
                            if let Some(content) = response.choices[0].delta.content.as_ref() {
                                string.push_str(content);
                            }
                        })
                    }
                    Err(e) => {
                        log::error!("OpenAI Error: {:?}", e);
                    }
                }
            }
        }
    });

    render! {
        div {
            p {
                if USE_AZURE {
                    "Using Azure OpenAI"
                } else {
                    "Using OpenAI"
                }
            }
            p {
                "User: {GREETING}"
            }
            p {
                "Assistant: {response_string.read()}"
            }
        }
    }
}


fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    dioxus_web::launch(app);
}