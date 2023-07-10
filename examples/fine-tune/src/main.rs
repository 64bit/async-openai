use std::error::Error;

use async_openai::{
    types::{CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    let file_path_train = "/tmp/train.jsonl";
    let contents = concat!(
        "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}\n", // \n is to make it valid jsonl
        "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}"
    );
    tokio::fs::write(file_path_train, contents).await.unwrap();

    let train_request = CreateFileRequestArgs::default()
        .file(file_path_train)
        .purpose("fine-tune")
        .build()
        .unwrap();

    let openai_training_file = client.files().create(train_request).await.unwrap();

    // Optional: This Request body field is Optional https://platform.openai.com/docs/api-reference/fine-tunes/create#fine-tunes/create-validation_file
    let file_path_validate = "/tmp/validate.jsonl";
    let contents = concat!(
        "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}\n", // \n is to make it valid jsonl
        "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}"
    );
    tokio::fs::write(file_path_validate, contents).await.unwrap();

    let validate_request = CreateFileRequestArgs::default().file(file_path_validate)
    .purpose("fine-tune")
    .build()
    .unwrap();

    let openai_validation_file = client.files().create(validate_request).await.unwrap();

    Ok(())
}
