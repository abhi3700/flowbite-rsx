//! Buttons
//!
//! https://flowbite.com/docs/components/buttons/

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Buttons() -> Element {
    rsx! {
        div { class: "flex flex-col p-2",
            div { class: "flex flex-row mb-2",
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
                    "Buttons"
                }
            }
            // Divider Line
            hr { class: "w-full border-t-2 border-gray-100 my-4" }
            h5 { class: "mb-2 text-lg font-bold tracking-tight text-gray-900 dark:text-white",
                "Back Buttons"
            }
            BackButtons {}
            hr { class: "w-full border-t-2 border-gray-100 my-4" }
        }
    }
}

#[component]
pub(crate) fn BackButtons() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 p-3",
            Link { to: Route::Buttons {},
                button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
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
                            d: "M20 12H4",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round"
                        }
                    }
                }
            }
            Link { to: Route::Buttons {},
                div { class: "flex items-center space-x-2 py-1 px-3 rounded-md hover:bg-gray-200 transition-all duration-200",
                    // Back arrow icon
                    svg {
                        class: "w-5 h-5 text-gray-600",
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
                            d: "M20 12H4",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round"
                        }
                    }

                    // Text label
                    span { class: "text-sm font-medium text-gray-700", "Back" }
                }
            }

            Link { to: Route::Buttons {},
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
                            d: "M22 12H4",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round"
                        }
                    }
                }
            }
        }
    }
}
