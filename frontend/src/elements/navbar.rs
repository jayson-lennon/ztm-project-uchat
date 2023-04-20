#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let router = use_router(cx);

    let hide_class = maybe_class!("hidden", *hide.get());

    const BUTTON_CLASS: &str = "grid grid-cols-[20px_1fr] gap-4 pl-4
                                justify-center items-center
                                w-full h-12
                                border-y navbar-border-color";
    cx.render(rsx! {
        div {
            class: "flex flex-col
                    absolute right-0 bottom-[var(--navbar-height)]
                    w-28 items-center {hide_class}
                    navbar-bg-color text-white text-sm",
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    router.navigate_to(page::POST_NEW_POLL);
                    hide.set(true);
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-poll.svg",
                },
                "Poll"
            }
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    router.navigate_to(page::POST_NEW_IMAGE);
                    hide.set(true);
                },

                img {
                    class: "invert",
                    src: "/static/icons/icon-image.svg",
                },
                "Image"
            }
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    router.navigate_to(page::POST_NEW_CHAT);
                    hide.set(true);
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-messages.svg",
                },
                "Chat"
            }
        }
    })
}

#[derive(Props)]
pub struct NavButtonProps<'a> {
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    highlight: Option<bool>,
    children: Element<'a>,
}

pub fn NavButton<'a>(cx: Scope<'a, NavButtonProps<'a>>) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", matches!(cx.props.highlight, Some(true)));

    cx.render(rsx! {
        button {
            class: "cursor-pointer flex flex-col items-center justify-center h-full {selected_bgcolor}",
            onclick: move |ev| cx.props.onclick.call(ev),
            img {
                class: "invert",
                src: cx.props.img,
                width: "25px",
                height: "25px",
            },
            div {
                class: "text-sm text-white",
                cx.props.label
            },
            &cx.props.children
        }
    })
}

pub fn Navbar(cx: Scope) -> Element {
    let hide_new_post_popup = use_state(cx, || true);
    let router = use_router(cx);
    let route = use_route(cx);

    let hide_navbar = use_state(cx, || false);

    let current_route = route.url().path().to_string();

    use_effect(cx, (&current_route,), |(current_route,)| {
        to_owned![hide_navbar];
        async move {
            let should_hide =
                current_route == page::ACCOUNT_LOGIN || current_route == page::ACCOUNT_REGISTER;
            hide_navbar.set(should_hide);
        }
    });

    if *hide_navbar.get() {
        return None;
    }

    cx.render(rsx! {
        nav {
            class: "max-w-[var(--content-max-width)] h-[var(--navbar-height)]
                fixed bottom-0 left-0 right-0 mx-auto
                border-t navbar-bg-color navbar-border-color",
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                NavButton {
                    img: "/static/icons/icon-home.svg",
                    label: "Home",
                    // onclick: |_| (),
                    onclick: move |_| router.navigate_to(page::HOME),
                },
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: move |_| router.navigate_to(page::POSTS_TRENDING),
                }
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: move |_| {
                        let is_hidden = *hide_new_post_popup.get();
                        hide_new_post_popup.set(!is_hidden);
                    }
                    NewPostPopup { hide: hide_new_post_popup.clone() }
                }
            }
        }
    })
}
