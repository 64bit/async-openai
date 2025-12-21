use async_openai::{config::OpenAIConfig, types::responses::CreateResponseArgs, Client};
use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut user_input = use_signal(|| "what is 10+10?".to_string());
    let mut response_text = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut api_key = use_signal(|| "abc".to_string());

    let submit = move |_| {
        let input = user_input();
        let key = api_key();

        if input.is_empty() || key.is_empty() {
            return;
        }

        is_loading.set(true);
        response_text.set(String::new());

        spawn(async move {
            let config = OpenAIConfig::new().with_api_key(key);
            let client = Client::with_config(config);

            let request = CreateResponseArgs::default()
                .model("gpt-5-mini")
                .input(input)
                .max_output_tokens(1024u32)
                .build()
                .unwrap();

            match client.responses().create(request).await {
                Ok(response) => {
                    response_text.set(format!("{:#?}", response));
                }
                Err(e) => {
                    response_text.set(format!("Error: {}", e));
                }
            }

            is_loading.set(false);
        });
    };

    rsx! {
        div {
            style: "padding: 20px; max-width: 600px; margin: 0 auto; font-family: sans-serif;",

            h1 { "async-openai WASM demo" }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "API Key: ",
                    input {
                        r#type: "password",
                        value: "{api_key}",
                        oninput: move |e| api_key.set(e.value()),
                        placeholder: "sk-...",
                        style: "width: 100%; padding: 8px; margin-top: 5px;"
                    }
                }
            }

            div {
                style: "margin-bottom: 20px;",
                label {
                    "Your message: ",
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
                disabled: is_loading(),
                style: "padding: 10px 20px; cursor: pointer;",
                if is_loading() { "Loading..." } else { "Send" }
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
