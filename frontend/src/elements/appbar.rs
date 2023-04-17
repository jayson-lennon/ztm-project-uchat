#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct AppbarProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Appbar<'a>(cx: Scope<'a, AppbarProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "max-w-[var(--content-max-width)] h-[var(--appbar-height)]
                    fixed top-0 right-0 mx-auto z-50
                    bg-slate-200",
            div {
                class: "flex flex-row gap-8 items-center w-full pr-5 h-full",
                div {
                    class: "cursor-pointer",
                    onclick: move |_| (),
                    img {
                        class: "profile-portrait",
                        src: ""
                    },
                },
                div {
                    class: "text-xl font-bold mr-auto",
                    "{cx.props.title}",
                }
                &cx.props.children
            }
        }
    })
}
