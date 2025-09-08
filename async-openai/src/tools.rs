//! This module provides functionality for managing and executing tools in an async OpenAI context.
//! It defines traits and structures for tool management, execution, and streaming.
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    pin::Pin,
    sync::Arc,
};

use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::types::{
    responses::{self, Function, ToolDefinition},
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCallChunk,
    ChatCompletionRequestToolMessage, ChatCompletionTool, ChatCompletionToolType, FunctionCall,
    FunctionObject,
};

/// A trait defining the interface for tools that can be used with the OpenAI API.
/// Tools must implement this trait to be used with the ToolManager.
pub trait Tool: Send + Sync {
    /// The type of arguments that the tool accepts.
    type Args: JsonSchema + for<'a> Deserialize<'a> + Send + Sync;
    /// The type of output that the tool produces.
    type Output: Serialize + Send + Sync;
    /// The type of error that the tool can return.
    type Error: ToString + Send + Sync;

    /// Returns the name of the tool.
    fn name() -> String {
        Self::Args::schema_name().to_string()
    }

    /// Returns an optional description of the tool.
    fn description() -> Option<String> {
        None
    }

    /// Returns an optional boolean indicating whether the tool should be strict about the arguments.
    fn strict() -> Option<bool> {
        None
    }

    /// Returns the tool's definition for chat.
    fn definition_for_chat() -> ChatCompletionTool {
        ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: FunctionObject {
                name: Self::name(),
                description: Self::description(),
                parameters: Some(json!(schema_for!(Self::Args))),
                strict: Self::strict(),
            },
        }
    }

    /// Returns the tool's definition for responses.
    fn definition_for_responses() -> ToolDefinition {
        ToolDefinition::Function(Function {
            name: Self::name(),
            description: Self::description(),
            parameters: json!(schema_for!(Self::Args)),
            strict: Self::strict().unwrap_or(false),
        })
    }

    /// Executes the tool with the given arguments.
    /// Returns a Future that resolves to either the tool's output or an error.
    fn call(
        &self,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;
}

/// A dynamic trait for tools that allows for runtime tool management.
/// This trait provides a way to work with tools without knowing their concrete types at compile time.
pub trait ToolDyn: Send + Sync {
    /// Returns the tool's name.
    fn name(&self) -> String;

    /// Returns the tool's definition for chat.
    fn definition_for_chat(&self) -> ChatCompletionTool;

    /// Returns the tool's definition for responses.
    fn definition_for_responses(&self) -> ToolDefinition;

    /// Executes the tool with the given JSON string arguments.
    /// Returns a Future that resolves to either a JSON string output or an error string.
    fn call(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>>;
}

// Implementation of ToolDyn for any type that implements Tool
impl<T: Tool> ToolDyn for T {
    fn name(&self) -> String {
        T::name()
    }

    fn definition_for_chat(&self) -> ChatCompletionTool {
        T::definition_for_chat()
    }

    fn definition_for_responses(&self) -> ToolDefinition {
        T::definition_for_responses()
    }

