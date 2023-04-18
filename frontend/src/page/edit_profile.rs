#![allow(non_snake_case)]

use crate::{elements::keyed_notification_box::KeyedNotifications, prelude::*};
use dioxus::prelude::*;

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

pub fn EditProfile(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);

    cx.render(rsx! {""})
}
