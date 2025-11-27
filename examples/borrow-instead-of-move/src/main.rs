use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::responses::{CreateResponse, CreateResponseArgs, Response},
    Client,
};

async fn make_request(
    client: &Client<OpenAIConfig>,
    request: &CreateResponse,
) -> Result<Response, Box<dyn Error>> {
    println!("\nRequest:\n{}", serde_json::to_string(&request)?);

    let response: Response = client.responses().create_byot(request).await?;

    println!("\nResponse:\n{}", response.output_text().ok_or("None")?);

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let input = "What is 10 + 5?";

    let mut request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-5-mini")
        .input(input)
        .build()?;

    // Instead of moving the request, we borrow it.
    // For input and output types - use the types used in
    // corresponding regular method `.create()`.
    let _ = make_request(&client, &request).await?;

    let input = "difference between climate and weather";
    request.input = input.into();

    let _ = make_request(&client, &request).await?;

    Ok(())
}
