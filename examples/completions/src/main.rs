use async_openai::{
    types::{CreateCompletionRequest, Prompt},
    Client, Completion,
};

#[tokio::main]
async fn main() {
    let client = Client::new();

    // single
    let request = CreateCompletionRequest {
        model: "text-ada-001".to_owned(),
        prompt: Some(Prompt::String(
            "Tell me a joke about the universe".to_owned(),
        )),
        max_tokens: Some(40),
        ..Default::default()
    };

    let response = Completion::create(&client, request).await;

    println!("Response (single)\n {:#?}", response);

    // multiple
    let request = CreateCompletionRequest {
        model: "text-ada-001".to_owned(),
        prompt: Some(Prompt::StringArray(vec![
            "How old the human civilization?".to_owned(),
            "How old is the Earth?".to_owned(),
        ])),
        max_tokens: Some(40),
        ..Default::default()
    };

    let response = Completion::create(&client, request).await;

    println!("Response (multiple)\n {:#?}", response);
}
