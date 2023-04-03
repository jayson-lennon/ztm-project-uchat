#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Router;
use fermi::use_init_atom_root;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    cx.render(rsx! {
        Router {
            Route { to: "some_url", page::Register {} },
        }
    })
}
