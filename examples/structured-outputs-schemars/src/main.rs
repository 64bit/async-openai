use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs, ResponseFormat,
        ResponseFormatJsonSchema,
    },
    Client,
};
use schemars::{schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub output: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MathReasoningResponse {
    pub final_answer: String,
    pub steps: Vec<Step>,
}

pub async fn structured_output<T: serde::Serialize + DeserializeOwned + JsonSchema>(
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<Option<T>, Box<dyn Error>> {
    let schema = schema_for!(T);
    let schema_value = serde_json::to_value(&schema)?;
    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: None,
            name: "math_reasoning".into(),
            schema: Some(schema_value),
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-mini")
        .messages(messages)
        .response_format(response_format)
        .build()?;

    let client = Client::new();
    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            return Ok(Some(serde_json::from_str::<T>(&content)?));
        }
    }

    Ok(None)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Expecting output schema
    // let schema = json!({
    //   "type": "object",
    //   "properties": {
    //     "steps": {
    //       "type": "array",
    //       "items": {
    //         "type": "object",
    //         "properties": {
    //           "explanation": { "type": "string" },
    //           "output": { "type": "string" }
    //         },
    //         "required": ["explanation", "output"],
    //         "additionalProperties": false
    //       }
    //     },
    //     "final_answer": { "type": "string" }
    //   },
    //   "required": ["steps", "final_answer"],
    //   "additionalProperties": false
    // });
    if let Some(response) = structured_output::<MathReasoningResponse>(vec![
        ChatCompletionRequestSystemMessage::from(
            "You are a helpful math tutor. Guide the user through the solution step by step.",
        )
        .into(),
        ChatCompletionRequestUserMessage::from("how can I solve 8x + 7 = -23").into(),
    ])
    .await?
    {
        println!("{}", serde_json::to_string(&response).unwrap());
    }

    Ok(())
}
