use std::error::Error;

use async_openai::{
    types::{
        AssistantToolFileSearchResources, AssistantToolsFileSearch, CreateAssistantRequestArgs,
        CreateFileRequest, CreateMessageRequestArgs, CreateRunRequest, CreateThreadRequest,
        CreateVectorStoreRequest, FilePurpose, MessageAttachment, MessageAttachmentTool,
        MessageContent, MessageRole, ModifyAssistantRequest, RunStatus,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    //
    // Step 1: Create a new Assistant with File Search Enabled
    //

    let create_assistant_request = CreateAssistantRequestArgs::default()
        .name("Financial Analyst Assistant")
        .instructions("You are an expert financial analyst. Use you knowledge base to answer questions about audited financial statements.")
        .model("gpt-4o")
        .tools(vec![
            AssistantToolsFileSearch::default().into()
        ])
        .build()?;

    let assistant = client.assistants().create(create_assistant_request).await?;

    //
    // Step 2: Upload files and add them to a Vector Store
    //

    // upload file to add to vector store
    let openai_file = client
        .files()
        .create(CreateFileRequest {
            file: "./input/uber-10k.pdf".into(),
            purpose: FilePurpose::Assistants,
        })
        .await?;

    // Create a vector store called "Financial Statements"
    // add uploaded file to vector store
    let vector_store = client
        .vector_stores()
        .create(CreateVectorStoreRequest {
            name: Some("Financial Statements".into()),
            file_ids: Some(vec![openai_file.id.clone()]),
            ..Default::default()
        })
        .await?;

    //
    // Step 3: Update the assistant to to use the new Vector Store
    //

    let assistant = client
        .assistants()
        .update(
            &assistant.id,
            ModifyAssistantRequest {
                tool_resources: Some(
                    AssistantToolFileSearchResources {
                        vector_store_ids: vec![vector_store.id.clone()],
                    }
                    .into(),
                ),
                ..Default::default()
            },
        )
        .await?;

    //
    // Step 4: Create a thread
    //

    // You can also attach files as Message attachments on your thread. Doing so will create another vector_store associated with the thread, or, if there is already a vector store attached to this thread, attach the new files to the existing thread vector store. When you create a Run on this thread, the file search tool will query both the vector_store from your assistant and the vector_store on the thread.

    // Upload user provided file to OpenAI
    let message_file = client
        .files()
        .create(CreateFileRequest {
            file: "./input/lyft-10k.pdf".into(),
            purpose: FilePurpose::Assistants,
        })
        .await?;

    // Create a thread and attach the file to the message

    let create_message_request = CreateMessageRequestArgs::default()
        .role(MessageRole::User)
        .content("What was the total annual profit of Uber and Lyft?")
        .attachments(vec![MessageAttachment {
            file_id: message_file.id.clone(),
            tools: vec![MessageAttachmentTool::FileSearch],
        }])
        .build()?;

    let create_thread_request = CreateThreadRequest {
        messages: Some(vec![create_message_request]),
        ..Default::default()
    };

    let thread = client.threads().create(create_thread_request).await?;

    //
    // Step 5: Create a run and check the output
    //

    let create_run_request = CreateRunRequest {
        assistant_id: assistant.id.clone(),
        ..Default::default()
    };

    let mut run = client
        .threads()
        .runs(&thread.id)
        .create(create_run_request)
        .await?;

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
                                println!("{annotations:?}");
                            }
                            MessageContent::ImageFile(_) | MessageContent::ImageUrl(_) => {
                                eprintln!("Images not supported on terminal");
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
    client.vector_stores().delete(&vector_store.id).await?;
    client.files().delete(&openai_file.id).await?;
    client.files().delete(&message_file.id).await?;
    client.assistants().delete(&assistant.id).await?;

    Ok(())
}
