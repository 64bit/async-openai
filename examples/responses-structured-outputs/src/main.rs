use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    traits::EventType,
    types::responses::{
        CreateResponseArgs, InputMessage, InputRole, OutputItem, OutputMessageContent,
        ResponseFormatJsonSchema, ResponseStreamEvent,
    },
    Client,
};
use clap::Parser;
use futures::StreamExt;
use serde_json::json;
use std::io::{stdout, Write};

/// Chain of thought example: Guides the model through step-by-step reasoning
async fn chain_of_thought(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    println!("=== Chain of Thought Example ===\n");

    let schema = json!({
        "type": "object",
        "properties": {
            "steps": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "explanation": { "type": "string" },
                        "output": { "type": "string" }
                    },
                    "required": ["explanation", "output"],
                    "additionalProperties": false
                }
            },
            "final_answer": { "type": "string" }
        },
        "required": ["steps", "final_answer"],
        "additionalProperties": false
    });

    let request = CreateResponseArgs::default()
        .model("gpt-4o-2024-08-06")
        .max_output_tokens(512u32)
        .text(ResponseFormatJsonSchema {
            description: Some(
                "A step-by-step reasoning process for solving math problems".to_string(),
            ),
            name: "math_reasoning".to_string(),
            schema: Some(schema),
            strict: Some(true),
        })
        .input(vec![
            InputMessage {
                role: InputRole::System,
                content: vec![
                    "You are a helpful math tutor. Guide the user through the solution step by step."
                        .into(),
                ],
                status: None,
            },
            InputMessage {
                role: InputRole::User,
                content: vec!["How can I solve 8x + 7 = -23?".into()],
                status: None,
            },
        ])
        .build()?;

    let response = client.responses().create(request).await?;

    for output in response.output {
        if let OutputItem::Message(message) = output {
            for content in message.content {
                if let OutputMessageContent::OutputText(text) = content {
                    println!("Response:\n{}\n", text.text);
                }
            }
        }
    }

    Ok(())
}

/// Structured data extraction example: Extracts specific fields from unstructured text
async fn structured_data_extraction(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    println!("=== Structured Data Extraction Example ===\n");

    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer" },
            "occupation": { "type": "string" },
            "location": { "type": "string" },
            "email": { "type": "string" }
        },
        "required": ["name", "age", "occupation", "email", "location"],
        "additionalProperties": false
    });

    let text = "Hi, I'm Sarah Johnson. I'm 28 years old and I work as a software engineer in San Francisco. You can reach me at sarah.johnson@email.com.";

    let request = CreateResponseArgs::default()
        .model("gpt-4o-2024-08-06")
        .max_output_tokens(256u32)
        .text(ResponseFormatJsonSchema {
            description: Some("Extract structured information from text".to_string()),
            name: "person_info".to_string(),
            schema: Some(schema),
            strict: Some(true),
        })
        .input(vec![
            InputMessage {
                role: InputRole::System,
                content: vec!["Extract the following information from the user's text: name, age, occupation, location, and email. If any information is not present, omit that field.".into()],
                status: None,
            },
            InputMessage {
                role: InputRole::User,
                content: vec![text.into()],
                status: None,
            },
        ])
        .build()?;

    let response = client.responses().create(request).await?;

    println!("Input text: {}\n", text);
    for output in response.output {
        if let OutputItem::Message(message) = output {
            for content in message.content {
                if let OutputMessageContent::OutputText(text) = content {
                    println!("Extracted data:\n{}\n", text.text);
                }
            }
        }
    }

    Ok(())
}

/// UI generation example: Generates UI component code based on description
async fn ui_generation(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    println!("=== UI Generation Example ===\n");

    let schema = json!({
        "type": "object",
        "properties": {
            "type": {
            "type": "string",
            "description": "The type of the UI component",
            "enum": ["div", "button", "header", "section", "field", "form"]
            },
            "label": {
            "type": "string",
            "description": "The label of the UI component, used for buttons or form fields"
            },
            "children": {
            "type": "array",
            "description": "Nested UI components",
            "items": {"$ref": "#"}
            },
            "attributes": {
            "type": "array",
            "description": "Arbitrary attributes for the UI component, suitable for any element",
            "items": {
                "type": "object",
                "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the attribute, for example onClick or className"
                },
                "value": {
                    "type": "string",
                    "description": "The value of the attribute"
                }
                },
                "required": ["name", "value"],
                "additionalProperties": false
            }
            }
        },
        "required": ["type", "label", "children", "attributes"],
        "additionalProperties": false

    });

    let request = CreateResponseArgs::default()
        .model("gpt-4o-2024-08-06")
        .max_output_tokens(1024u32)
        .text(ResponseFormatJsonSchema {
            description: Some("Generate HTML and CSS code for UI components".to_string()),
            name: "ui_component".to_string(),
            schema: Some(schema),
            strict: Some(true),
        })
        .input(vec![
            InputMessage {
                role: InputRole::System,
                content: vec!["You are a UI designer. Generate clean, modern HTML and CSS code for the requested UI component. The HTML should be semantic and accessible, and the CSS should be well-organized.".into()],
                status: None,
            },
            InputMessage {
                role: InputRole::User,
                content: vec!["Create a login form with email and password fields, a submit button, and a link to sign up. Use a modern, clean design with a light color scheme.".into()],
                status: None,
            },
        ])
        .build()?;

    let response = client.responses().create(request).await?;

    for output in response.output {
        if let OutputItem::Message(message) = output {
            for content in message.content {
                if let OutputMessageContent::OutputText(text) = content {
                    println!("Generated UI Component:\n{}\n", text.text);
                }
            }
        }
    }

    Ok(())
}

