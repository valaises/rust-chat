pub mod shared_state;
pub mod models;
mod components;
mod globals;
mod openai;
mod messages;
mod image_utils;
mod chat_completions;

use dioxus::prelude::*;
use crate::components::ChatBody::ChatBody;
use crate::components::InputBox::InputBox;
use crate::models::model_polling_service;
use crate::shared_state::SharedState;


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

