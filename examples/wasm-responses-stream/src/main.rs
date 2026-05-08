use async_openai::{
    config::OpenAIConfig,
    traits::EventType,
    types::responses::{CreateResponseArgs, ResponseStreamEvent},
    Client,
};
use dioxus::prelude::*;
use futures::StreamExt;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut user_input = use_signal(|| "Write a haiku about programming.".to_string());
    let mut response_text = use_signal(String::new);
    let mut event_log = use_signal(Vec::<String>::new);
    let mut is_loading = use_signal(|| false);
    let mut api_key = use_signal(|| "abc".to_string());
    let mut client = use_signal(|| None::<Client<OpenAIConfig>>);
    let mut configured_api_key = use_signal(String::new);
    let mut client_status = use_signal(|| "Client is not configured.".to_string());

    let configure_client = move |_| {
        let key = api_key();

        if key.is_empty() || is_loading() {
            return;
        }

        if client().is_some() && configured_api_key() == key {
            client_status.set("Client already configured for this API key.".to_string());
            return;
        }

        let config = OpenAIConfig::new().with_api_key(key);
        client.set(Some(Client::with_config(config)));
        configured_api_key.set(api_key());
        client_status.set("Client configured.".to_string());
        event_log.set(Vec::new());
        response_text.set(String::new());
    };

    let submit = move |_| {
        let input = user_input();
        let configured_client = client();

        if input.is_empty() || configured_client.is_none() || is_loading() {
            return;
        }

        is_loading.set(true);
        response_text.set(String::new());
        event_log.set(Vec::new());

        spawn(async move {
            let client = configured_client.unwrap();

            let request = CreateResponseArgs::default()
                .model("gpt-5-mini")
                .input(input)
                .max_output_tokens(1024u32)
                .stream(true)
                .build()
                .unwrap();

            match client.responses().create_stream(request).await {
                Ok(mut stream) => {
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(event) => {
                                let event_type = event.event_type().to_string();
                                event_log.with_mut(|events| events.push(event_type));

                                if let ResponseStreamEvent::ResponseOutputTextDelta(delta) = event {
                                    response_text.with_mut(|text| text.push_str(&delta.delta));
                                }
                            }
                            Err(error) => {
                                event_log.with_mut(|events| events.push(format!("error: {error}")));
                                break;
                            }
                        }
                    }
                }
                Err(error) => {
                    event_log.with_mut(|events| events.push(format!("error: {error}")));
                }
            }

            is_loading.set(false);
        });
    };

    let event_count = event_log().len();
    let can_stream = client().is_some() && !is_loading();

    rsx! {
        div {
            style: "padding: 20px; max-width: 760px; margin: 0 auto; font-family: sans-serif;",

            h1 { "async-openai WASM streaming demo" }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "API Key: "
                    input {
                        r#type: "password",
                        value: "{api_key}",
                        oninput: move |e| api_key.set(e.value()),
                        placeholder: "sk-...",
                        style: "width: 100%; padding: 8px; margin-top: 5px;"
                    }
                }
                button {
                    onclick: configure_client,
                    disabled: is_loading(),
                    style: "margin-top: 10px; padding: 8px 14px; cursor: pointer;",
                    "Configure client"
                }
                p { style: "margin: 8px 0 0; color: #555;", "{client_status}" }
            }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "Your message: "
                    textarea {
                        value: "{user_input}",
                        oninput: move |e| user_input.set(e.value()),
                        placeholder: "Enter your message here...",
                        style: "width: 100%; padding: 8px; margin-top: 5px; min-height: 100px;"
                    }
                }
            }

            button {
                onclick: submit,
                disabled: !can_stream,
                style: "padding: 10px 20px; cursor: pointer;",
                if is_loading() { "Streaming..." } else { "Stream response" }
            }

            div {
                style: "margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 5px; min-height: 120px;",
                h3 { "Streaming output" }
                p { style: "white-space: pre-wrap;", "{response_text}" }
            }

            if event_count > 0 {
                div {
                    style: "margin-top: 20px; padding: 15px; background: #fafafa; border: 1px solid #ddd; border-radius: 5px;",
                    h3 { "Events ({event_count})" }
                    ul {
                        style: "max-height: 180px; overflow: auto;",
                        for event in event_log() {
                            li { "{event}" }
                        }
                    }
                }
            }
        }
    }
}
