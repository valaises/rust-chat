use dioxus::prelude::*;

use crate::components::ModelSelector::ModelSelector;
use crate::components::SendButton::SendButton;
use crate::shared_state::SharedState;


const INPUT_BOX_CSS: Asset = asset!("assets/input-box.css");



#[derive(Props, PartialEq, Clone)]
pub struct InputBoxProps {
    #[props(default = false)]
    pub for_new_chat: bool,
}

#[component]
pub fn InputBox(props: InputBoxProps) -> Element {
    let sh = use_context::<SharedState>();

    let mut chat_input_value = use_signal(|| "".to_string());

    let textarea_box_sidebar_classname = if sh.side_bar_state.read().is_expanded {
        ""
    } else {
        "sidebar-collapsed"
    };
    
    let (textarea_box_class, textarea_wrapper_class, textarea_placeholder) = if props.for_new_chat {
        ("textarea-box-new-chat", "textarea-wrapper-new-chat", "How can I help you?")
    } else {
        ("textarea-box", "textarea-wrapper", "Type your message...")
    };

    rsx! {
        document::Link { rel: "stylesheet", href: INPUT_BOX_CSS}
        
        div {class: "{textarea_box_class} {textarea_box_sidebar_classname}",
            div {class: "{textarea_wrapper_class}",
                textarea {
                    id: "chat-input",
                    value: "{chat_input_value}",
                    oninput: move |evt| chat_input_value.set(evt.data.value()),
                    placeholder: "{textarea_placeholder}"
                }
                div {class: "sub-input-box-container",
                    ModelSelector {}
                    SendButton {
                        chat_input_value: chat_input_value.clone(),
                        for_new_chat: props.for_new_chat,
                    }
                }
            }
        }
    }
}
