use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use std::pin::pin;
use futures::StreamExt;

use crate::chat_completions::comp_stream::test_stream_frontend;
use crate::chat_completions::comp_chunks_collector::extract_content_from_chunks;
use crate::openai;
use crate::shared_state::{Message, SharedState};


#[derive(Clone)]
pub struct SendMessageRequest {
    pub content: String,
}

pub async fn message_service(mut rx: UnboundedReceiver<SendMessageRequest>) {
    let sh = use_context::<SharedState>();

    while let Some(request) = rx.next().await {
        send_message_and_stream(sh.clone(), request.content).await;
    }
}

async fn send_message_and_stream(sh: SharedState, message_content: String) {
    if message_content.trim().is_empty() {
        return;
    }

    let mut sh_messages = sh.messages.clone();
    let mut sh_scrolled_to_bottom_element = sh.scrolled_to_bottom_element.clone();

    let assistant_message_index;
    {
        let mut messages = sh_messages.write();
        messages.push(Message {
            role: "user".to_string(),
            content: message_content,
        });
        info!("Adding user message");
        sh_scrolled_to_bottom_element.set(false);

        // Create a new assistant message with empty content
        assistant_message_index = messages.len();
        messages.push(Message {
            role: "assistant".to_string(),
            content: "".to_string(),
        });
        info!("Adding empty assistant message");
    }

    let model = match sh.active_model.read().clone() {
        Some(model) => model.id,
        None => {
            info!("No active model selected");
            return;
        }
    };

    // Convert messages to OMessages for the API
    let omessages = sh_messages.read().iter()
        .filter(|m| m.role == "user" || m.role == "assistant")
        .map(|m| openai::OMessage::new(
            m.role.clone(),
            m.content.clone(),
        ))
        .collect::<Vec<_>>();
    // todo: remove the last empty user message

    info!("{:#?}", omessages);
    info!("Starting streaming");
    
    // Start streaming the response
    match test_stream_frontend(omessages, model).await {
        Ok(stream) => {
            let mut received_chunks = Vec::new();

            // Process each chunk as it arrives
            let mut stream = pin!(stream);
            while let Some(chunk) = stream.next().await {
                received_chunks.push(chunk);
                {
                    let mut messages = sh_messages.write();
                    if let Some(assistant_message) = messages.get_mut(assistant_message_index) {
                        // Update with all chunks received so far
                        assistant_message.content = extract_content_from_chunks(&received_chunks);
                    }
                }
            }
            info!("Stream completed. Received {} chunks in total.", received_chunks.len());
        },
        Err(e) => {
            info!("Failed to start stream: {}", e);
            let mut messages = sh_messages.write();
            if let Some(assistant_message) = messages.get_mut(assistant_message_index) {
                assistant_message.content = format!("Error: {}", e);
            }
        }
    }
}
