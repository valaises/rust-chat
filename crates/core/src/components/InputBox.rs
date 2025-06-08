use dioxus::prelude::*;

use crate::components::ModelSelector::ModelSelector;
use crate::components::SendButton::SendButton;
use crate::shared_state::SharedState;


const INPUT_BOX_CSS: Asset = asset!("assets/input-box.css");


#[component]
pub fn InputBox() -> Element {
    let sh = use_context::<SharedState>();

    let mut chat_input_value = use_signal(|| "".to_string());

    let textarea_box_sidebar_classname = if sh.side_bar_state.read().is_expanded {
        ""
    } else {
        "sidebar-collapsed"
    };

    rsx! {
        document::Link { rel: "stylesheet", href: INPUT_BOX_CSS}
        
        div {class: "textarea-box {textarea_box_sidebar_classname}",
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
