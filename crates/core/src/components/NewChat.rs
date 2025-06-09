use dioxus::prelude::*;
use crate::components::InputBox::InputBox;
use crate::shared_state::{ActiveView, SharedState};

const NEW_CHAT_CSS: Asset = asset!("assets/new-chat.css");


#[component]
pub fn NewChat() -> Element {
    let sh = use_context::<SharedState>();
    let is_hidden = *sh.active_view.read() != ActiveView::NewChat;
    if is_hidden {
        return rsx! { div { style: "display: none;" } };
    }

    let new_chat_box_sidebar_classname = if sh.side_bar_state.read().is_expanded {
        ""
    } else {
        "sidebar-collapsed"
    };
    
    rsx! {
        document::Link { rel: "stylesheet", href: NEW_CHAT_CSS}

        div { class: "new-chat-box {new_chat_box_sidebar_classname}",
            h1 { class: "new-chat-greeting {new_chat_box_sidebar_classname}",
                "Good Afternoon!"
            }
            div {
                InputBox {
                    for_new_chat: true,
                }
            }
        }
    }
}
