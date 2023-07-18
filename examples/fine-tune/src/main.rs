use std::error::Error;
use serde_json::{Value, Error as OtherError};
use std::fs::OpenOptions;
use std::io::Write;

use async_openai::{
    types::{CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Sentiment analysis, tweets about the aspects of the current state of 
// airtificial intelegence; prompting techniques and AI dev tools. 
// positive(true) / negative(false)

// View first entry in file to ensure correctness.
async fn print_first_line(path: &str) -> io::Result<()> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let first_line = reader.lines().nth(0).unwrap().unwrap();
    println!("{}", first_line);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Delete previous file records
    let list_prev_files = client.files().list().await.unwrap();

    for files in list_prev_files.data.into_iter() {
        client.files().delete(&files.id).await.unwrap();
    }
    
    let file_path_train = "./data/train.jsonl";

    // print_first_line(file_path_train).await?;

    let train_request = CreateFileRequestArgs::default()
        .file(file_path_train)
        .purpose("fine-tune")
        .build()
        .unwrap();

    let openai_training_file = client.files().create(train_request).await.unwrap();

    // Optional: This Request body field is Optional https://platform.openai.com/docs/api-reference/fine-tunes/create#fine-tunes/create-validation_file
    let file_path_validate = "./data/validate.jsonl";

    // print_first_line(file_path_validate).await?;

    let validate_request = CreateFileRequestArgs::default().file(file_path_validate)
    .purpose("fine-tune")
    .build()
    .unwrap();

    let openai_validation_file = client.files().create(validate_request).await.unwrap();

    let list_files = client.files().list().await.unwrap();

    let mut validation_id = String::new();

    let mut training_id = String::new();

    for file in list_files.data.into_iter() {
        
        if file.filename == "train.jsonl" {
            training_id = file.id
            
        } else {
            validation_id = file.id;
        }
    }

    let fine_tune = CreateFineTuneRequestArgs::default().training_file(training_id).validation_file(validation_id).build().unwrap();

    let job = client.fine_tunes().create(fine_tune).await.unwrap();

    let r = client.fine_tunes().list_events_stream(&job.id).await;
    
    Ok(())
}
