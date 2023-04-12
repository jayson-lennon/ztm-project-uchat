#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            toaster
                .write()
                .info("Retrieving trending posts...", chrono::Duration::seconds(3));
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => (),
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        })
    };

    cx.render(rsx! {
        h1 {"trending"}
    })
}
