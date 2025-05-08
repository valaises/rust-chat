pub mod shared_state;
pub mod models;
mod components;
mod globals;
mod openai;
mod messages;
mod image_utils;
mod chat_completions;

use dioxus::prelude::*;
use crate::components::ModelSelector::ModelSelector;
use crate::models::model_polling_service;
use crate::shared_state::{Message, SharedState};
use crate::components::SendButton::SendButton;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");


fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let _sh = use_context_provider(|| SharedState::new());
    use_coroutine(model_polling_service);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app",
            h1 { "Chat Application" }
            PrimaryChat {}
        }
    }
}

#[component]
fn PrimaryChat() -> Element {
    rsx! {
        ChatBody {}
        InputBox {}
    }
}

#[component]
pub fn ChatBody() -> Element {
    let sh = use_context::<SharedState>();

    let messages = sh.messages.read();

    rsx! {
        div { class: "messages-container",
            {
                messages.iter().enumerate().map(|(m_idx, message)| {
                    rsx! {
                        MessageComponent {
                            message_idx: m_idx,
                            message: message.clone(),
                        }
                    }
                })
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct MessageProps {
    pub message_idx: usize,
    pub message: Message,
}

#[component]
pub fn MessageComponent(props: MessageProps) -> Element {
    let role = props.message.role.clone();
    let content = props.message.content.clone();
    rsx! {
        div { class: "message-container",
            {format!("<{role}>")}
            div { dangerous_inner_html: comrak::markdown_to_html(&content, &comrak::Options::default()) }
        }
    }
}

#[component]
pub fn InputBox() -> Element {
    let mut chat_input_value = use_signal(|| "".to_string());

    rsx! {
        div { class: "input-box-container",
            div {class: "textarea-wrapper",
                textarea {
                    id: "chat-input",
                    value: "{chat_input_value}",  // Use string interpolation
                    oninput: move |evt| chat_input_value.set(evt.data.value()),
                    placeholder: "Type your message..."
                }
                div {class: "sub-input-box-container",
                    ModelSelector {}
                    SendButton {
                        chat_input_value: chat_input_value.clone()
                    }
                }
            }
        }
    }
}
