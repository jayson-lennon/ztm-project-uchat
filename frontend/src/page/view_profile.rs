#![allow(non_snake_case)]

use std::str::FromStr;

use crate::prelude::*;
use dioxus::prelude::*;
use uchat_domain::ids::UserId;

pub fn ViewProfile(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let router = use_router(cx);
    let post_manager = use_post_manager(cx);

    let profile = use_ref(cx, || None);

    let user_id = dioxus_router::use_route(cx)
        .last_segment()
        .and_then(|id| UserId::from_str(id).ok())
        .unwrap_or_default();

    cx.render(rsx! {""})
}
