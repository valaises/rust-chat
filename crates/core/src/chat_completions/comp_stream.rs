use dioxus::logger::tracing::{error, info};
use dioxus::prelude::server_fn::serde::{Serialize, Deserialize};

use futures::{Stream, StreamExt};
use reqwest::Client;
use crate::chat_completions::comp_chunks_collector::{completion_response_chunk_collector, CollectedChunk};
use crate::openai::OMessage;

use crate::chat_completions::comp_post::{CompletionRestreamerPost};
use crate::globals::{API_KEY, BACKEND_URL};


#[derive(Serialize, Deserialize)]
pub struct CompletionResponseChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<CompletionResponseChunkChoice>,
    pub stream_options: Option<CompletionResponseChunkStreamOptions>,
    pub citations: Option<serde_json::Value>,
    pub tool_res_messages: Option<Vec<OMessage>>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponseChunkChoice {
    pub index: usize,
    pub delta: CompletionResponseChunkChoiceDelta,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponseChunkChoiceDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub refusal: Option<serde_json::Value>,
    pub function_call: Option<serde_json::Value>,
    pub tool_calls: Option<serde_json::Value>,
    pub audio: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionResponseChunkStreamOptions {
    pub include_usage: bool,
}


pub async fn test_stream_frontend(
    messages: Vec<OMessage>,
    model: String,
) -> Result<impl Stream<Item = CollectedChunk>, String> {
    info!("Starting test_stream_frontend with model: {}", model);
    info!("Number of messages: {}", messages.len());

    let post = CompletionRestreamerPost {
        model: model.clone(),
        messages,
        tools: None,
        tool_choice: None,
        stream: true,
        max_tokens: None,
        temperature: None,
        n: None,
        top_p: None,
        top_n: None,
        stop: None,
    };

    // Directly create the HTTP request and get the stream
    let url = format!("{}/chat/completions", BACKEND_URL);
    let client = Client::new();

    let response = match client
        .post(url)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("Accept", "text/event-stream")
        .json(&post)
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            info!("Error sending request: {:?}", e);
            return Err(format!("Server error: {}", e));
        }
    };

    // Create a stream that processes the response body
    let stream = futures::stream::unfold(
        (response.bytes_stream(), String::new(), Vec::new()),
        |(mut bytes_stream, buffer, mut pending_chunks)| async move {
            // Return any pending chunks first
            if !pending_chunks.is_empty() {
                let chunk = pending_chunks.remove(0);
                return Some((Result::<CompletionResponseChunk, String>::Ok(chunk),
                             (bytes_stream, buffer, pending_chunks)));
            }

            let mut current_buffer = buffer;

            // Read the next chunk from the stream
            let bytes = match bytes_stream.next().await {
                Some(Ok(bytes)) => bytes,
                Some(Err(e)) => {
                    error!("Error reading from stream: {:?}", e);
                    return None;
                },
                None => {
                    // Process any remaining data at end of stream
                    process_buffer(&current_buffer, &mut pending_chunks);

                    if !pending_chunks.is_empty() {
                        let chunk = pending_chunks.remove(0);
                        return Some((Ok(chunk), (bytes_stream, String::new(), pending_chunks)));
                    }
                    return None;
                }
            };

            // Append new chunk to existing buffer
            current_buffer.push_str(&String::from_utf8_lossy(&bytes));

            // Extract and process complete chunks
            let mut remaining = current_buffer.clone();
            let mut processed_up_to = 0;

            while let Some(start_idx) = remaining.find("data: ") {
                processed_up_to += start_idx;
                let chunk_start = processed_up_to;

                // Look for the next "data: " marker
                if let Some(next_idx) = remaining[start_idx + 6..].find("data: ") {
                    let chunk_end = processed_up_to + start_idx + 6 + next_idx;
                    let chunk_text = &current_buffer[chunk_start..chunk_end];

                    // Process this complete chunk
                    if let Some(chunk) = parse_chunk(chunk_text) {
                        pending_chunks.push(chunk);
                    }

                    // Move past this chunk
                    processed_up_to = chunk_end;
                    remaining = current_buffer[processed_up_to..].to_string();
                } else {
                    // No more complete chunks
                    break;
                }
            }

            // Keep only the unprocessed part
            current_buffer = if processed_up_to > 0 {
                current_buffer[processed_up_to..].to_string()
            } else {
                current_buffer
            };
            
            // Return a chunk if we have any
            if !pending_chunks.is_empty() {
                let chunk = pending_chunks.remove(0);
                Some((Ok(chunk), (bytes_stream, current_buffer, pending_chunks)))
            } else {
                // No chunks yet, continue with the next iteration
                Some((Err("No chunks yet".to_string()), (bytes_stream, current_buffer, pending_chunks)))
            }
        }
    )
        .filter_map(|result| async move {
            match result {
                Ok(chunk) => Some(chunk),
                Err(_) => None  // Filter out our "No chunks yet" placeholder errors
            }
        });

    // Use the collector to process the chunks into CollectedChunk objects
    let collected_stream = completion_response_chunk_collector(Box::pin(stream));

    Ok(collected_stream)
}

// Helper function to parse a chunk of text into a CompletionResponseChunk
fn parse_chunk(text: &str) -> Option<CompletionResponseChunk> {
    if !text.starts_with("data: ") {
        return None;
    }

    let data = text.trim_start_matches("data: ").trim();
    if data == "[DONE]" {
        return None;
    }

    match serde_json::from_str::<CompletionResponseChunk>(data) {
        Ok(chunk) => Some(chunk),
        Err(e) => {
            info!("Error parsing JSON: {:?}, Data: {}", e, data);
            None
        }
    }
}

// Helper function to process a buffer and extract chunks
fn process_buffer(buffer: &str, chunks: &mut Vec<CompletionResponseChunk>) {
    // Split by "data: " to handle multiple events
    let parts: Vec<&str> = buffer.split("data: ")
        .filter(|s| !s.is_empty())
        .collect();

    for part in parts {
        let data = part.trim();
        if data == "[DONE]" {
            continue;
        }

        match serde_json::from_str::<CompletionResponseChunk>(data) {
            Ok(chunk) => {
                chunks.push(chunk);
            },
            Err(e) => {
                error!("Error parsing JSON part: {:?}, Data: {}", e, data);
            }
        }
    }
}
