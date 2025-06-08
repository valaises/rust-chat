use dioxus::prelude::*;
use crate::shared_state::SharedState;


const SIDEBAR_CSS: Asset = asset!("assets/sidebar.css");
const ICON_LAYOUT_SIDEBAR: Asset = asset!("assets/icons/layout-sidebar.svg");
const ICON_LAYOUT_SIDEBAR_LEFT_COLLAPSE: Asset = asset!("assets/icons/layout-sidebar-left-collapse.svg");
const ICON_LAYOUT_SIDEBAR_LEFT_EXPAND: Asset = asset!("assets/icons/layout-sidebar-left-expand.svg");
const ICON_CIRCLE_LETTER_V: Asset = asset!("assets/icons/circle-letter-v.svg");


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
    let mut is_hovered = use_signal(|| false);

    let sidebar_toggle_style = if sidebar_state.read().is_expanded {
        "expanded"
    } else {
        "collapsed"
    };

    // Icon logic: show what will happen on click when hovered
    let current_icon = match (sidebar_state.read().is_expanded, *is_hovered.read()) {
        (true, false) => ICON_LAYOUT_SIDEBAR,                   // Expanded, no hover: neutral icon
        (true, true) => ICON_LAYOUT_SIDEBAR_LEFT_COLLAPSE,      // Expanded + hover: show collapse action
        (false, false) => ICON_LAYOUT_SIDEBAR,                  // Collapsed, no hover: neutral icon
        (false, true) => ICON_LAYOUT_SIDEBAR_LEFT_EXPAND,       // Collapsed + hover: show expand action
    };

    let sidebar_toggle_btn_on_click = move |_| {
        let is_expanded = sidebar_state.read().is_expanded;
        sidebar_state.write().is_expanded = !is_expanded;
    };
    
    rsx! {
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS}
        
        div {class: "sidebar {sidebar_toggle_style}",
            button {
                class: "sidebar-toggle-btn {sidebar_toggle_style}",
                onclick: sidebar_toggle_btn_on_click,
                onmouseover: move |_| {
                    is_hovered.set(true);
                },
                onmouseout: move |_| {
                    is_hovered.set(false);
                },
                img {
                    src: current_icon,
                    alt: "Toggle layout sidebar",
                    width: "24",
                    height: "24",
                },
            }
            div {class: "sidebar-content"}
            button {
                class: "sidebar-user-profile-btn",
                img {
                    src: ICON_CIRCLE_LETTER_V,
                    alt: "User Profile",
                    width: "24",
                    height: "24",
                }
            }
        }
    }
}
