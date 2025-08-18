use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContentPart, CreateChatCompletionRequestArgs,
        CreateEmbeddingRequestArgs, CreateImageRequestArgs, Image, ImageModel, ImageResponseFormat,
        InputAudio, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use base64::Engine;
use dotenv::dotenv;
use futures::StreamExt;
use gemini_types::{
    GeminiChatCompletionResponseStream, GeminiCreateChatCompletionResponse,
    GeminiCreateEmbeddingResponse, GeminiImagesResponse, GeminiModel, ListGeminiModelResponse,
};
use serde_json::json;
use std::error::Error;
use std::fs;
mod gemini_types;

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
    let models: ListGeminiModelResponse = client.models().list_byot().await?;

    println!("Available Models:");
    for model in models.data {
        println!("ID: {}", model.id);
        println!("Object: {}", model.object);
        println!("Owned By: {}", model.owned_by);
        println!();
    }
    Ok(())
}

/// Retrieves details of a specific model
async fn retrieve_model(model_id: &str) -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();
    let model: GeminiModel = client.models().retrieve_byot(model_id).await?;

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

    let mut stream: GeminiChatCompletionResponseStream =
        client.chat().create_stream_byot(request).await?;

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

    let response: GeminiCreateChatCompletionResponse = client.chat().create_byot(request).await?;

    // if let Ok(response) = response {
    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.message.content.unwrap());
    }
    Ok(())
}

async fn function_call() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();

    let response: serde_json::Value = client
        .chat()
        .create_byot(serde_json::json!({

                "model": "gemini-2.0-flash",
                "messages": [
                  {
                    "role": "user",
                    "content": "What'\''s the weather like in Chicago today?"
                  }
                ],
                "tools": [
                  {
                    "type": "function",
                    "function": {
                      "name": "get_weather",
                      "description": "Get the current weather in a given location",
                      "parameters": {
                        "type": "object",
                        "properties": {
                          "location": {
                            "type": "string",
                            "description": "The city and state, e.g. Chicago, IL"
                          },
                          "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"]
                          }
                        },
                        "required": ["location"]
                      }
                    }
                  }
                ],
                "tool_choice": "auto"

        }))
        .await?;

    println!("\nResponse (function call):\n");
    println!("{}", response);
    Ok(())
}

async fn image_understanding() -> Result<(), Box<dyn Error>> {
    // ref: https://unsplash.com/photos/an-elderly-couple-walks-through-a-park-Mpf6IQpiq3A
    let image_file = "./sample_data/gavin-allanwood-Mpf6IQpiq3A-unsplash.jpg";
    let image_data = fs::read(image_file)?;
    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_data);
    let client = get_gemini_client();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gemini-2.0-flash")
        .messages([
            ChatCompletionRequestMessage::User("What do you see in this image?".into()),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestUserMessageContentPart::ImageUrl(
                        async_openai::types::ChatCompletionRequestMessageContentPartImage {
                            image_url: ("data:image/jpg;base64,".to_string() + &image_base64)
                                .into(),
                        },
                    ),
                ]),
                ..Default::default()
            }),
        ])
        .build()?;

    let response: GeminiCreateChatCompletionResponse = client.chat().create_byot(request).await?;

    println!("\nResponse (image understanding):\n");
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

    let response: GeminiImagesResponse = client.images().create_byot(request).await?;

    let images = response.data;

    println!("\nResponse (image):\n");
    for image in images {
        if let Image::B64Json {
            b64_json,
            revised_prompt: _,
        } = &*image
        {
            println!("Image b64_json: {}", b64_json);
        } else if let Image::Url {
            url,
            revised_prompt: _,
        } = &*image
        {
            println!("Image URL: {}", url);
        }
    }

    Ok(())
}

async fn audio_understanding() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();

    // Credits and Source for audio: https://www.youtube.com/watch?v=W05FYkqv7hM
    let audio_file = "./sample_data/How to Stop Holding Yourself Back Simon Sinek.mp3";
    let audio_data = fs::read(audio_file)?;
    let audio_base64 = base64::engine::general_purpose::STANDARD.encode(audio_data);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gemini-2.0-flash")
        .messages([
            ChatCompletionRequestMessage::User("Transcribe this audio file.".into()),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestUserMessageContentPart::InputAudio(
                        async_openai::types::ChatCompletionRequestMessageContentPartAudio {
                            input_audio: InputAudio {
                                data: audio_base64,
                                format: async_openai::types::InputAudioFormat::Mp3,
                            },
                        },
                    ),
                ]),
                ..Default::default()
            }),
        ])
        .build()?;

    let response: GeminiCreateChatCompletionResponse = client.chat().create_byot(request).await?;

    println!("\nResponse (audio understanding):\n");

    for choice in response.choices {
        println!("{}", choice.message.content.unwrap());
    }

    Ok(())
}

async fn structured_output() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();

    let schema = json!({
        "type": "object",
        "properties": {
            "steps": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "explanation": { "type": "string" },
                        "output": { "type": "string" }
                    },
                    "required": ["explanation", "output"],
                    "additionalProperties": false
                }
            },
            "final_answer": { "type": "string" }
        },
        "required": ["steps", "final_answer"],
        "additionalProperties": false
    });

    let request = CreateChatCompletionRequestArgs::default()
        .model("gemini-2.0-flash")
        .messages([ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                    "How can I solve 8x + 7 = -23?".to_string(),
                ),
                ..Default::default()
            },
        )])
        .response_format(ResponseFormat::JsonSchema {
            json_schema: ResponseFormatJsonSchema {
                schema: Some(schema),
                description: None,
                name: "math_reasoning".into(),
                strict: Some(true),
            },
        })
        .build()?;

    let response: GeminiCreateChatCompletionResponse = client.chat().create_byot(request).await?;

    println!("\nResponse (structured output):\n");
    for choice in response.choices {
        println!("{}", choice.message.content.unwrap());
    }

    Ok(())
}

async fn create_embeddings() -> Result<(), Box<dyn Error>> {
    let client = get_gemini_client();

    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-004")
        .input("The food was delicious and the waiter...")
        .build()?;

    let response: GeminiCreateEmbeddingResponse = client.embeddings().create_byot(request).await?;

    println!("\nResponse (embedding):\n");
    for embedding in response.data {
        println!("Embedding: {:?}", embedding.embedding);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    if let Err(e) = list_models().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = retrieve_model("gemini-2.0-flash").await {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = chat_completion().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = function_call().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = stream_chat().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = image_understanding().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = generate_image("a futuristic city at night").await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = audio_understanding().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = structured_output().await {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = create_embeddings().await {
        eprintln!("Error: {}", e);
    }
}
