use std::error::Error;

use async_openai::types::OutputContent::McpApprovalRequest;
use async_openai::types::ToolDefinition::Mcp;
use async_openai::types::{AllowedTools, McpArgs, RequireApproval, RequireApprovalPolicy};
use async_openai::{
    types::{
        CreateResponseRequestArgs, InputItem, InputMessageArgs, ResponseFormatJsonSchema,
        ResponseInput, ResponsesRole, TextConfig, TextResponseFormat,
        ToolDefinition::WebSearchPreview, WebSearchPreviewArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateResponseRequestArgs::default()
        .max_output_tokens(512u32)
        .model("gpt-4.1")
        .input(ResponseInput::Items(vec![InputItem::Message(
            InputMessageArgs::default()
                .role(ResponsesRole::User)
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
