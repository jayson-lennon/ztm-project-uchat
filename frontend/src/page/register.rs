#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    elements::{keyed_notification_box::KeyedNotifications, KeyedNotificationBox},
    prelude::*,
};

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }
    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
            || self.username.current().is_empty()
            || self.password.current().is_empty())
    }
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "Password",
            },
            input {
                id: "password",
                r#type: "password",
                name: "password",
                class: "input-field",
                placeholder: "Password",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username",
            },
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "User Name",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Username::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-username", e.to_string()));
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-username"));
        }
        page_state.with_mut(|state| state.username.set(ev.value.clone()));
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Password::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-password", e.to_string()));
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-password"));
        }
        page_state.with_mut(|state| state.password.set(ev.value.clone()));
    });

    let submit_btn_style =
        maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},

            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput,
            },

            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput,
            },

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone()),
            }

            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.with(|state| state.can_submit()),
                "Signup"
            }
        }
    })
}
