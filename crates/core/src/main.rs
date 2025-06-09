pub mod shared_state;
pub mod models;
mod components;
mod globals;
mod openai;
mod messages;
mod image_utils;
mod chat_completions;
pub mod chat_service;

use dioxus::prelude::*;
use crate::chat_service::message_service;
use crate::components::ChatBody::ChatBody;
use crate::components::InputBox::InputBox;
use crate::components::NewChat::NewChat;
use crate::components::SideBar::SideBar;
use crate::models::model_polling_service;
use crate::shared_state::{ActiveView, SharedState};


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");


fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let _sh = use_context_provider(|| SharedState::new());
    use_coroutine(model_polling_service);
    use_coroutine(message_service);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app",
            SideBar {},
            MainContent {},
            NewChat {},
        }
    }
}

#[component]
fn MainContent() -> Element {
    let sh = use_context::<SharedState>();
    
    let is_hidden = *sh.active_view.read() != ActiveView::Chat;
    if is_hidden {
        return rsx! { div { style: "display: none;" } };
    }

    let main_content_sidebar_classname = if sh.side_bar_state.read().is_expanded {
        ""
    } else {
        "sidebar-collapsed"
    };


    rsx! {
        div {class: "main-content {main_content_sidebar_classname}",
            PrimaryChat {},
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
