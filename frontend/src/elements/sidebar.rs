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

    pub fn is_open(&mut self) -> bool {
        self.is_open
    }
}

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {"sidebar"})
}
