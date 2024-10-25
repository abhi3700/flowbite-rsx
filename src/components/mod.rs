//! Components

pub(crate) mod accordion;
pub(crate) mod alerts;
pub(crate) mod buttons;
pub(crate) mod forms;
// TODO: Add more

use crate::Route;
use dioxus::prelude::*;

// === Common

#[component]
pub(crate) fn BackButtonWTitleWDivLine(route: Route, title: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-row",
            BackButton { route }
            h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                {title}
            }
        }
        // Divider Line
        DividerLine {}
    }
}

#[component]
pub(crate) fn BackButton(route: Route) -> Element {
    rsx! {
        Link { to: route,
            button { class: "hover:bg-gray-300 py-1.5 px-5 rounded-md",
                svg {
                    class: "w-5 h-5",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    path {
                        d: "M10 19l-7-7 7-7",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round"
                    }
                    path {
                        d: "M100 12H4",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round"
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DividerLine() -> Element {
    rsx! {
        hr { class: "w-full border-t-2 border-gray-100 my-4" }
    }
}
