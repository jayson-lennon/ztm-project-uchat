#![allow(non_snake_case)]

use std::collections::HashSet;

use crate::prelude::*;
use dioxus::prelude::*;
use itertools::Itertools;
use uchat_domain::ids::{PollChoiceId, PostId};

use uchat_endpoint::post::types::{
    Chat as EndpointChat, Image as EndpointImage, ImageKind, Poll as EndpointPoll, PublicPost,
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
pub fn Poll<'a>(cx: Scope<'a>, post_id: PostId, content: &'a EndpointPoll) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let total_votes = content
        .choices
        .iter()
        .map(|choice| choice.num_votes)
        .sum::<i64>();

    let leader_ids = {
        let leaders = content
            .choices
            .iter()
            .max_set_by(|x, y| x.num_votes.cmp(&y.num_votes));
        let ids: HashSet<PollChoiceId> = HashSet::from_iter(leaders.iter().map(|choice| choice.id));
        ids
    };

    let Choices = content.choices.iter().map(|choice| {
        let percent = if total_votes > 0 {
            let percent = (choice.num_votes as f64 / total_votes as f64) * 100.0;
            format!("{percent:.0}%")
        } else {
            "0%".to_string()
        };

        let background_color = if leader_ids.contains(&choice.id) {
            "bg-blue-300"
        } else {
            "bg-neutral-300"
        };

        let foreground_styles = maybe_class!("font-bold", leader_ids.contains(&choice.id));

        rsx! { 
            li {
                key: "{choice.id.to_string()}",
                class: "relative p-2 m-2 cursor-pointer grid grid-cols-[3rem_1fr] border rounded border-slate-400",
                onclick: move |_| (),
                div {
                    class: "absolute left-0 {background_color} h-full rounded z-[-1]",
                    style: "width: {percent}",
                },
                div {
                    class: "{foreground_styles}",
                    "{percent}",
                },
                div {
                    class: "{foreground_styles}",
                    "{choice.description.as_ref()}",
                }
            }
        }
    });

    let Headline = rsx! { figcaption { "{content.headline.as_ref()}"}};

    cx.render(rsx! {
        div {
            Headline,
            ul {
                Choices.into_iter()
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
                EndpointContent::Poll(content) => rsx! { Poll { post_id: post.id, content: content} },
            }
        }
    })
}
