use dioxus::prelude::*;
use dioxus::prelude::server_fn::serde::{Serialize, Deserialize};
use serde;

use crate::components::SideBar::SideBarState;


#[derive(Clone, Default)]
pub struct SharedState {
    pub messages: Signal<Vec<Message>>,
    
    pub models: Signal<CompletionModelState>,
    pub active_model: Signal<Option<CompletionModel>>,
    pub empty_space_height: Signal<usize>,
    pub message_container_bottom_element: Signal<Option<Event<MountedData>>>,
    pub scrolled_to_bottom_element: Signal<bool>,
    
    pub side_bar_state: Signal<SideBarState>,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            models: Signal::new(CompletionModelState::default()),
            scrolled_to_bottom_element: Signal::new(true),
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
