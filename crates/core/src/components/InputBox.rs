use dioxus::prelude::*;
use crate::components::ModelSelector::ModelSelector;
use crate::components::SendButton::SendButton;


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
