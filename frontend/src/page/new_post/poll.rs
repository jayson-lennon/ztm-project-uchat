#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{fetch_json, prelude::*, util};
use chrono::Duration;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_domain::post::{PollChoiceDescription, PollHeadline};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub headline: String,
    pub poll_choices: HashMap<usize, String>,
    pub next_id: usize,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            headline: "".to_string(),
            poll_choices: {
                let mut map = HashMap::new();
                map.insert(0, "".to_string());
                map.insert(1, "".to_string());
                map
            },
            next_id: 2,
        }
    }
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        if PollHeadline::new(&self.headline).is_err() {
            return false;
        }

        if self.poll_choices.len() < 2 {
            return false;
        }

        if self
            .poll_choices
            .values()
            .map(PollChoiceDescription::new)
            .collect::<Result<Vec<PollChoiceDescription>, _>>()
            .is_err()
        {
            return false;
        }
        true
    }

    pub fn push_choice<T: Into<String>>(&mut self, choice: T) {
        self.poll_choices.insert(self.next_id, choice.into());
        self.next_id += 1;
    }

    pub fn replace_choice<T: Into<String>>(&mut self, key: usize, choice: T) {
        self.poll_choices.insert(key, choice.into());
    }
}

#[inline_props]
pub fn HeadlineInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let max_chars = PollHeadline::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().headline.len() > max_chars || page_state.read().headline.is_empty()
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "headline",
                div {
                    class: "flex flex-row justify-between",
                    span { "Headline" },
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().headline.len()}/{max_chars}",
                    }
                }
            },
            input {
                class: "input-field",
                id: "headline",
                value: "{page_state.read().headline}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.headline = ev.data.value.clone());
                }
            }
        }
    })
}

pub fn NewPoll(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let page_state = use_ref(cx, PageState::default);

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, toaster, router],
        move |_| async move {
            use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};

            let request = NewPost {
                content: Poll {
                    // todo
                }
                .into(),
                options: NewPostOptions::default(),
            };
            let response = fetch_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    toaster.write().success("Posted!", Duration::seconds(3));
                    router.replace_route(page::HOME, None, None);
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Post failed: {e}"), Duration::seconds(3));
                }
            }
        }
    );

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.read().can_submit());

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: form_onsubmit,
            prevent_default: "onsubmit",
            HeadlineInput { page_state: page_state.clone() },
            // poll choices
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Post"
            }
        }
    })
}
