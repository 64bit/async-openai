use serde::{Deserialize, Serialize};
use worker::*;
use async_openai::{
    types::{
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_openai::config::OpenAIConfig;

const AUTH: &str = "dsasakjhj-odfhbodfhuery21432p";

const OPENAI_API_KEY: &str = "...";
const OPENAI_API_BASE: &str = "...";

const README: &str = include_str!("../README.md");

#[derive(Deserialize, Serialize)]
struct Message {
    content: String,
}

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |_req, _ctx| async move {
            Response::ok(README)
        })
        .get_async("/readme", |_req, _ctx| async move {
            Response::ok(README)
        })
        .get_async("/help", |_req, _ctx| async move {
            Response::ok(README)
        })
        // handle files and fields from multipart/form-data requests
        .post_async("/chat", |mut req, _ctx| async move {
            // check for auth
            if !req.headers()
                .get("x-api-key")
                .is_ok_and(|k|
                    k.is_some_and(|k| k == AUTH)) {
                return Response::error("Unauthorized", 401);
            }
            let message = req.json::<Message>().await?;
            assert_ne!(OPENAI_API_KEY, "...", "Please set OPENAI_API_KEY");
            let mut config = OpenAIConfig::new().with_api_key(OPENAI_API_KEY);
            if OPENAI_API_BASE != "..." {
                config = config.with_api_base(OPENAI_API_BASE);
            }
            let client = Client::with_config(config);
            // make request to OpenAI
            let request = CreateChatCompletionRequestArgs::default()
                .max_tokens(512u16)
                .model("gpt-3.5-turbo")
                .messages([
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(message.content)
                        .build()
                        .unwrap()
                        .into(),
                ])
                .build()
                .unwrap();

            let response = client.chat().create(request).await.unwrap();
            let message = response
                .choices
                .first().unwrap()
                .message
                .content.as_ref().unwrap();
            Response::from_json(&Message { content: message.clone() })
        })
        .run(req, env).await
}