    fn call(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
        let future = async move {
            // Special handling for T::Args = () case
            // If the tool doesn't require arguments (T::Args is unit type),
            // we can safely ignore the provided arguments string
            match serde_json::from_str::<T::Args>(&args)
                .or_else(|e| serde_json::from_str::<T::Args>("null").map_err(|_| e))
            {
                Ok(args) => T::call(self, args)
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(|output| {
                        serde_json::to_string(&output)
                            .map_err(|e| format!("Failed to serialize output: {}", e))
                    }),
                Err(e) => Err(format!("Failed to parse arguments: {}", e)),
            }
        };
        Box::pin(future)
    }
}

/// A manager for tools that allows adding, retrieving, and executing tools.
#[derive(Default, Clone)]
pub struct ToolManager {
    /// A map of tool names to their dynamic implementations.
    tools: BTreeMap<String, Arc<dyn ToolDyn>>,
}

impl ToolManager {
    /// Creates a new ToolManager.
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
        }
    }

    /// Adds a new tool to the manager.
    pub fn add_tool<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(T::name(), Arc::new(tool));
    }

    /// Adds a new tool with an Arc to the manager.
    ///
    /// Use this if you want to access this tool after being added to the manager.
    pub fn add_tool_dyn(&mut self, tool: Arc<dyn ToolDyn>) {
        self.tools.insert(tool.name(), tool);
    }

    /// Removes a tool from the manager.
    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    /// Returns the definitions of all tools for chat in the manager.
    pub fn get_tools_for_chat(&self) -> Vec<ChatCompletionTool> {
        self.tools
            .values()
            .map(|tool| tool.definition_for_chat())
            .collect()
    }

    /// Returns the definitions of all tools for responses in the manager.
    pub fn get_tools_for_responses(&self) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .map(|tool| tool.definition_for_responses())
            .collect()
    }

    /// Executes multiple tool calls concurrently and returns their results for chat.
    pub async fn call_for_chat(
        &self,
        calls: impl IntoIterator<Item = ChatCompletionMessageToolCall>,
    ) -> Vec<ChatCompletionRequestToolMessage> {
        let mut handles = Vec::new();
        let mut outputs = Vec::new();

        // Spawn a task for each tool call
        for call in calls {
            if let Some(tool) = self.tools.get(&call.function.name).cloned() {
                let handle = tokio::spawn(async move { tool.call(call.function.arguments).await });
                handles.push((call.id, handle));
            } else {
                outputs.push(ChatCompletionRequestToolMessage {
                    content: "Tool call failed: tool not found".into(),
                    tool_call_id: call.id,
                });
            }
        }

        // Collect results from all spawned tasks
        for (id, handle) in handles {
            let output = match handle.await {
                Ok(Ok(output)) => output,
                Ok(Err(e)) => {
                    format!("Tool call failed: {}", e)
                }
                Err(_) => "Tool call failed: runtime error".to_string(),
            };
            outputs.push(ChatCompletionRequestToolMessage {
                content: output.into(),
                tool_call_id: id,
            });
        }
        outputs
    }

    /// Executes multiple tool calls concurrently and returns their results for responses.
    pub async fn call_for_responses(
        &self,
        calls: impl IntoIterator<Item = responses::FunctionCall>,
    ) -> Vec<responses::InputItem> {
        let mut handles = Vec::new();
        let mut outputs = Vec::new();

        // Spawn a task for each tool call
        for call in calls {
            if let Some(tool) = self.tools.get(&call.name).cloned() {
                let handle = tokio::spawn(async move { tool.call(call.arguments).await });
                handles.push((call.call_id, handle));
            } else {
                outputs.push(responses::InputItem::Custom(json!({
                    "type": "function_call_output",
                    "call_id": call.call_id,
                    "output": "Tool call failed: tool not found",
                })));
            }
        }

        // Collect results from all spawned tasks
        for (id, handle) in handles {
            let output = match handle.await {
                Ok(Ok(output)) => output,
                Ok(Err(e)) => {
                    format!("Tool call failed: {}", e)
                }
                Err(_) => "Tool call failed: runtime error".to_string(),
            };
            outputs.push(responses::InputItem::Custom(json!({
                "type": "function_call_output",
                "call_id": id,
                "output": output,
            })));
        }
        outputs
    }
}

/// A manager for handling streaming tool calls.
/// This structure helps manage and merge tool call chunks that arrive in a streaming fashion.
#[derive(Default, Clone, Debug)]
pub struct ToolCallStreamManager(HashMap<u32, ChatCompletionMessageToolCall>);

impl ToolCallStreamManager {
    /// Creates a new empty ToolCallStreamManager.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Processes a single streaming tool call chunk and merges it with existing data.
    pub fn process_chunk(&mut self, chunk: ChatCompletionMessageToolCallChunk) {
        let tool_call =
            self.0
                .entry(chunk.index)
                .or_insert_with(|| ChatCompletionMessageToolCall {
                    id: "".to_string(),
                    r#type: ChatCompletionToolType::Function,
                    function: FunctionCall {
                        name: "".to_string(),
                        arguments: "".to_string(),
                    },
                });
        if let Some(id) = chunk.id {
            tool_call.id = id;
        }
        if let Some(function) = chunk.function {
            if let Some(name) = function.name {
                tool_call.function.name = name;
            }
            if let Some(arguments) = function.arguments {
                tool_call.function.arguments.push_str(&arguments);
            }
        }
    }

    /// Processes multiple streaming tool call chunks and merges them with existing data.
    pub fn process_chunks(
        &mut self,
        chunks: impl IntoIterator<Item = ChatCompletionMessageToolCallChunk>,
    ) {
        for chunk in chunks {
            self.process_chunk(chunk);
        }
    }

    /// Returns all completed tool calls as a vector.
    pub fn finish_stream(self) -> Vec<ChatCompletionMessageToolCall> {
        self.0
            .into_values()
            .filter(|tool_call| {
                let is_complete = !tool_call.id.is_empty() && !tool_call.function.name.is_empty();
                if !is_complete {
                    tracing::error!("Tool call is not complete: {:?}", tool_call);
                }
                is_complete
            })
            .collect()
    }
}
