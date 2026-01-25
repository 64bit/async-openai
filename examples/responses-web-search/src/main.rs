use std::error::Error;

use async_openai::{
    types::responses::{
        CreateResponseArgs, Tool, WebSearchApproximateLocationArgs,
        WebSearchApproximateLocationType, WebSearchToolArgs, WebSearchToolFilters,
        WebSearchToolSearchContextSize,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let user_location = WebSearchApproximateLocationArgs::default()
        .r#type(WebSearchApproximateLocationType::Approximate)
        .city("San Francisco")
        .country("US")
        .build()?;

    let web_search = WebSearchToolArgs::default()
        .search_context_size(WebSearchToolSearchContextSize::Low)
        .filters(WebSearchToolFilters {
            allowed_domains: Some(vec!["news.ycombinator.com".to_string()]),
        })
        .user_location(user_location)
        .build()?;

    let request = CreateResponseArgs::default()
        .model("gpt-5-mini")
        .input(
            "Search the Hacker News front page and summarize the top 3 stories in one sentence each. And also see if there was any new stories about Apple on Hacker News.",
        )
        .tools(vec![Tool::WebSearch(web_search)])
        .build()?;

    let response = client.responses().create(request).await?;

    println!("Response: {:#?}", response);

    Ok(())
}
