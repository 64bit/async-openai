use async_openai::{
    types::{
        responses::{EasyInputContent, EasyInputMessage, InputItem, MessageType, Role},
        CreateConversationItemsRequestArgs, CreateConversationRequestArgs,
        ListConversationItemsQuery, UpdateConversationRequestArgs,
    },
    Client,
};
use std::collections::HashMap;

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
                .metadata({
                    let mut metadata = HashMap::new();
                    metadata.insert("topic".to_string(), serde_json::json!("demo"));
                    metadata
                })
                .items(vec![InputItem::from_easy_message(EasyInputMessage {
                    r#type: MessageType::Message,
                    role: Role::User,
                    content: EasyInputContent::Text(
                        "Hello! Can you help me understand conversations?".to_string(),
                    ),
                })])
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
                    InputItem::from_easy_message(EasyInputMessage {
                        r#type: MessageType::Message,
                        role: Role::User,
                        content: EasyInputContent::Text("What are the main features?".to_string()),
                    }),
                    InputItem::from_easy_message(EasyInputMessage {
                        r#type: MessageType::Message,
                        role: Role::User,
                        content: EasyInputContent::Text("Can you give me an example?".to_string()),
                    }),
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
        .list(&query)
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
                .metadata({
                    let mut metadata = HashMap::new();
                    metadata.insert("topic".to_string(), serde_json::json!("updated-demo"));
                    metadata.insert("status".to_string(), serde_json::json!("active"));
                    metadata
                })
                .build()?,
        )
        .await?;

    println!("Updated metadata: {:?}\n", updated_conversation.metadata);

    // 6. Delete an item from the conversation
    if !all_items.data.is_empty() {
        println!("6. Deleting an item from the conversation...");
        let item_to_delete = &all_items.last_id;
        let updated_conv = client
            .conversations()
            .items(&conversation.id)
            .delete(item_to_delete)
            .await?;
        println!("Item deleted. Conversation still exists: {}\n", updated_conv.id);
    }

    // 7. Retrieve the conversation
    println!("7. Retrieving the conversation...");
    let retrieved_conversation = client.conversations().retrieve(&conversation.id).await?;
    println!("Retrieved conversation: {}", retrieved_conversation.id);
    println!("Metadata: {:?}\n", retrieved_conversation.metadata);

    // 8. Delete the conversation
    println!("8. Deleting the conversation...");
    let deleted = client.conversations().delete(&conversation.id).await?;
    println!("Conversation deleted: {}", deleted.deleted);
    println!("Deleted ID: {}\n", deleted.id);

    println!("=== Example completed successfully! ===");

    Ok(())
}

