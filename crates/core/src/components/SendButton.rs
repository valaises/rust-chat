use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

use crate::chat_service::SendMessageRequest;
use crate::shared_state::{ActiveView, SharedState};


#[derive(Props, PartialEq, Clone)]
pub struct SendButtonProps {
    pub chat_input_value: Signal<String>,
    #[props(default = false)]
    pub for_new_chat: bool,
}

#[component]
pub fn SendButton(props: SendButtonProps) -> Element {
    let mut sh = use_context::<SharedState>();
    
    let mut chat_input_value = props.chat_input_value.clone();
    let message_service = use_coroutine_handle::<SendMessageRequest>();

    let button_on_click = move |_| {
        let value = chat_input_value.read().clone();

        if value.trim().is_empty() {
            return;
        }
        chat_input_value.set("".to_string());

        message_service.send(SendMessageRequest {
            content: value,
        });

        sh.active_view.set(ActiveView::Chat);
    };

    rsx! {
        button { class: "send-button",
            onclick: button_on_click,
            "Send"
        }
    }
}
