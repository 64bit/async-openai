#![cfg(feature = "response-types")]

use async_openai::types::responses::{
    EasyInputContent, InputItem, MessageType, Role, WebSearchApproximateLocation,
    WebSearchApproximateLocationType,
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
