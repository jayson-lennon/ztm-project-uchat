#![allow(non_snake_case)]

use dioxus::{html::h1, prelude::*};
use fermi::use_init_atom_root;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    cx.render(rsx! {
        h1 { "Hello, World!" }
    })
}
