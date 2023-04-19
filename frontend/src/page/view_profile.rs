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

    use_effect(cx, (&user_id,), |(user_id,)| {
        to_owned![api_client, post_manager, profile, toaster];
        async move {
            post_manager.write().clear();
            use uchat_endpoint::user::endpoint::{ViewProfile, ViewProfileOk};
            let request = ViewProfile { for_user: user_id };
            let response = fetch_json!(<ViewProfileOk>, api_client, request);
            match response {
                Ok(res) => {
                    profile.with_mut(|profile| *profile = Some(res.profile));
                    post_manager.write().populate(res.posts.into_iter());
                }
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        }
    });

    cx.render(rsx! {""})
}
