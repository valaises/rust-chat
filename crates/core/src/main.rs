pub mod shared_state;
pub mod models;
mod components;
mod globals;
mod openai;
mod messages;
mod image_utils;
mod chat_completions;
use web_sys::{window, ScrollToOptions};
use dioxus::logger::tracing::info;
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
    let mut sh = use_context::<SharedState>();
    let mut empty_space_height = sh.empty_space_height.clone();

    let mut recent_messages: Signal<Option<Event<MountedData>>> = use_signal(|| None);
    let mut message_container_el: Signal<Option<Event<MountedData>>> = use_signal(|| None);

    let messages = sh.messages.read();
    
    use_effect(move || {
        let _messages = (sh.messages)();

        spawn(async move {

            if let Some(recent_messages_el) = recent_messages.cloned() {
                if let Some(window) = web_sys::window() {
                    let viewport_height = match window.inner_height() {
                        Ok(height) => height.as_f64().unwrap_or(0.0),
                        Err(_) => 0.0,
                    };
                    let rect = recent_messages_el.data.get_client_rect().await.unwrap();
                    let height = rect.height();

                    let needed_space = viewport_height - height;
                    if needed_space > 0.0 {
                        if empty_space_height.read().clone() != needed_space as usize {
                            empty_space_height.set(needed_space as usize);
                        }
                    }
                }
            }
        });
    });
    
    use_effect(move || {
        let _empty_space_height = empty_space_height();
        spawn(async move {
            
            if !sh.scrolled_to_bottom_element.read().clone() {
                let window = web_sys::window().unwrap();
                let options = ScrollToOptions::new();
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                options.set_top(f64::MAX / 2.0); // Use a very large but safe number
                window.scroll_with_scroll_to_options(&options);
                sh.scrolled_to_bottom_element.set(true);
            }
            
        });
    });

    rsx! {
        div { class: "messages-container",
            onmounted: move |cx: Event<MountedData>| {
              message_container_el.set(Some(cx));  
            },
            div { class: "earlier-messages",
                {
                    messages.iter().enumerate()
                        .filter(|(m_idx, _)| *m_idx < messages.len().saturating_sub(2))
                        .map(|(m_idx, message)| {
                            rsx! {
                                MessageComponent {
                                    message_idx: m_idx,
                                    message: message.clone(),
                                    total_idxs: messages.len(),
                                    empty_space_height: empty_space_height.clone(),
                                }
                            }
                        })
                }
            }
            div { class: "recent-messages",
                onmounted: move |cx: Event<MountedData>| {
                    recent_messages.set(Some(cx));
                },
                {
                    messages.iter().enumerate()
                        .filter(|(m_idx, _)| *m_idx >= messages.len().saturating_sub(2))
                        .map(|(m_idx, message)| {
                            rsx! {
                                MessageComponent {
                                    message_idx: m_idx,
                                    message: message.clone(),
                                    total_idxs: messages.len(),
                                    empty_space_height: empty_space_height.clone(),
                                }
                            }
                        })
                }
            }
            div {
                class: "messages-container-empty-space",
                style: "height: {empty_space_height}px"
            }
            div {
                onmounted: move |cx: Event<MountedData>| {
                  sh.message_container_bottom_element.set(Some(cx));  
                },
                class: "message-container-bottom-element"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct MessageProps {
    pub message_idx: usize,
    pub message: Message,
    pub total_idxs: usize,
    pub empty_space_height: Signal<usize>,
}

#[component]
pub fn MessageComponent(props: MessageProps) -> Element {
    let message = props.message.clone();
    
    let role = message.role.clone();
    let content = message.content.clone();
    
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
