#![allow(non_snake_case)]

use crate::prelude::*;
use chrono::Duration;
use dioxus::prelude::*;
use uchat_domain::{post::Message};

fn can_submit(message: &str) -> bool {
    message.len() <= Message::MAX_CHARS && !message.is_empty()
}

#[inline_props]
pub fn MessageInput<'a>(
    cx: Scope<'a>,
    message: &'a str,
    on_input: EventHandler<'a, FormEvent>,
) -> Element {
    let max_chars = Message::MAX_CHARS;

    let wrong_len = maybe_class!("err-text-color", !can_submit(message));

    cx.render(rsx! {
        div {
            class: "flex flex-row relative",
            textarea {
                class: "input-field",
                id: "message",
                rows: 3,
                value: "{message}",
                oninput: move |ev| on_input.call(ev),
            },
            div {
                class: "text-right {wrong_len} absolute bottom-1 right-1",
                "{message.len()}/{max_chars}"
            }
        }
    })
}

#[inline_props]
pub fn QuickRespond(cx: Scope, opened: UseState<bool>) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);

    let message = use_state(cx, || "".to_string());

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, toaster, message, opened],
        move |_| async move {
            use uchat_domain::post::Message;
            use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
            use uchat_endpoint::post::types::{Chat, NewPostOptions};

            let request = NewPost {
                content: Chat {
                    headline: None,
                    message: Message::new(message.get()).unwrap(),
                }
                .into(),
                options: NewPostOptions::default(),
            };
            let response = fetch_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    toaster.write().success("Posted!", Duration::seconds(3));
                    opened.set(false);
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Reply failed: {e}"), Duration::seconds(3));
                }
            }
        }
    );

    let submit_cursor = if can_submit(message.get()) {
        "cursor-pointer"
    } else {
        "cursor-not-allowed"
    };

    let submit_btn_style = maybe_class!("btn-disabled", !can_submit(message.get()));

    cx.render(rsx! {
        form {
            onsubmit: form_onsubmit,
            prevent_default: "onsubmit",
            MessageInput {
                message: message,
                on_input: move |ev: FormEvent| {
                    message.set(ev.value.clone());
                }
            }
            div {
                class: "w-full flex flex-row justify-end",
                button {
                    class: "mt-2 btn {submit_cursor} {submit_btn_style}",
                    r#type: "submit",
                    disabled: !can_submit(message.get()),
                    "Respond"
                }
            }
        }
    })
}
