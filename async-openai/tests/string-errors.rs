#![allow(dead_code)]
//! The purpose of this test to make sure that with the string-errors feature enabled, the error is returned as a string.
//! Enabling the byot feature allows for a simpler test, as the body can be written as an empty json value.

use async_openai::{error::OpenAIError, Client};
use serde_json::{json, Value};

#[tokio::test]
async fn test_byot_errors() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.chat().create_byot(json!({})).await;

    match _r.unwrap_err() {
        OpenAIError::ApiError(raw_error) => {
            let _value: Value = raw_error.parse().unwrap();
        }
        _ => {}
    };
}
