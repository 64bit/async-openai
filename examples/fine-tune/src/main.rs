use std::error::Error;
//use serde_json::{Value, Error as OtherError};
//use std::fs::OpenOptions;
//use std::io::Write;
use futures::StreamExt;

use async_openai::{
    types::{CreateFileRequestArgs, CreateFineTuneRequestArgs},
    Client,
};

use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
     // This should come from env var outside the program
     std::env::set_var("RUST_LOG", "warn");

     // Setup tracing subscriber so that library can log the rate limited message
     tracing_subscriber::registry()
         .with(fmt::layer())
         .with(EnvFilter::from_default_env())
         .init();
    let client = Client::new();

    // Delete previous file records
    let list_prev_files = client.files().list().await.unwrap();

    for files in list_prev_files.data.into_iter() {
        client.files().delete(&files.id).await.unwrap();
    }
    
    let file_path_train = "./data_files/train.jsonl";

    // print_first_line(file_path_train).await?;

    let train_request = CreateFileRequestArgs::default()
        .file(file_path_train)
        .purpose("fine-tune")
        .build()
        .unwrap();

    let _ = client.files().create(train_request).await.unwrap();

    // Optional: This Request body field is Optional https://platform.openai.com/docs/api-reference/fine-tunes/create#fine-tunes/create-validation_file
    let file_path_validate = "./data_files/validate.jsonl";

    // print_first_line(file_path_validate).await?;

    let validate_request = CreateFileRequestArgs::default().file(file_path_validate)
    .purpose("fine-tune")
    .build()
    .unwrap();

    let _ = client.files().create(validate_request).await.unwrap();

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

    let fine_tune = CreateFineTuneRequestArgs::default().training_file(training_id).validation_file(validation_id).n_epochs(1u32).build().unwrap();

    let job = client.fine_tunes().create(fine_tune).await.unwrap();

    match client.fine_tunes().list_events_stream(&job.id).await {
        Ok(mut stream) => {
            while let Some(item) = stream.next().await {
                match item {
                    Ok(response) => {
                        println!("{:?}", response);

                        let sio = client.fine_tunes().retrieve(&job.id).await.unwrap();
                        // println!{"{:?}",sio }
                        
                        if let Some(name) = sio.fine_tuned_model {
                        
                            println!("{:?}", name);
                        }
                    },
                    Err(error) => continue //println!("-// {:?}", error),
                }
            }
        },
        Err(e) => {
            println!("-/ {:?}", e)
        }
    }
    Ok(())
}
