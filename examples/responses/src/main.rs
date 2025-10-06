use std::error::Error;

use async_openai::{
    types::responses::{
        AllowedTools, CreateResponseArgs, Input, InputItem, InputMessageArgs, McpArgs, RequireApproval, RequireApprovalPolicy, Role, TextConfig, ToolDefinition::{Mcp, WebSearchPreview}, Verbosity, WebSearchPreviewArgs
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .text(TextConfig {
            format: async_openai::types::responses::TextResponseFormat::Text,
            verbosity: Some(Verbosity::Medium), // only here to test the config, but gpt-4.1 only supports medium
        })
        .input(Input::Items(vec![InputItem::Message(
            InputMessageArgs::default()
                .role(Role::User)
                .content("What transport protocols does the 2025-03-26 version of the MCP spec (modelcontextprotocol/modelcontextprotocol) support?")
                .build()?,
        )]))
        .tools(vec![
            WebSearchPreview(WebSearchPreviewArgs::default().build()?),
            Mcp(McpArgs::default()
                .server_label("deepwiki")
                .server_url("https://mcp.deepwiki.com/mcp")
                .require_approval(RequireApproval::Policy(RequireApprovalPolicy::Never))
                .allowed_tools(AllowedTools::List(vec!["ask_question".to_string()]))
                .build()?),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    for output in response.output {
        println!("\nOutput: {:?}\n", output);
    }

    Ok(())
}
