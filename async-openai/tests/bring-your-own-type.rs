use async_openai::Client;
use serde_json::Value;

#[tokio::test]
async fn test_bring_your_own_type() {
    let client = Client::new();

    
    // Models
    let r: Value = client.models().list_byot();
    let r: Value = client.models().retrieve_byot("");
    let r: Value = client.models().delete_byot(String::new());

}
