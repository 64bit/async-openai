use async_openai::{Client, Models};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let response = Models::list(&client).await;

    println!("Models::list:\n {:#?}", response);

    let response = Models::retrieve(&client, "text-davinci-003").await;
    println!("Retrieved text-davinci-003: {response:#?}");
}
