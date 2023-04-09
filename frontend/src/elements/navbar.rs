#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct NavButtonProps<'a> {
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    highlight: Option<bool>,
    children: Element<'a>,
}

pub fn NavButton<'a>(cx: Scope<'a, NavButtonProps<'a>>) -> Element {
    cx.render(rsx! {})
}

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "max-w-[var(--content-max-width)] h-[var(-navbar-height)]
                fixed bottom-0 left-0 right-0 mx-auto
                border-t navbar-bg-color navbar-border-color"
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
            }
        }
    })
}
