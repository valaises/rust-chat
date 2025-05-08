use std::pin::pin;
use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_context;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use crate::chat_completions::comp_stream::{test_stream_frontend};
use crate::openai;
use crate::shared_state::{Message, SharedState};
use futures::StreamExt;
use crate::chat_completions::comp_chunks_collector::extract_content_from_chunks;

#[derive(Props, PartialEq, Clone)]
pub struct SendButtonProps {
    pub chat_input_value: Signal<String>,
}

#[component]
pub fn SendButton(props: SendButtonProps) -> Element {
    let mut sh = use_context::<SharedState>();
    let mut chat_input_value = props.chat_input_value.clone();

    let button_on_click = move |_| {
        let value = chat_input_value.read().clone();
        if !value.trim().is_empty() {
            // Add user message
            let assistant_message_index;
            {
                let mut messages = sh.messages.write();
                messages.push(
                    Message {
                        role: "user".to_string(),
                        content: value,
                    }
                );

                // Create a new assistant message with empty content
                assistant_message_index = messages.len();
                messages.push(
                    Message {
                        role: "assistant".to_string(),
                        content: "".to_string(),
                    }
                );
            }

            let model = sh.active_model.read().clone().unwrap().id;

            // Convert messages to OMessages for the API
            let omessages = sh.messages.read().iter()
                .filter(|m| m.role == "user" || m.role == "assistant")
                .map(|m| openai::OMessage::new(
                    m.role.clone(),
                    m.content.clone(),
                ))
                .collect::<Vec<_>>();

            // Start streaming the response
            spawn(async move {
                match test_stream_frontend(omessages, model).await {
                    Ok(stream) => {
                        let mut received_chunks = Vec::new();

                        // Process each chunk as it arrives
                        let mut stream = pin!(stream);
                        while let Some(chunk) = stream.next().await {
                            received_chunks.push(chunk);
                            {
                                let mut messages = sh.messages.write();
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
                        let mut messages = sh.messages.write();
                        if let Some(assistant_message) = messages.get_mut(assistant_message_index) {
                            assistant_message.content = format!("Error: {}", e);
                        }
                    }
                }
            });

            // Clear the input after sending
            chat_input_value.set("".to_string());
        }
    };

    rsx! {
        button { class: "send-button",
            onclick: button_on_click,
            "Send"
        }
    }
}
