#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use uchat_domain::ids::PostId;

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
