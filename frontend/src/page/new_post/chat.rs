#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub fn NewChat(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |_| (),
            prevent_default: "onsubmit",
            // message input
            // headline input
            button {
                class: "btn",
                r#type: "submit",
                disabled: true,
                "Post"
            }
        }
    })
}
