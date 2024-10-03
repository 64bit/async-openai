use std::error::Error;

use async_openai::{
    types::{
        AssistantToolCodeInterpreterResources, AssistantTools, CreateAssistantRequestArgs,
        CreateFileRequest, CreateMessageRequestArgs, CreateRunRequest, CreateThreadRequest,
        FilePurpose, MessageContent, MessageContentTextAnnotations, MessageRole, RunStatus,
    },
    Client,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "ERROR");

    // Setup tracing subscriber so that library can log the errors
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new();

    // Upload data file with "assistants" purpose
    let data_file = client
        .files()
        .create(CreateFileRequest {
            file: "./input/CASTHPI.csv".into(),
            purpose: FilePurpose::Assistants,
        })
        .await?;

    // Create an assistant with code_interpreter tool with the uploaded file
    let create_assistant_request = CreateAssistantRequestArgs::default()
    .instructions("You are a data processor. When asked a question about data in a file, write and run code to answer the question.")
    .model("gpt-4o")
    .tools(vec![
        AssistantTools::CodeInterpreter
    ])
    .tool_resources(
        AssistantToolCodeInterpreterResources { file_ids: vec![data_file.id.clone()] }
    )
    .build()?;

    let assistant = client.assistants().create(create_assistant_request).await?;

    // create a thread
    let create_message_request = CreateMessageRequestArgs::default()
        .role(MessageRole::User)
        .content("Generate a graph of price index vs year in png format")
        .build()?;

    let create_thread_request = CreateThreadRequest {
        messages: Some(vec![create_message_request]),
        ..Default::default()
    };

    let thread = client.threads().create(create_thread_request).await?;

    // create run and check the output
    let create_run_request = CreateRunRequest {
        assistant_id: assistant.id.clone(),
        ..Default::default()
    };

    let mut run = client
        .threads()
        .runs(&thread.id)
        .create(create_run_request)
        .await?;

    let mut generated_file_ids: Vec<String> = vec![];

    // poll the status of run until its in a terminal state
    loop {
        //check the status of the run
        match run.status {
            RunStatus::Completed => {
                let messages = client
                    .threads()
                    .messages(&thread.id)
                    .list(&[("limit", "10")])
                    .await?;

                for message_obj in messages.data {
                    let message_contents = message_obj.content;
                    for message_content in message_contents {
                        match message_content {
                            MessageContent::Text(text) => {
                                let text_data = text.text;
                                let annotations = text_data.annotations;
                                println!("{}", text_data.value);
                                for annotation in annotations {
                                    match annotation {
                                        MessageContentTextAnnotations::FileCitation(object) => {
                                            println!("annotation: file citation : {object:?}");
                                        }
                                        MessageContentTextAnnotations::FilePath(object) => {
                                            println!("annotation: file path: {object:?}");
                                            generated_file_ids.push(object.file_path.file_id);
                                        }
                                    }
                                }
                            }
                            MessageContent::ImageFile(object) => {
                                let file_id = object.image_file.file_id;
                                println!("Retrieving image file_id: {}", file_id);
                                let contents = client.files().content(&file_id).await?;
                                let path = "./output/price_index_vs_year_graph.png";
                                tokio::fs::write(path, contents).await?;
                                print!("Graph file: {path}");
                                generated_file_ids.push(file_id);
                            }
                            MessageContent::ImageUrl(object) => {
                                eprintln!("Got Image URL instead: {object:?}");
                            }
                            MessageContent::Refusal(refusal) => {
                                println!("{refusal:?}");
                            }
                        }
                    }
                }

                break;
            }
            RunStatus::Failed => {
                println!("> Run Failed: {:#?}", run);
                break;
            }
            RunStatus::Queued => {
                println!("> Run Queued");
            }
            RunStatus::Cancelling => {
                println!("> Run Cancelling");
            }
            RunStatus::Cancelled => {
                println!("> Run Cancelled");
                break;
            }
            RunStatus::Expired => {
                println!("> Run Expired");
                break;
            }
            RunStatus::RequiresAction => {
                println!("> Run Requires Action");
            }
            RunStatus::InProgress => {
                println!("> In Progress ...");
            }
            RunStatus::Incomplete => {
                println!("> Run Incomplete");
            }
        }

        // wait for 1 sec before polling run object again
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        //retrieve the run
        run = client.threads().runs(&thread.id).retrieve(&run.id).await?;
    }

    // clean up
    client.threads().delete(&thread.id).await?;
    client.files().delete(&data_file.id).await?;
    for file_id in generated_file_ids {
        client.files().delete(&file_id).await?;
    }
    client.assistants().delete(&assistant.id).await?;

    Ok(())
}
