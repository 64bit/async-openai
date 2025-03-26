use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
        CreateImageRequestArgs, ImageModel, ImageResponseFormat,
        Model,
    },
    Client,
};
use dotenv::dotenv;
use futures::StreamExt;
use std::error::Error;

/// Initializes the OpenAI client with Gemini API compatibility
fn get_gemini_client() -> Client<OpenAIConfig> {
    let base_url = "https://generativelanguage.googleapis.com/v1beta/openai";
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);
    Client::with_config(config)
}

/// Lists available models from the Gemini API
async fn list_models() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let models = client.models().list().await?;

    println!("Available Models:");
    for model in models.data {
        println!("- {}", model.id);
    }
    Ok(())
}

/// Retrieves details of a specific model
async fn retrieve_model(model_id: &str) -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let model: Model = client.models().retrieve(model_id).await?;

    println!("Model: {:?}", model);
    Ok(())
}

/// Streams a chat response using Gemini API
async fn stream_chat() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let request = CreateChatCompletionRequestArgs::default()
        //Usage of gemini model
        .model("gemini-2.0-flash")
        .messages(vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                    "What is the meaning of life?".to_string(),
                ),
                ..Default::default()
            },
        )])
        .n(1)
        .stream(true)
        .max_tokens(500_u32)
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                print!("{}", c.delta.content.clone().unwrap());
            }),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

async fn chat_completion() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let request = CreateChatCompletionRequestArgs::default()
        //Usage of gemini model
        .model("gemini-2.0-flash")
        .messages([ChatCompletionRequestMessage::User(
            "How old is the human civilization?".into(),
        )])
        // .max_tokens(40_u32)
        .build()?;

    let response = client.chat().create(request).await?;

    // if let Ok(response) = response {
    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.message.content.unwrap());
    }
    Ok(())
}

/// Generates an image based on a text prompt
async fn generate_image(prompt: &str) -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();

    let request = CreateImageRequestArgs::default()
        .prompt(prompt)
        //Usage of gemini model
        .model(ImageModel::Other("imagen-3.0-generate-002".to_string()))
        .n(1)
        .response_format(ImageResponseFormat::B64Json)
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    // Choose an operation
    if let Err(e) = list_models().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = retrieve_model("gemini-2.0-flash").await {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = chat_completion().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = stream_chat().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = generate_image("a futuristic city at night").await {
        eprintln!("Error: {}", e);
    }
}