/// Moderation example: Analyzes content for policy violations with structured output
async fn moderation(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    println!("=== Moderation Example ===\n");

    let schema = json!({
      "type": "object",
      "properties": {
        "is_violating": {
          "type": "boolean",
          "description": "Indicates if the content is violating guidelines"
        },
        "category": {
          "type": ["string", "null"],
          "description": "Type of violation, if the content is violating guidelines. Null otherwise.",
          "enum": ["violence", "sexual", "self_harm"]
        },
        "explanation_if_violating": {
          "type": ["string", "null"],
          "description": "Explanation of why the content is violating"
        }
      },
      "required": ["is_violating", "category", "explanation_if_violating"],
      "additionalProperties": false
    });

    let content_to_moderate = "How do I prepare for a job interview?";

    let request = CreateResponseArgs::default()
        .model("gpt-4o-2024-08-06")
        .max_output_tokens(256u32)
        .text(ResponseFormatJsonSchema {
            description: Some("Analyze content for policy violations and provide structured moderation results".to_string()),
            name: "moderation_result".to_string(),
            schema: Some(schema),
            strict: Some(true),
        })
        .input(vec![
            InputMessage {
                role: InputRole::System,
                content: vec!["Determine if the user input violates specific guidelines and explain if they do.".into()],
                status: None,
            },
            InputMessage {
                role: InputRole::User,
                content: vec![content_to_moderate.into()],
                status: None,
            },
        ])
        .build()?;

    let response = client.responses().create(request).await?;

    for output in response.output {
        if let OutputItem::Message(message) = output {
            for content in message.content {
                if let OutputMessageContent::OutputText(text) = content {
                    println!("Content analyzed: {}\n", content_to_moderate);
                    println!("Moderation result:\n{}\n", text.text);
                }
            }
        }
    }

    Ok(())
}

/// Streaming structured output example: Extract entities from text with streaming
async fn streaming_structured_output(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    println!("=== Streaming Structured Output Example ===\n");

    let schema = json!({
        "type": "object",
        "properties": {
            "attributes": {
                "type": "array",
                "items": { "type": "string" }
            },
            "colors": {
                "type": "array",
                "items": { "type": "string" }
            },
            "animals": {
                "type": "array",
                "items": { "type": "string" }
            }
        },
        "required": ["attributes", "colors", "animals"],
        "additionalProperties": false
    });

    let request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .stream(true)
        .text(ResponseFormatJsonSchema {
            description: Some("Extract entities from the input text".to_string()),
            name: "entities".to_string(),
            schema: Some(schema),
            strict: Some(true),
        })
        .input(vec![
            InputMessage {
                role: InputRole::System,
                content: vec!["Extract entities from the input text".into()],
                status: None,
            },
            InputMessage {
                role: InputRole::User,
                content: vec![
                    "The quick brown fox jumps over the lazy dog with piercing blue eyes".into(),
                ],
                status: None,
            },
        ])
        .build()?;

    let mut stream = client.responses().create_stream(request).await?;
    let mut lock = stdout().lock();
    let mut final_response = None;

    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => match event {
                ResponseStreamEvent::ResponseRefusalDelta(delta) => {
                    write!(lock, "{}", delta.delta)?;
                    lock.flush()?;
                }
                ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                    write!(lock, "{}", delta.delta)?;
                    lock.flush()?;
                }
                ResponseStreamEvent::ResponseError(error) => {
                    writeln!(lock, "\nError: {}", error.message)?;
                    if let Some(code) = &error.code {
                        writeln!(lock, "Code: {}", code)?;
                    }
                    if let Some(param) = &error.param {
                        writeln!(lock, "Param: {}", param)?;
                    }
                }
                ResponseStreamEvent::ResponseCompleted(completed) => {
                    writeln!(lock, "\nCompleted")?;
                    final_response = Some(completed.response);
                    break;
                }
                _ => {
                    writeln!(lock, "\n{}: skipping\n", event.event_type())?;
                }
            },
            Err(e) => {
                writeln!(lock, "\nStream error: {:#?}", e)?;
            }
        }
    }

    if let Some(response) = final_response {
        writeln!(lock, "\nFinal response:")?;
        for output in response.output {
            if let OutputItem::Message(message) = output {
                for content in message.content {
                    if let OutputMessageContent::OutputText(text) = content {
                        writeln!(lock, "{}", text.text)?;
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "responses-structured-outputs")]
#[command(about = "Examples of structured outputs using the Responses API", long_about = None)]
struct Cli {
    /// Which example to run
    #[arg(value_enum)]
    example: Example,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Example {
    /// Chain of thought: Step-by-step reasoning for math problems
    ChainOfThought,
    /// Structured data extraction: Extract fields from unstructured text
    DataExtraction,
    /// UI generation: Generate HTML and CSS for UI components
    UiGeneration,
    /// Moderation: Analyze content for policy violations
    Moderation,
    /// Streaming structured output: Extract entities with streaming
    Streaming,
    /// Run all examples
    All,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.example {
        Example::ChainOfThought => {
            chain_of_thought(&client).await?;
        }
        Example::DataExtraction => {
            structured_data_extraction(&client).await?;
        }
        Example::UiGeneration => {
            ui_generation(&client).await?;
        }
        Example::Moderation => {
            moderation(&client).await?;
        }
        Example::Streaming => {
            streaming_structured_output(&client).await?;
        }
        Example::All => {
            chain_of_thought(&client).await?;
            structured_data_extraction(&client).await?;
            ui_generation(&client).await?;
            moderation(&client).await?;
            streaming_structured_output(&client).await?;
        }
    }

    Ok(())
}
