use async_openai as openai;
use openai::{types::CreateCompletionRequest, Client, Completion};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let completion_request = CreateCompletionRequest {
        model: "text-davinci-003".to_owned(),
        prompt: Some("Tell me a joke about universe".to_owned()),
        ..Default::default()
    };

    let completion_response = Completion::create(&client, completion_request).await;

    println!("{:#?}", completion_response);
}
