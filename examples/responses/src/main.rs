use std::error::Error;

use async_openai::{
    types::{
        mcp::{MCPToolApprovalSetting, MCPToolArgs},
        responses::{
            CreateResponseArgs, ResponseTextParam, TextResponseFormatConfiguration, Tool,
            Verbosity, WebSearchTool,
        },
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateResponseArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .text(ResponseTextParam {
            format: TextResponseFormatConfiguration::Text,
            verbosity: Some(Verbosity::Medium), // only here to test the config, but gpt-4.1 only supports medium
        })
        .input([
            "What transport protocols does the 2025-03-26 version of the MCP spec (modelcontextprotocol/modelcontextprotocol) support?",
            "what is MCP?"
        ])
        .tools(vec![
            Tool::WebSearchPreview(WebSearchTool::default()),
            Tool::Mcp(MCPToolArgs::default()
                .server_label("deepwiki")
                .server_url("https://mcp.deepwiki.com/mcp")
                .require_approval(MCPToolApprovalSetting::Never)
                .allowed_tools(["ask_question"])
                .build()?),
        ])
        .build()?;

    println!("Request:\n{}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;

    println!("\n SDK: output_text()\n: {:?}", response.output_text());

    for output in response.output {
        println!("\nOutput: {:?}\n", output);
    }

    Ok(())
}
