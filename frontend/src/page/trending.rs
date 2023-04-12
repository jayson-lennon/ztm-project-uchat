#![allow(non_snake_case)]

use crate::{elements::post::PublicPostEntry, prelude::*};
use dioxus::prelude::*;

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            toaster
                .write()
                .info("Retrieving trending posts...", chrono::Duration::seconds(3));
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        })
    };

    let TrendingPosts = post_manager
        .read()
        .posts
        .iter()
        .map(|(&id, _)| {
            rsx! {
                div {
                    PublicPostEntry {
                        post_id: id,
                    }
                }
            }
        })
        .collect::<Vec<LazyNodes>>();

    cx.render(rsx! {
        TrendingPosts.into_iter(),
    })
}
