use crate::{components::BackButtonWTitleWDivLine, Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn Dropdowns() -> Element {
    rsx! {
        div { class: "flex flex-col p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Home {}, title: "Dropdowns" }

            div { class: "grid sm:grid-cols-4 gap-3 p-2 items-center",
                Link { to: Route::DropdownExample {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Dropdown example"
                        }
                    }
                }
                Link { to: Route::DropdownHover {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Dropdown hover"
                        }
                    }
                }
                Link { to: Route::DropdownDelayDuration {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Delay duration"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DropdownExample() -> Element {
    rsx! {
        div { class: "flex flex-col p-2 w-full min-h-screen dark:bg-gray-700",
            button {
                id: "dropdownDefaultButton",
                "data-dropdown-toggle": "dropdown",
                class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none \
                        focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center \
                        inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                "type": "button",
                "Dropdown button"
                svg {
                    class: "w-2.5 h-2.5 ms-3",
                    "aria-hidden": "true",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 10 6",
                    path {
                        stroke: "currentColor",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        "stroke-width": "2",
                        d: "M1 1 L5 5 L9 1"
                    }
                }
            }

            div {
                id: "dropdown",
                class: "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700",
                ul {
                    class: "py-2 text-sm text-gray-700 dark:text-gray-200",
                    "aria-labelledby": "dropdownDefaultButton",
                    li {
                        a {
                            href: "#",
                            class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                            "Dashboard"
                        }
                    }
                    li {
                        a {
                            href: "#",
                            class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                            "Settings"
                        }
                    }
                    li {
                        a {
                            href: "#",
                            class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                            "Earnings"
                        }
                    }
                    li {
                        a {
                            href: "#",
                            class: "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white",
                            "Sign out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DropdownHover() -> Element {
    rsx! {}
}

#[component]
pub(crate) fn DropdownDelayDuration() -> Element {
    rsx! {}
}
