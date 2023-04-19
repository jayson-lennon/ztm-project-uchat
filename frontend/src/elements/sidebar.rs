#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};

pub fn use_sidebar(cx: &ScopeState) -> &UseAtomRef<SidebarManager> {
    use_atom_ref(cx, crate::app::SIDEBAR)
}

#[derive(Default)]
pub struct SidebarManager {
    is_open: bool,
}

impl SidebarManager {
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

pub fn Sidebar(cx: Scope) -> Element {
    let sidebar = use_sidebar(cx);
    let router = use_router(cx);
    let local_profile = use_local_profile(cx);

    let sidebar_width = if sidebar.read().is_open() {
        "w-[var(--sidebar-width)]"
    } else {
        "w-0"
    };

    let overlay_class = if sidebar.read().is_open() {
        "w-full opacity-80"
    } else {
        "w-0 opacity-0"
    };

    let Overlay = rsx! {
        div {
            class: "fixed top-0 left-0 h-full navbar-bg-color transition z-[99] {overlay_class}",
            onclick: move |_| sidebar.write().close(),
        }
    };

    cx.render(rsx! {
        Overlay,
        div {
            class: "{sidebar_width} z-[100] fixed top-0 left-0 h-full
            overflow-x-hidden
            flex flex-col
            navbar-bg-color transition-[width] duration-300",
            a {
                class: "sidebar-navlink border-t",
                onclick: move |_| {
                    sidebar.write().close();
                    router.navigate_to(page::PROFILE_EDIT);
                },
                "Edit Profile"
            }
            a {
                class: "sidebar-navlink",
                onclick: move |_| {
                    use chrono::Utc;
                    use uchat_domain::ids::SessionId;
                    crate::util::cookie::set_session("".to_string(), SessionId::new(), Utc::now());
                    local_profile.write().user_id = None;
                    local_profile.write().image = None;
                    sidebar.write().close();
                    router.navigate_to(page::ACCOUNT_LOGIN);
                },
                "Logout"
            }
        }
    })
}
