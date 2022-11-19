use async_openai as openai;

#[tokio::main]
async fn main() {
    let client = openai::Client::new();

    let completion_request = openai::types::CreateCompletionRequest {
        model: "text-davinci-002".to_string(),
        prompt: Some("What year is this?".to_string()),
        ..Default::default()
    };

    let completion_response = openai::Completion::create(&client, completion_request).await;

    println!("{:#?}", completion_response);
}
