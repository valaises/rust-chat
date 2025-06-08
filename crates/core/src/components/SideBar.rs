use dioxus::prelude::*;
use crate::shared_state::SharedState;


const SIDEBAR_CSS: Asset = asset!("assets/sidebar.css");


#[derive(Clone)]
pub struct SideBarState {
    pub is_expanded: bool,
}

impl Default for SideBarState {
    fn default() -> Self {
        Self {
            is_expanded: true,
        }
    }
}

#[component]
pub fn SideBar() -> Element {
    let sh = use_context::<SharedState>();
    
    let mut sidebar_state = sh.side_bar_state.clone();
    
    let sidebar_toggle_style = if sidebar_state.read().is_expanded {
        "expanded"
    } else {
        "collapsed"
    };
    
    let sidebar_toggle_btn_on_click = move |_| {
        let is_expanded = sidebar_state.read().is_expanded;
        sidebar_state.write().is_expanded = !is_expanded;
    };
    
    rsx! {
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS}
        div {class: "sidebar {sidebar_toggle_style}",
            button {class: "sidebar-toggle-btn {sidebar_toggle_style}",
                onclick: sidebar_toggle_btn_on_click,
            },
            div {class: "sidebar-content"},
            div {class: "sidebar-user-profile"},
        }
    }
}
