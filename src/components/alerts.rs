//! Alerts
//!
//! https://flowbite.com/docs/components/alerts/

use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn Alerts() -> Element {
    // TODO: complete the code
    rsx! {
        div { class: "flex flex-col p-2",
            div { class: "flex flex-row",
                Link { to: Route::Home {},
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

                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Alerts"
                }
            }
            // Divider Line
            hr { class: "w-full border-t-2 border-gray-100 my-4" }
        }
    }
}
