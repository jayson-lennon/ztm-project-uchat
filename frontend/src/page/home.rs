#![allow(non_snake_case)]

use crate::prelude::*;
use chrono::Duration;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);

    cx.render(rsx! {
        h1 {
            "this is the home page"
        }
        button {
            onclick: move |_| {
                toaster.write().success("success", Duration::seconds(5));
                toaster.write().info("info", Duration::seconds(5));
                toaster.write().error("error", Duration::seconds(5));
            },
            "toast"
        }
    })
}
