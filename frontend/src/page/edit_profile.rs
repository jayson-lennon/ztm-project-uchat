#![allow(non_snake_case)]

use crate::{
    elements::{keyed_notification_box::KeyedNotifications, KeyedNotificationBox},
    prelude::*,
    util,
};
use dioxus::prelude::*;
use uchat_domain::UserFacingError;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug)]
enum PreviewImageData {
    DataUrl(String),
    Remote(String),
}

#[derive(Clone, Debug, Default)]
pub struct PageState {
    form_errors: KeyedNotifications,

    display_name: String,
    email: String,
    password: String,
    password_confirmation: String,
    profile_image: Option<PreviewImageData>,
}

#[inline_props]
pub fn ImageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);

    cx.render(rsx! {
        div {
            label {
                r#for: "image-input",
                "Upload Image"
            },
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: |_| {
                    to_owned![page_state, toaster];
                    async move {
                        use gloo_file::{File, futures::read_as_data_url};
                        use wasm_bindgen::JsCast;

                        let el = util::document()
                            .get_element_by_id("image-input")
                            .unwrap()
                            .unchecked_into::<HtmlInputElement>();
                        let file: File = el.files().unwrap().get(0).unwrap().into();
                        match read_as_data_url(&file).await {
                            Ok(data) => page_state.with_mut(|state| state.profile_image = Some(PreviewImageData::DataUrl(data))),
                            Err(e) => toaster.write().error(format!("Error loading file: {e}"), chrono::Duration::seconds(5)),
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn ImagePreview(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let image_data = page_state.with(|state| state.profile_image.clone());

    let img_el = |img_src| {
        rsx! {
            img {
                class: "profile-portrait-lg",
                src:"{img_src}",
            }
        }
    };

    let image_data = match image_data {
        Some(PreviewImageData::DataUrl(ref data)) => img_el(data),
        Some(PreviewImageData::Remote(ref url)) => img_el(url),
        None => rsx! { div { "No image uploaded"}},
    };

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-center",
            image_data
        }
    })
}

#[inline_props]
pub fn DisplayNameInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::user::DisplayName;

    let max_chars = DisplayName::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().display_name.len() > max_chars
    );


    cx.render(rsx! {
        div {
            label {
                r#for: "display-name",
                div {
                    class: "flex flex-row justify-between",
                    span { "Display Name" },
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().display_name.len()}/{max_chars}",
                    }
 
                }
            },
            input {
                id: "display-name",
                class: "input-field",
                placeholder: "Display Name",
                value: "{page_state.read().display_name}",
                oninput: move |ev| {
                    match DisplayName::new(&ev.value) {
                        Ok(_) => {
                            page_state.with_mut(|state| state.form_errors.remove("bad-displayname"));
                        }
                        Err(e) => {
                            page_state.with_mut(|state| state.form_errors.set("bad-displayname", e.formatted_error()));
                        }
                    }
                    page_state.with_mut(|state| state.display_name = ev.value.clone());
                }
            }
        }
    })
}
#[inline_props]
pub fn PasswordInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use uchat_domain::user::Password;

    let check_password_mismatch = move || {
        let password_matches = state.with(|state| state.password == state.password_confirmation);
        match password_matches {
            true => state.with_mut(|state| state.form_errors.remove("password-mismatch")),
            false => state.with_mut(|state| state.form_errors.set("password-mismatch", "Passwords must match")),
        }
    };

    cx.render(rsx!{
        fieldset {
            class: "fieldset",
            legend { "Set new password" },
            div {
                class: "flex flex-row w-full gap-2",
                div {
                    label {
                        r#for: "password",
                        "Password"
                    },
                    input {
                        id: "password",
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Password",
                        value: "{state.read().password}",
                        oninput: move |ev| {
                            match Password::new(&ev.value) {
                                Ok(_) => state.with_mut(|state| state.form_errors.remove("bad-password")),
                                Err(e) => state.with_mut(|state| state.form_errors.set("bad-password", e.formatted_error())),
                            }
                            state.with_mut(|state| state.password = ev.value.clone());
                            state.with_mut(|state| state.password_confirmation = "".to_string());

                            if state.with(|state| state.password.is_empty()) {
                                state.with_mut(|state| state.form_errors.remove("bad-password"));
                                state.with_mut(|state| state.form_errors.remove("password-mismatch"));
                            } else {
                                check_password_mismatch();
                            }
                        }
                    }
                }
                div {
                    label {
                        r#for: "password-confirm",
                        "Confirm"
                    },
                    input {
                        id: "password-confirm",
                        class: "input-field",
                        r#type: "password",
                        placeholder: "Confirm",
                        value: "{state.read().password_confirmation}",
                        oninput: move |ev| {
                            state.with_mut(|state| state.password_confirmation = ev.value.clone());
                            check_password_mismatch();
                        }
 
                }
            }
        }
    }
    })
}
 


#[inline_props]
pub fn EmailInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::user::Email;

    cx.render(rsx! {
        div {
            label {
                r#for: "email",
                div {
                    class: "flex flex-row justify-between",
                    span { "Email Address" },
                }
            },
            input {
                class: "input-field",
                id: "email",
                placeholder: "Email Address",
                value: "{page_state.read().email}",
                oninput: move |ev| {
                    if !&ev.value.is_empty() {
                        match Email::new(&ev.value) {
                            Ok(_) => {
                                page_state.with_mut(|state| state.form_errors.remove("bad-email"));
                            }
                            Err(e) => {
                                page_state.with_mut(|state| state.form_errors.set("bad-email", e.formatted_error()));
                            }
                        }
                    } else {
                        page_state.with_mut(|state| state.form_errors.remove("bad-email"));
                    }
                    page_state.with_mut(|state| state.email = ev.value.clone());
                }
            }
        }
    })
}

pub fn EditProfile(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let router = use_router(cx);

    let disable_submit = page_state.with(|state| state.form_errors.has_messages());
    let submit_btn_style = maybe_class!("btn-disabled", disable_submit);

    cx.render(rsx! {
        form {
            class: "flex flex-col w-full gap-3",
            prevent_default: "onsubmit",

            ImagePreview { page_state: page_state.clone() },
            ImageInput { page_state: page_state.clone() },
            DisplayNameInput { page_state: page_state.clone() },
            EmailInput { page_state: page_state.clone() },
            PasswordInput { state: page_state.clone() },

            KeyedNotificationBox { notifications: page_state.clone().read().form_errors.clone() },

            div {
                class: "flex flex-row justify-end gap-3",
                button {
                    class: "btn",
                    prevent_default: "onclick",
                    onclick: move |_| router.pop_route(),
                    "Cancel"
                },
                button {
                    class: "btn {submit_btn_style}",
                    r#type: "submit",
                    disabled: disable_submit,
                    "Submit"
                },
            }
        }
    })
}
