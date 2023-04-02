#![allow(non_snake_case)]

use dioxus::prelude::*;
use fermi::use_init_atom_root;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    None
}
