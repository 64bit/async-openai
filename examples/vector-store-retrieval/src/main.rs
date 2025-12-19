use std::error::Error;

use async_openai::{
    types::{
        files::{CreateFileRequest, FilePurpose},
        vectorstores::{CreateVectorStoreRequest, VectorStoreSearchRequest, VectorStoreStatus},
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    //
    // Step 1: Upload files and add them to a Vector Store
    //

    // upload files to add to vector store
    let uber_file = client
        .files()
        .create(CreateFileRequest {
            file: "./input/uber-10k.pdf".into(),
            purpose: FilePurpose::Assistants,
            expires_after: None,
        })
        .await?;

    let lyft_file = client
        .files()
        .create(CreateFileRequest {
            file: "./input/lyft-10k.pdf".into(),
            purpose: FilePurpose::Assistants,
            expires_after: None,
        })
        .await?;

    // Create a vector store called "Financial Statements"
    // add uploaded file to vector store
    let mut vector_store = client
        .vector_stores()
        .create(CreateVectorStoreRequest {
            name: Some("Financial Statements".into()),
            file_ids: Some(vec![uber_file.id.clone(), lyft_file.id.clone()]),
            ..Default::default()
        })
        .await?;

    //
    // Step 4: Wait for the vector store to be ready
    //
    while vector_store.status != VectorStoreStatus::Completed {
        println!("Waiting for vector store to be ready...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        vector_store = client.vector_stores().retrieve(&vector_store.id).await?;
    }

    //
    // Step 5: Search the vector store
    //
    let results = client
        .vector_stores()
        .search(
            &vector_store.id,
            VectorStoreSearchRequest {
                query: "uber profit".into(),
                ..Default::default()
            },
        )
        .await?;

    // Print the search results
    println!("Search results: {:#?}", results);
    // Cleanup to avoid costs
    let _ = client.vector_stores().delete(&vector_store.id).await?;

    let _ = client.files().delete(&uber_file.id).await?;

    let _ = client.files().delete(&lyft_file.id).await?;
    Ok(())
}
