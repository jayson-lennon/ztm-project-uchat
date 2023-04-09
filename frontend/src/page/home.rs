#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "this is the home page"
        }
    })
}
