use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::responses::{
        CreateResponseArgs, ImageDetail, ImageGenTool, InputContent, InputImageContent,
        InputMessage, InputRole, OutputItem, OutputMessageContent,
    },
    Client,
};

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use std::fs::OpenOptions;
use std::io::Write;

async fn analyze_image_url(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    let image_url =
        "https://images.unsplash.com/photo-1554990772-0bea55d510d5?q=80&w=512&auto=format";
    let request = CreateResponseArgs::default()
        .model("gpt-4.1-mini")
        .input(InputMessage {
            content: vec![
                "what is in this image? Along with count of objects in the image?".into(),
                InputContent::InputImage(InputImageContent {
                    detail: ImageDetail::Auto,
                    image_url: Some(image_url.to_string()),
                    file_id: None,
                }),
            ],
            role: InputRole::User,
            status: None,
        })
        .build()?;

    println!(
        "analyze_image_url request:\n{}",
        serde_json::to_string(&request)?
    );

    let response = client.responses().create(request).await?;

    for output in response.output {
        match output {
            OutputItem::Message(message) => {
                for content in message.content {
                    match content {
                        OutputMessageContent::OutputText(text) => {
                            println!("Text: {:?}", text.text);
                        }
                        OutputMessageContent::Refusal(refusal) => {
                            println!("Refusal: {:?}", refusal.refusal);
                        }
                    }
                }
            }
            _ => println!("Other output: {:?}", output),
        }
    }

    Ok(())
}

async fn generate_image(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    let request = CreateResponseArgs::default()
        .model("gpt-4.1-mini")
        .input("Generate an image of gray tabby cat hugging an otter with an orange scarf")
        .tools(ImageGenTool::default())
        .build()?;

    println!(
        "generate_image request:\n{}",
        serde_json::to_string(&request)?
    );

    let response = client.responses().create(request).await?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./data/image.png")?;

    for output in response.output {
        match output {
            OutputItem::ImageGenerationCall(image_gen_call) => {
                if let Some(result) = image_gen_call.result {
                    println!("Image generation call has result");
                    let decoded = BASE64_STANDARD.decode(&result)?;
                    file.write_all(&decoded)?;
                } else {
                    println!("Image generation call has no result");
                }
            }
            _ => println!("Other output: {:?}", output),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    std::fs::create_dir_all("./data")?;

    generate_image(&client).await?;
    analyze_image_url(&client).await?;

    Ok(())
}
