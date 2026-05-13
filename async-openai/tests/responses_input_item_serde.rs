#![cfg(feature = "response-types")]

use async_openai::types::responses::{
    EasyInputContent, ImageDetail, InputContent, InputItem, InputRole, Item, MessageItem,
    MessageType, Role, WebSearchApproximateLocation, WebSearchApproximateLocationType,
};
use serde_json::json;

#[test]
fn input_item_easy_message_without_type_defaults_and_serializes_canonically() {
    let input_item: InputItem = serde_json::from_value(json!({
        "role": "user",
        "content": "hello from easy input"
    }))
    .expect("deserialize easy input without type");

    match &input_item {
        InputItem::EasyMessage(msg) => {
            assert_eq!(msg.r#type, MessageType::Message);
            assert_eq!(msg.role, Role::User);
            assert_eq!(
                msg.content,
                EasyInputContent::Text("hello from easy input".to_string())
            );
        }
        other => panic!("expected EasyMessage, got {other:?}"),
    }

    let serialized = serde_json::to_value(&input_item).expect("serialize input item");
    assert_eq!(
        serialized,
        json!({
            "type": "message",
            "role": "user",
            "content": "hello from easy input"
        })
    );
}

#[test]
fn web_search_approximate_location_without_type_defaults_and_serializes_canonically() {
    let location: WebSearchApproximateLocation = serde_json::from_value(json!({
        "country": "US"
    }))
    .expect("deserialize web search approximate location without type");

    assert_eq!(
        location.r#type,
        WebSearchApproximateLocationType::Approximate
    );
    assert_eq!(location.country.as_deref(), Some("US"));
    assert_eq!(location.city.as_deref(), None);
    assert_eq!(location.region.as_deref(), None);
    assert_eq!(location.timezone.as_deref(), None);

    let serialized = serde_json::to_value(&location).expect("serialize web search location");
    assert_eq!(
        serialized,
        json!({
            "type": "approximate",
            "country": "US"
        })
    );
}

#[test]
fn input_item_easy_message_multimodal_without_detail_defaults_and_serializes_canonically() {
    let input_item: InputItem = serde_json::from_value(json!({
        "role": "user",
        "content": [
            {"type": "input_text", "text": "describe this"},
            {"type": "input_image", "image_url": "https://example.com/cat.png"}
        ]
    }))
    .expect("deserialize easy input image without detail");

    match &input_item {
        InputItem::EasyMessage(msg) => {
            assert_eq!(msg.r#type, MessageType::Message);
            assert_eq!(msg.role, Role::User);
            match &msg.content {
                EasyInputContent::ContentList(parts) => {
                    assert_eq!(parts.len(), 2);
                    match &parts[1] {
                        InputContent::InputImage(img) => {
                            assert_eq!(img.detail, ImageDetail::Auto);
                            assert_eq!(
                                img.image_url.as_deref(),
                                Some("https://example.com/cat.png")
                            );
                            assert_eq!(img.file_id, None);
                        }
                        other => panic!("expected InputImage, got {other:?}"),
                    }
                }
                other => panic!("expected ContentList, got {other:?}"),
            }
        }
        other => panic!("expected EasyMessage, got {other:?}"),
    }

    let serialized = serde_json::to_value(&input_item).expect("serialize input item");
    assert_eq!(
        serialized,
        json!({
            "type": "message",
            "role": "user",
            "content": [
                {"type": "input_text", "text": "describe this"},
                {"type": "input_image", "detail": "auto", "image_url": "https://example.com/cat.png"}
            ]
        })
    );
}

#[test]
fn input_item_strict_message_multimodal_without_detail_defaults() {
    let input_item: InputItem = serde_json::from_value(json!({
        "type": "message",
        "role": "user",
        "content": [
            {"type": "input_image", "image_url": "https://example.com/cat.png"}
        ]
    }))
    .expect("deserialize strict input image without detail");

    match &input_item {
        InputItem::Item(Item::Message(MessageItem::Input(msg))) => {
            assert_eq!(msg.role, InputRole::User);
            match &msg.content[0] {
                InputContent::InputImage(img) => {
                    assert_eq!(img.detail, ImageDetail::Auto);
                }
                other => panic!("expected InputImage, got {other:?}"),
            }
        }
        other => panic!("expected Item::Message(Input), got {other:?}"),
    }
}
