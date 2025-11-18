use async_openai::{
    traits::RequestOptionsBuilder,
    types::containers::{
        ContainerExpiresAfter, ContainerExpiresAfterAnchor, CreateContainerFileRequest,
        CreateContainerRequestArgs,
    },
    types::InputSource,
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the OpenAI client
    let client = Client::new();

    // Create a new container
    println!("Creating a new container...");
    let create_request = CreateContainerRequestArgs::default()
        .name("My Test Container")
        .expires_after(ContainerExpiresAfter {
            anchor: ContainerExpiresAfterAnchor::LastActiveAt,
            minutes: 20,
        })
        .build()?;

    let container = client.containers().create(create_request).await?;
    println!("Created container with ID: {}", container.id);
    println!("Container name: {}", container.name);
    println!("Container status: {}", container.status);

    // List all containers
    println!("\nListing all containers...");
    let query = [("limit", "10")];
    let list_response = client.containers().query(&query)?.list().await?;
    println!("Found {} containers", list_response.data.len());
    for c in &list_response.data {
        println!("  - {} ({})", c.name, c.id);
    }

    // Retrieve the container
    println!("\nRetrieving container...");
    let retrieved = client.containers().retrieve(&container.id).await?;
    println!("Retrieved container: {}", retrieved.name);

    // Create a file in the container using in-memory content
    println!("\nCreating a file in the container...");
    let file_content = b"Hello from the container!";
    let create_file_request = CreateContainerFileRequest {
        file: Some(InputSource::VecU8 {
            filename: "hello.txt".to_string(),
            vec: file_content.to_vec(),
        }),
        file_id: None,
    };

    let container_file = client
        .containers()
        .files(&container.id)
        .create(create_file_request)
        .await?;
    println!("Created file with ID: {}", container_file.id);
    println!("File path: {}", container_file.path);
    println!("File size: {} bytes", container_file.bytes);

    // List files in the container
    println!("\nListing files in the container...");
    let files_query = [("limit", "10")];
    let files_list = client
        .containers()
        .files(&container.id)
        .query(&files_query)?
        .list()
        .await?;
    println!("Found {} files", files_list.data.len());
    for f in &files_list.data {
        println!("  - {} ({} bytes)", f.path, f.bytes);
    }

    // Retrieve the file
    println!("\nRetrieving the file...");
    let retrieved_file = client
        .containers()
        .files(&container.id)
        .retrieve(&container_file.id)
        .await?;
    println!("Retrieved file: {}", retrieved_file.path);

    // Get file content
    println!("\nRetrieving file content...");
    let content = client
        .containers()
        .files(&container.id)
        .content(&container_file.id)
        .await?;
    println!(
        "File content: {}",
        String::from_utf8_lossy(content.as_ref())
    );

    // Delete the file
    println!("\nDeleting the file...");
    let delete_file_response = client
        .containers()
        .files(&container.id)
        .delete(&container_file.id)
        .await?;
    println!("File deleted: {}", delete_file_response.deleted);

    // Delete the container
    println!("\nDeleting the container...");
    let delete_response = client.containers().delete(&container.id).await?;
    println!("Container deleted: {}", delete_response.deleted);

    println!("\nAll operations completed successfully!");
    Ok(())
}
