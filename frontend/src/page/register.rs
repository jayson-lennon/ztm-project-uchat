#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},

            button {
                class: "btn",
                r#type: "submit",
                "Signup"
            }
        }
    })
}
