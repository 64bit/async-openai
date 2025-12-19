use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::chatkit::{CreateChatSessionRequestArgs, WorkflowParamArgs},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get workflow_id from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <workflow_id>", args[0]);
        std::process::exit(1);
    }
    let workflow_id = &args[1];

    println!("Using workflow_id: {}", workflow_id);

    let config = OpenAIConfig::default()
        .with_header("OpenAI-Beta", "chatkit_beta=v1")
        .unwrap();
    let client = Client::with_config(config);

    // 1. Create a ChatKit session
    println!("\n=== Creating ChatKit Session ===");
    let workflow = WorkflowParamArgs::default()
        .id(workflow_id.clone())
        .build()?;

    let session_request = CreateChatSessionRequestArgs::default()
        .workflow(workflow)
        .user("example_user".to_string())
        .build()?;

    let session = client.chatkit().sessions().create(session_request).await?;
    println!("Created session:");
    println!("  ID: {}", session.id);
    println!("  Status: {:?}", session.status);
    println!("  Expires at: {}", session.expires_at);
    println!("  Client secret: {}", session.client_secret);
    println!("  Workflow ID: {}", session.workflow.id);
    println!("  User: {}", session.user);

    // 2. Cancel the session (cleanup)
    println!("\n=== Cancelling Session ===");
    let cancelled_session = client.chatkit().sessions().cancel(&session.id).await?;
    println!("Cancelled session: {}", cancelled_session.id);
    println!("  Status: {:?}", cancelled_session.status);

    Ok(())
}
