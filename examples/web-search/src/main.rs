use async_openai::types::{
    Annotation, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, UrlCitation,
};
use async_openai::Client;
use futures::StreamExt;
use std::error::Error;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let system_prompt = ChatCompletionRequestSystemMessageArgs::default()
        .content(
            "You are a helpful ai search tool. You will help users find and summarize results"
                .to_string(),
        )
        .build()
        .unwrap();

    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .content("I'm looking for headphones. Search the web and find a pair of wireless headphones for under $80".to_string())
        .build()
        .unwrap();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-search-preview".to_string())
        .messages(vec![system_prompt.into(), user_message.into()])
        .stream(true)
        .build()?;

    let client = Client::new();
    let mut stream = client.chat().create_stream(request).await?;

    println!("Streaming response!");
    let mut lock = stdout().lock();
    let mut sources = Vec::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => response.choices.iter().for_each(|choice| {
                if let Some(ref content) = choice.delta.content {
                    write!(lock, "{}", content).unwrap();
                }
                if let Some(ref annotations) = choice.delta.annotations {
                    let mut annotations_clone = annotations.clone();
                    sources.append(&mut annotations_clone);
                }
            }),
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }
    let sources = sources
        .into_iter()
        .map(|source| match source {
            Annotation::UrlCitation {
                url_citation: UrlCitation { url, .. },
                ..
            } => url,
        })
        .collect::<Vec<_>>();

    println!("\nsources: \n{:#?}", sources);

    Ok(())
}
