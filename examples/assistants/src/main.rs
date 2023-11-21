use async_openai::{
    types::{CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, RunStatus, MessageContent, CreateAssistantRequestArgs},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = [("limit", "1")]; //limit the list responses to 1 message
    
    //create a client
    let client = Client::new();

    //create a thread for the conversation
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;

    //ask the user for the name of the assistant
    println!("--- Enter the name of your assistant");
    //get user input
    let mut assistant_name = String::new();
    std::io::stdin().read_line(&mut assistant_name).unwrap();

    //ask the user for the instruction set for the assistant
    println!("--- Enter the instruction set for your new assistant");
    //get user input
    let mut instructions = String::new();
    std::io::stdin().read_line(&mut instructions).unwrap();
    
    //create the assistant
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&assistant_name)
        .instructions(&instructions)
        .model("gpt-3.5-turbo-1106")
        .build()?;
    let assistant = client.assistants().create(assistant_request).await?;
    //get the id of the assistant
    let assistant_id = &assistant.id;

    loop{
        println!("--- How can I help you?");
        //get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //break out of the loop if the user enters exit()
        if input.trim() == "exit()" {
            break;
        }   

        //create a message for the thread
        let message = CreateMessageRequestArgs::default()
            .role("user")
            .content(input.clone())
            .build()?;

        //attach message to the thread
        let _message_obj = client
            .threads()
            .messages(&thread.id)
            .create(message)
            .await?;

        //create a run for the thread
        let run_request = CreateRunRequestArgs::default()
            .assistant_id(assistant_id)
            .build()?;
        let run = client
            .threads()
            .runs(&thread.id)
            .create(run_request)
            .await?;

        //wait for the run to complete
        let mut awaiting_response = true;
        while awaiting_response {
            //retrieve the run
            let run = client
                .threads()
                .runs(&thread.id)
                .retrieve(&run.id)
                .await?;
            //check the status of the run
            match run.status {
                RunStatus::Completed => {
                    awaiting_response = false;
                    // once the run is completed we
                    // get the response from the run
                    // which will be the first message
                    // in the thread

                    //retrieve the response from the run
                    let response = client
                        .threads()
                        .messages(&thread.id)
                        .list(&query)
                        .await?;
                    //get the message id from the response
                    let message_id = response
                        .data.get(0).unwrap()
                        .id.clone();
                    //get the message from the response
                    let message = client
                        .threads()
                        .messages(&thread.id)
                        .retrieve(&message_id)
                        .await?;
                    //get the content from the message
                    let content = message
                        .content.get(0).unwrap();
                    //get the text from the content
                    let text = match content {
                        MessageContent::Text(text) => text.text.value.clone(),
                        MessageContent::ImageFile(_) => panic!("imaged are not supported in the terminal"),
                    };
                    //print the text
                    println!("--- Response: {}", text);
                    println!("");

                }
                RunStatus::Failed => {
                    awaiting_response = false;
                    println!("--- Run Failed: {:#?}", run);
                }
                RunStatus::Queued => {
                    println!("--- Run Queued");
                },
                RunStatus::Cancelling => {
                    println!("--- Run Cancelling");
                },
                RunStatus::Cancelled => {
                    println!("--- Run Cancelled");
                },
                RunStatus::Expired => {
                    println!("--- Run Expired");
                },
                RunStatus::RequiresAction => {
                    println!("--- Run Requires Action");
                },
                RunStatus::InProgress => {
                    println!("--- Waiting for response...");
                }
            }
            //wait for 1 second before checking the status again
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    //once we have broken from the main loop we can delete the assistant and thread
    client.assistants().delete(assistant_id).await?;
    client.threads().delete(&thread.id).await?;

    Ok(())
}