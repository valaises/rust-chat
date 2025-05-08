use futures::stream::{Stream, StreamExt};
use dioxus::prelude::server_fn::serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::chat_completions::comp_stream::CompletionResponseChunk;
use crate::openai::OMessage;


pub fn extract_content_from_chunks(chunks: &Vec<CollectedChunk>) -> String {
    chunks.iter()
        .filter_map(|chunk| {
            match chunk {
                CollectedChunk::ContentDelta { content } => Some(content.clone()),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CollectedChunk {
    ContentDelta { content: String },
    ToolCall { content: ToolCallContent },
    ToolResMessages { content: Vec<OMessage> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallContent {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub function: ToolCallFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // todo: move to openai.rs
pub struct ToolCallFunction {
    pub name: Option<String>,
    pub arguments: String,
}

pub fn completion_response_chunk_collector<S>(
    mut chunks: S
) -> impl Stream<Item = CollectedChunk>
where
    S: Stream<Item = CompletionResponseChunk> + Unpin,
{
    let mut current_tool_call: Option<ToolCallContent> = None;

    async_stream::stream! {
        while let Some(chunk) = chunks.next().await {
            // Handle tool_res_messages
            if chunk.object == "tool_res_messages" {
                if let Some(tool_res_messages) = chunk.tool_res_messages {
                    yield CollectedChunk::ToolResMessages {
                        content: tool_res_messages,
                    };
                }
                continue;
            }
            
            if chunk.choices.is_empty() {
                continue;
            }
            
            for choice in chunk.choices {
                let delta = choice.delta;
                
                // Handle content deltas immediately
                if let Some(content) = delta.content {
                    if !content.is_empty() {
                        yield CollectedChunk::ContentDelta {
                            content,
                        };
                    }
                }
                
                // Handle tool calls
                if let Some(tool_calls_value) = delta.tool_calls {
                    if let Ok(tool_calls) = serde_json::from_value::<Vec<Value>>(tool_calls_value) {
                        for tool_call_value in tool_calls {
                            if let Ok(tool_call) = serde_json::from_value::<ToolCallPart>(tool_call_value) {
                                // New tool call started
                                if tool_call.index.is_some() && current_tool_call.is_none() {
                                    let function_name = tool_call.function.as_ref().and_then(|f| f.name.clone());
                                    
                                    current_tool_call = Some(ToolCallContent {
                                        id: tool_call.id.clone(),
                                        tool_type: tool_call.tool_type.clone(),
                                        function: ToolCallFunction {
                                            name: function_name,
                                            arguments: String::new(),
                                        },
                                    });
                                }
                                
                                if let Some(ref mut current) = current_tool_call {
                                    // Append arguments if present
                                    if let Some(function) = &tool_call.function {
                                        if let Some(arguments) = &function.arguments {
                                            current.function.arguments.push_str(arguments);
                                        }
                                    }
                                    
                                    // Update other fields if present
                                    if let Some(id) = &tool_call.id {
                                        current.id = Some(id.clone());
                                    }
                                    if let Some(tool_type) = &tool_call.tool_type {
                                        current.tool_type = Some(tool_type.clone());
                                    }
                                    if let Some(function) = &tool_call.function {
                                        if let Some(name) = &function.name {
                                            current.function.name = Some(name.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Yield tool call when complete
                if choice.finish_reason.as_deref() == Some("tool_calls") && current_tool_call.is_some() {
                    if let Some(tool_call) = current_tool_call.take() {
                        yield CollectedChunk::ToolCall {
                            content: tool_call,
                        };
                    }
                }
            }
        }
    }
}

// Helper struct to parse tool call parts from JSON
#[derive(Debug, Deserialize)]
struct ToolCallPart {
    id: Option<String>,
    index: Option<usize>,
    #[serde(rename = "type")]
    tool_type: Option<String>,
    function: Option<ToolCallFunctionPart>,
}

// Helper struct to parse tool call function parts
#[derive(Debug, Deserialize)]
struct ToolCallFunctionPart {
    name: Option<String>,
    arguments: Option<String>,
}
