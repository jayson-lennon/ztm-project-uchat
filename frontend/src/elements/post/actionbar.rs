#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use uchat_domain::ids::PostId;

#[inline_props]
pub fn Bookmark(cx: Scope, post_id: PostId, bookmarked: bool) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let icon = match bookmarked {
        true => "/static/icons/icon-bookmark-saved.svg",
        false => "/static/icons/icon-bookmark.svg",
    };

    let bookmark_onclick = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |_| async move {
            use uchat_endpoint::post::endpoint::{Bookmark, BookmarkOk};
        }
    );

    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: bookmark_onclick,
            img {
                class: "actionbar-icon",
                src: "{icon}",
            }
        }
    })
}

#[inline_props]
pub fn Actionbar(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);

    let this_post = post_manager.read();
    let this_post = this_post.get(&post_id).unwrap();
    let this_post_id = this_post.id;

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between w-full opacity-70 mt-4",
            // boost
            // bookmark
            // like & dislike
            // comment
        }
        // quick respond
    })
}
