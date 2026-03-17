#![cfg(feature = "response-types")]

use async_openai::types::responses::{CreateResponse, InputItem, Item, OutputItem};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

fn assert_json_roundtrip<T>(value: Value)
where
    T: DeserializeOwned + Serialize + std::fmt::Debug,
{
    let parsed: T = serde_json::from_value(value.clone()).expect("deserialize typed value");
    let serialized = serde_json::to_value(&parsed).expect("serialize typed value");
    assert_eq!(serialized, value);
}

#[test]
fn responses_create_request_round_trips_hosted_tool_search_namespace_and_deferred_functions() {
    assert_json_roundtrip::<CreateResponse>(json!({
        "model": "gpt-5.4",
        "input": "List open orders for customer CUST-12345.",
        "parallel_tool_calls": false,
        "tools": [
            {
                "type": "namespace",
                "name": "crm",
                "description": "CRM tools for customer lookup and order management.",
                "tools": [
                    {
                        "type": "function",
                        "name": "get_customer_profile",
                        "description": "Fetch a customer profile by customer ID.",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "customer_id": { "type": "string" }
                            },
                            "required": ["customer_id"],
                            "additionalProperties": false
                        },
                        "strict": true
                    },
                    {
                        "type": "function",
                        "name": "list_open_orders",
                        "description": "List open orders for a customer ID.",
                        "defer_loading": true,
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "customer_id": { "type": "string" }
                            },
                            "required": ["customer_id"],
                            "additionalProperties": false
                        },
                        "strict": true
                    }
                ]
            },
            {
                "type": "tool_search"
            }
        ]
    }));
}

#[test]
fn responses_create_request_round_trips_mixed_function_namespace_and_tool_search_tools() {
    assert_json_roundtrip::<CreateResponse>(json!({
        "model": "gpt-5.4",
        "input": "List open orders for customer CUST-12345.",
        "tools": [
            {
                "type": "function",
                "name": "lookup_customer",
                "description": "Look up a customer by customer ID.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "customer_id": { "type": "string" }
                    },
                    "required": ["customer_id"],
                    "additionalProperties": false
                },
                "strict": true
            },
            {
                "type": "namespace",
                "name": "orders",
                "description": "Order lookup and fulfillment tools.",
                "tools": [
                    {
                        "type": "function",
                        "name": "list_open_orders",
                        "description": "List open orders for a customer ID.",
                        "defer_loading": true,
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "customer_id": { "type": "string" }
                            },
                            "required": ["customer_id"],
                            "additionalProperties": false
                        },
                        "strict": true
                    }
                ]
            },
            {
                "type": "tool_search"
            }
        ]
    }));
}

#[test]
fn responses_output_item_deserializes_hosted_tool_search_call() {
    assert_json_roundtrip::<OutputItem>(json!({
        "type": "tool_search_call",
        "id": "tsc_123",
        "execution": "server",
        "call_id": null,
        "status": "completed",
        "arguments": {
            "paths": ["crm"]
        }
    }));
}

#[test]
fn responses_output_item_deserializes_hosted_tool_search_output() {
    assert_json_roundtrip::<OutputItem>(json!({
        "type": "tool_search_output",
        "id": "tso_123",
        "execution": "server",
        "call_id": null,
        "status": "completed",
        "tools": [
            {
                "type": "namespace",
                "name": "crm",
                "description": "CRM tools for customer lookup and order management.",
                "tools": [
                    {
                        "type": "function",
                        "name": "list_open_orders",
                        "description": "List open orders for a customer ID.",
                        "defer_loading": true,
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "customer_id": { "type": "string" }
                            },
                            "required": ["customer_id"],
                            "additionalProperties": false
                        },
                        "strict": true
                    }
                ]
            }
        ]
    }));
}

#[test]
fn responses_create_request_round_trips_client_tool_search_configuration() {
    assert_json_roundtrip::<CreateResponse>(json!({
        "model": "gpt-5.4",
        "input": "Find the shipping ETA tool first, then use it for order_42.",
        "parallel_tool_calls": false,
        "tools": [
            {
                "type": "tool_search",
                "execution": "client",
                "description": "Find the project-specific tools needed to continue the task.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "goal": { "type": "string" }
                    },
                    "required": ["goal"],
                    "additionalProperties": false
                }
            }
        ]
    }));
}

#[test]
fn responses_output_item_deserializes_client_tool_search_call() {
    assert_json_roundtrip::<OutputItem>(json!({
        "type": "tool_search_call",
        "id": "tsc_456",
        "execution": "client",
        "call_id": "call_abc123",
        "status": "completed",
        "arguments": {
            "goal": "Find the shipping ETA tool for order_42."
        }
    }));
}

#[test]
fn responses_input_item_round_trips_client_tool_search_output() {
    assert_json_roundtrip::<InputItem>(json!({
        "type": "tool_search_output",
        "id": "tso_456",
        "execution": "client",
        "call_id": "call_abc123",
        "status": "completed",
        "tools": [
            {
                "type": "function",
                "name": "get_shipping_eta",
                "description": "Look up shipping ETA details for an order.",
                "defer_loading": true,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "order_id": { "type": "string" }
                    },
                    "required": ["order_id"],
                    "additionalProperties": false
                },
                "strict": true
            }
        ]
    }));
}

#[test]
fn responses_input_item_round_trips_function_call_with_namespace() {
    assert_json_roundtrip::<Item>(json!({
        "type": "function_call",
        "id": "fc_123",
        "call_id": "call_xyz456",
        "namespace": "crm",
        "name": "list_open_orders",
        "arguments": "{\"customer_id\":\"CUST-12345\"}",
        "status": "completed"
    }));
}
