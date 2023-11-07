use async_openai::{
    types::{CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs},
    Client,
};
use std::error::Error;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new();

    let query = [("limit", "10")];

    let assistants = client.assistants().list(&query).await?;
    println!("assistants: {assistants:#?}");

    let create_thread_request = CreateThreadRequestArgs::default().build()?;
    let thread_object = client.threads().create(create_thread_request).await?;

    println!("thread object: {thread_object:#?}");

    let create_message_request = CreateMessageRequestArgs::default().build()?;
    let message_object = client
        .threads()
        .messages(&thread_object.id)
        .create(create_message_request)
        .await?;

    println!("message object: {message_object:#?}");

    let create_run_request = CreateRunRequestArgs::default().build()?;
    let run_object = client
        .threads()
        .runs(&thread_object.id)
        .create(create_run_request)
        .await;

    println!("run object: {run_object:#?}");

    let thread_runs = client
        .threads()
        .runs(&thread_object.id)
        .list(&query)
        .await?;

    println!("runs for thread id {}: {thread_runs:#?}", thread_object.id);

    let thread_messages = client
        .threads()
        .messages(&thread_object.id)
        .list(&query)
        .await?;

    println!("thread messages: {thread_messages:#?}");

    let thread_message_files = client
        .threads()
        .messages(&thread_object.id)
        .files("message_id")
        .list(&query)
        .await;
    println!("thread message file: {thread_message_files:#?}");

    let thread_run_steps = client
        .threads()
        .runs(&thread_object.id)
        .steps("run_id")
        .list(&query)
        .await;

    println!("thread run steps: {thread_run_steps:#?}");

    let deleted_thread_object = client.threads().delete(&thread_object.id).await?;
    println!("deleted thread object: {deleted_thread_object:#?}");

    let assistant_files = client.assistants().files("assistant_id").list(&query).await;

    println!("assistant files: {assistant_files:#?}");

    Ok(())
}
