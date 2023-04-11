#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub headline: String,
}

#[inline_props]
pub fn MessageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Message;

    let max_chars = Message::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().message.len() > max_chars || page_state.read().message.is_empty()
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "message",
                div {
                    class: "flex flex-row justify-between",
                    span { "Message" },
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().message.len()}/{max_chars}",
                    }
                }
            },
            textarea {
                class: "input-field",
                id: "message",
                rows: 5,
                value: "{page_state.read().message}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.message = ev.data.value.clone());
                }
            }
        }
    })
}

pub fn NewChat(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |_| (),
            prevent_default: "onsubmit",
            MessageInput{ page_state: page_state.clone() },
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
