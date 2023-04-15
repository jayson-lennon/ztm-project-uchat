#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use uchat_domain::ids::PostId;

use uchat_endpoint::post::types::{
    Chat as EndpointChat, Image as EndpointImage, ImageKind, PublicPost,
};

#[inline_props]
pub fn Chat<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointChat) -> Element {
    let Headline = content.headline.as_ref().map(|headline| {
        rsx! {
            div {
                class: "font-bold",
                "{headline.as_ref()}"
            }
        }
    });

    cx.render(rsx! {
        div {
            Headline,
            p { "{content.message.as_ref()}" }
        }
    })
}

#[inline_props]
pub fn Image<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointImage) -> Element {
    let url = if let ImageKind::Url(url) = &content.kind {
        url
    } else {
        return cx.render(rsx! { "image not found" });
    };

    let Caption = content
        .caption
        .as_ref()
        .map(|caption| rsx! { figcaption { em { "{caption.as_ref()}" } } });

    cx.render(rsx! {
        figure {
            class: "flex flex-col gap-2",
            Caption,
            img {
                class: "w-full object-contain max-h-[80vh]",
                src: "{url}"
            }
        }
    })
}

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element {
    use uchat_endpoint::post::types::Content as EndpointContent;
    cx.render(rsx! {
        div {
            match &post.content {
                EndpointContent::Chat(content) => rsx! { Chat { post_id: post.id, content: content} },
                EndpointContent::Image(content) => rsx! { Image { post_id: post.id, content: content} },
                _ => rsx! { "" }
            }
        }
    })
}
