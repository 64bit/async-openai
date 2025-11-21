use async_openai::{
    traits::RequestOptionsBuilder,
    types::responses::{
        ConversationItem, CreateConversationItemsRequestArgs, CreateConversationRequestArgs,
        EasyInputMessage, ListConversationItemsQuery, UpdateConversationRequestArgs,
    },
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = Client::new();

    println!("=== Conversations API Example ===\n");

    // 1. Create a conversation with initial items
    println!("1. Creating a conversation with initial messages...");
    let conversation = client
        .conversations()
        .create(
            CreateConversationRequestArgs::default()
                .metadata(json!({
                    "topic": "demo",
                }))
                .items(vec![EasyInputMessage::from(
                    "Hello! Can you help me understand conversations?",
                )
                .into()])
                .build()?,
        )
        .await?;

    println!("Created conversation: {}", conversation.id);
    println!("Created at: {}", conversation.created_at);
    println!("Metadata: {:?}\n", conversation.metadata);

    // 2. Add more items to the conversation
    println!("2. Adding more items to the conversation...");
    let items_list = client
        .conversations()
        .items(&conversation.id)
        .create(
            CreateConversationItemsRequestArgs::default()
                .items(vec![
                    EasyInputMessage::from("What are the main features?").into(),
                    EasyInputMessage::from("Can you give me an example?").into(),
                ])
                .build()?,
        )
        .await?;

    println!("Added {} items", items_list.data.len());
    println!("First item ID: {}", items_list.first_id);
    println!("Last item ID: {}\n", items_list.last_id);

    // 3. List all items in the conversation
    println!("3. Listing conversation items...");
    let query = ListConversationItemsQuery {
        limit: Some(10),
        order: None,
        after: None,
        include: None,
    };
    let all_items = client
        .conversations()
        .items(&conversation.id)
        .query(&query)?
        .list()
        .await?;

    println!("Total items retrieved: {}", all_items.data.len());
    println!("Has more: {}", all_items.has_more);
    for (i, item) in all_items.data.iter().enumerate() {
        println!("  Item {}: {:?}", i + 1, item);
    }
    println!();

    // 4. Retrieve a specific item
    if !all_items.data.is_empty() {
        println!("4. Retrieving a specific item...");
        let first_item_id = &all_items.first_id;
        let item = client
            .conversations()
            .items(&conversation.id)
            .retrieve(first_item_id)
            .await?;
        println!("Retrieved item: {:?}\n", item);
    }

    // 5. Update conversation metadata
    println!("5. Updating conversation metadata...");
    let updated_conversation = client
        .conversations()
        .update(
            &conversation.id,
            UpdateConversationRequestArgs::default()
                .metadata(json!({
                    "topic": "updated-demo",
                }))
                .build()?,
        )
        .await?;

    println!("Updated metadata: {:?}\n", updated_conversation.metadata);

    // 6. Retrieve the conversation
    println!("6. Retrieving the conversation...");
    let retrieved_conversation = client.conversations().retrieve(&conversation.id).await?;
    println!("Retrieved conversation: {}", retrieved_conversation.id);
    println!("Metadata: {:?}\n", retrieved_conversation.metadata);

    // 7. Delete the conversation items.
    println!("7. Deleting the conversation items...");
    for item in all_items.data {
        let item_id = match item {
            ConversationItem::Message(message) => message.id,
            ConversationItem::FileSearchCall(file_search_tool_call) => file_search_tool_call.id,
            ConversationItem::WebSearchCall(web_search_tool_call) => web_search_tool_call.id,
            ConversationItem::ImageGenerationCall(image_gen_tool_call) => image_gen_tool_call.id,
            ConversationItem::ComputerCall(computer_tool_call) => computer_tool_call.id,
            ConversationItem::Reasoning(reasoning_item) => reasoning_item.id,
            ConversationItem::CodeInterpreterCall(code_interpreter_tool_call) => {
                code_interpreter_tool_call.id
            }
            ConversationItem::LocalShellCall(local_shell_tool_call) => local_shell_tool_call.id,
            ConversationItem::LocalShellCallOutput(local_shell_tool_call_output) => {
                local_shell_tool_call_output.id
            }
            ConversationItem::McpListTools(mcplist_tools) => mcplist_tools.id,
            ConversationItem::McpApprovalRequest(mcpapproval_request) => mcpapproval_request.id,
            ConversationItem::McpApprovalResponse(mcpapproval_response) => {
                mcpapproval_response.id.unwrap()
            }
            ConversationItem::McpCall(mcptool_call) => mcptool_call.id,
            ConversationItem::CustomToolCall(custom_tool_call) => custom_tool_call.id,
            ConversationItem::CustomToolCallOutput(custom_tool_call_output) => {
                custom_tool_call_output.id.unwrap()
            }
            ConversationItem::ItemReference(any_item_reference) => any_item_reference.id,
        };

        let conversation_resource = client
            .conversations()
            .items(&conversation.id)
            .delete(&item_id)
            .await?;
        println!(
            "Item deleted: item id: {item_id}, conversation id: {}",
            conversation_resource.id
        );
    }

    // 8. Delete the conversation
    println!("8. Deleting the conversation...");
    let deleted = client.conversations().delete(&conversation.id).await?;
    println!("Conversation deleted: {}", deleted.deleted);
    println!("Deleted ID: {}\n", deleted.id);

    println!("=== Example completed successfully! ===");

    Ok(())
}
