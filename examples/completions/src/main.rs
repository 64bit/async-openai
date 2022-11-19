#[tokio::main]
async fn main() {
    let client = async_openai::client::Client::new();

    let completion_request = async_openai::types::CreateCompletionRequest {
        model: "text-davinci-002".to_string(),
        prompt: Some("What year is this?".to_string()),
        ..Default::default()
    };

    let completion_response =
        async_openai::completion::Completion::create(&client, completion_request).await;

    println!("{:#?}", completion_response);
}
