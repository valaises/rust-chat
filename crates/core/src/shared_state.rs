use dioxus::prelude::*;
use dioxus::prelude::server_fn::serde::{Serialize, Deserialize};
use serde;


#[derive(Clone, Default)]
pub struct SharedState {
    pub messages: Signal<Vec<Message>>,
    
    pub models: Signal<CompletionModelState>,
    pub active_model: Signal<Option<CompletionModel>>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            models: Signal::new(CompletionModelState::default()),
            // messages: Signal::new(vec![
            //     Message {
            //         role: "user".to_string(),
            //         content: "Hello".to_string(),
            //     },
            //     Message {
            //         role: "assistant".to_string(),
            //         content: "# Header \nHello! **how** can I help you?\n\n`print()`\n\n$$x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}$$".to_string(),
            //     },
            // ]),
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct CompletionModelState {
    pub is_loading: bool,
    pub error: Option<String>,
    pub models: Vec<CompletionModel>,
}

impl Default for CompletionModelState {
    fn default() -> Self {
        Self {
            is_loading: false,
            error: None,
            models: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompletionModel {
    pub id: String,
    pub object: String,
    pub created: f32,
    pub owned_by: String,
}


#[derive(serde::Deserialize)]
pub struct CompletionModelResponse {
    pub object: String,
    pub data: Vec<CompletionModel>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub content: String,
    pub role: String,
}
