use std::error::Error;

use async_openai::{
    types::{
        responses::{
            CreateResponseArgs, EasyInputContent, EasyInputMessage, InputItem, InputParam,
            MessageType, ResponseTextParam, Role, TextResponseFormatConfiguration, Tool, Verbosity,
            WebSearchToolArgs,
        },
        MCPToolAllowedTools, MCPToolApprovalSetting, MCPToolArgs, MCPToolRequireApproval,
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
        .input(InputParam::Items(vec![InputItem::EasyMessage(
            EasyInputMessage {
                r#type: MessageType::Message,
                role: Role::User,
                content: EasyInputContent::Text("What transport protocols does the 2025-03-26 version of the MCP spec (modelcontextprotocol/modelcontextprotocol) support?".to_string()),
            }
        )]))
        .tools(vec![
            Tool::WebSearchPreview(WebSearchToolArgs::default().build()?),
            Tool::Mcp(MCPToolArgs::default()
                .server_label("deepwiki")
                .server_url("https://mcp.deepwiki.com/mcp")
                .require_approval(MCPToolRequireApproval::ApprovalSetting(MCPToolApprovalSetting::Never))
                .allowed_tools(MCPToolAllowedTools::List(vec!["ask_question".to_string()]))
                .build()?),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.responses().create(request).await?;
    let output_text = response.output_text().unwrap_or("Empty text output".into());

    println!("\nOutput Text: {output_text:?}\n",);
    for output in response.output {
        println!("\nOutput: {:?}\n", output);
    }

    Ok(())
}
