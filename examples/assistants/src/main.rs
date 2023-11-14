use async_openai::{
    types::{CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, CreateThreadAndRunRequestArgs, CreateThreadAndRunRequest, CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs, Role, RunStatus, MessageContent, MessageContentTextObject},
    Client, config::OpenAIConfig,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = [("limit", "1")]; //limit the list responses to 1 message

    //our api key
    let KEY = "YOUR KEY HERE";

    //the id of the Assistant we want to use
    let rust_bot_id = "Your Assistant ID Here";
    
    //create a client
    let config = Client::new();

    //create a thread for the conversation
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;

    loop{
        println!("How can I help you?");
        //get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //create a message for the thread
        let message = CreateMessageRequestArgs::default()
            .role("user")
            .content(input.clone())
            .build()?;

        //attach message to the thread
        let message_obj = client
            .threads()
            .messages(&thread.id)
            .create(message)
            .await?;

        //create a run for the thread
        let run_request = CreateRunRequestArgs::default()
            .assistant_id(rust_bot_id)
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
                    //in the thread

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
                    println!("Response: {}", text);

                }
                RunStatus::Failed => {
                    awaiting_response = false;
                    println!("Run Failed: {:#?}", run);
                }
                RunStatus::Queued => {
                    println!("Run Queued");
                },
                RunStatus::Cancelling => {
                    println!("Run Cancelling");
                },
                RunStatus::Cancelled => {
                    println!("Run Cancelled");
                },
                RunStatus::Expired => {
                    println!("Run Expired");
                },
                RunStatus::RequiresAction => {
                    println!("Run Requires Action");
                },
                RunStatus::InProgress => {
                    println!("Waiting for response...");
                },
                _ => {
                    println!("Waiting for response...");
                }
            }
            //wait for 1 second before checking the status again
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    Ok(())
}