use dioxus::prelude::*;

use crate::shared_state::SharedState;


#[component]
pub fn ModelSelector() -> Element {
    let mut sh = use_context::<SharedState>();

    // Read values from state
    let active_model = sh.active_model.read();
    let models_state = sh.models.read();
    let models = &models_state.models;
    let is_loading = models_state.is_loading;
    // let error = models_state.error.clone();
    let model_for_handler = models.clone();
    
    // Function to handle model selection
    let handle_model_selection = move |event: Event<FormData>| {
        let model_id = event.data.value();

        if let Some(selected_model) = model_for_handler.iter().find(|m| m.id == model_id) {
            sh.active_model.write().replace(selected_model.clone());
        }
    };

    rsx! {
        div { class: "model-selector-container",
            // Model dropdown
            div { class: "model-dropdown-container",
                label { for: "model-select", "" }
                select {
                    id: "model-select",
                    class: "model-dropdown",
                    onchange: handle_model_selection,
                    disabled: models.is_empty() || is_loading,
                    
                    option { value: "", disabled: true, selected: active_model.is_none(), "Choose a model" }
                    
                    {models.iter().map(|model| {
                        let is_selected = active_model.as_ref().map_or(false, |m| m.id == model.id);
                        rsx! {
                            option { 
                                key: "{model.id}", 
                                value: "{model.id}",
                                selected: is_selected,
                                "{model.id}" 
                            }
                        }
                    })}
                }
            }
        }
    }
}
