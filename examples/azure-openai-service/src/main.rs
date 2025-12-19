use std::error::Error;

use async_openai::{
    config::AzureConfig,
    types::{
        chat::ChatCompletionRequestSystemMessageArgs, chat::ChatCompletionRequestUserMessageArgs,
        chat::CreateChatCompletionRequestArgs, embeddings::CreateEmbeddingRequestArgs,
    },
    Client,
};

async fn chat_completion_example(client: &Client<AzureConfig>) -> Result<(), Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("How does large language model work?")
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }
    Ok(())
}

// Bug (help wanted): https://github.com/64bit/async-openai/pull/67#issuecomment-1555165805
// async fn completions_stream_example(client: &Client<AzureConfig>) -> Result<(), Box<dyn Error>> {
//     let request = CreateCompletionRequestArgs::default()
//         .model("text-davinci-003")
//         .n(1)
//         .prompt("Tell me a short bedtime story about Optimus Prime and Bumblebee in Sir David Attenborough voice")
//         .stream(true)
//         .max_tokens(512_u32)
//         .build()?;

//     let mut stream = client.completions().create_stream(request).await?;

//     while let Some(response) = stream.next().await {
//         match response {
//             Ok(ccr) => ccr.choices.iter().for_each(|c| {
//                 print!("{}", c.text);
//             }),
//             Err(e) => eprintln!("{}", e),
//         }
//     }
//     Ok(())
// }

async fn embedding_example(client: &Client<AzureConfig>) -> Result<(), Box<dyn Error>> {
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input("Why do programmers hate nature? It has too many bugs.")
        .build()?;

    let response = client.embeddings().create(request).await?;

    for data in response.data {
        println!(
            "[{}]: has embedding of length {}",
            data.index,
            data.embedding.len()
        )
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AzureConfig::new()
        .with_api_base("https://your-resource-name.openai.azure.com")
        .with_api_key("...")
        .with_deployment_id("deployment-id")
        .with_api_version("2023-03-15-preview");

    let client = Client::with_config(config);

    // Run embedding Example
    embedding_example(&client).await?;

    // Run completions stream Example
    // Bug (help wanted): https://github.com/64bit/async-openai/pull/67#issuecomment-1555165805
    //completions_stream_example(&client).await?;

    // Run chat completion example
    chat_completion_example(&client).await?;

    Ok(())
}
