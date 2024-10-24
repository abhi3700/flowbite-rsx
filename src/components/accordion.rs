//! Accordion
//!
//! https://flowbite.com/docs/components/accordion/

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Accordion() -> Element {
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
                    "Accordian"
                }
            }
            // Divider Line
            hr { class: "w-full border-t-2 border-gray-100 my-4" }

            div { class: "grid sm:grid-cols-4 gap-3 p-2 items-center",
                Link { to: Route::AccordionDefault {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Default accordion"
                        }
                    }
                }
                Link { to: Route::AccordionAlwaysOpen {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Always open"
                        }
                    }
                }
                Link { to: Route::AccordionAlwaysOpen {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Color options"
                        }
                    }
                }
            }
        }
    }
}

/// https://flowbite.com/docs/components/accordion/#default-accordion
#[component]
pub(crate) fn AccordionDefault() -> Element {
    // State management for accordion sections
    let mut is_open_1 = use_signal(|| false);
    let mut is_open_2 = use_signal(|| false);
    let mut is_open_3 = use_signal(|| false);

    rsx! {
        div {
            id: "accordion-collapse",
            "data-accordion": "collapse",
            class: "p-2",
            div { class: "flex flex-row gap-3 mb-2",
                Link { to: Route::Accordion {},
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
                    "Default Accordian"
                }
            }

            // Accordion Item 1
            h2 { id: "accordion-collapse-heading-1",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 rounded-t-xl focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-1",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-1",
                    onclick: move |_| is_open_1.set(!is_open_1()),
                    span { "What is Flowbite?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: if is_open_1() { "w-3 h-3 shrink-0" } else { "w-3 h-3 shrink-0 rotate-180" },
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5"
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-1",
                class: if is_open_1() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-1",
                div { class: "p-5 border border-b-0 border-gray-200 dark:border-gray-700 dark:bg-gray-900",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Flowbite is an open-source library..."
                    }
                    p { class: "text-gray-500 dark:text-gray-400",
                        "Check out this guide to learn how to "
                        a {
                            href: "/docs/getting-started/introduction/",
                            class: "text-blue-600 dark:text-blue-500 hover:underline",
                            "get started"
                        }
                        " and start developing websites even faster with components on top of Tailwind CSS."
                    }
                }
            }

            // Accordion Item 2
            h2 { id: "accordion-collapse-heading-2",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-2",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-2",
                    onclick: move |_| is_open_2.set(!is_open_2()),
                    span { "Is there a Figma file available?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: format_args!("w-3 h-3 shrink-0 {}", if is_open_2() { "" } else { "rotate-180" }),
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5"
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-2",
                class: if is_open_2() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-2",
                div { class: "p-5 border border-b-0 border-gray-200 dark:border-gray-700",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Flowbite is first conceptualized and designed..."
                    }
                    p { class: "text-gray-500 dark:text-gray-400",
                        "Check out the "
                        a {
                            href: "https://flowbite.com/figma/",
                            class: "text-blue-600 dark:text-blue-500 hover:underline",
                            "Figma design system"
                        }
                        " based on the utility classes from Tailwind CSS and components from Flowbite."
                    }
                }
            }

            // Accordion Item 3
            h2 { id: "accordion-collapse-heading-3",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-3",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-3",
                    onclick: move |_| is_open_3.set(!is_open_3()),
                    span { "What are the differences between Flowbite and Tailwind UI?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: if is_open_3() { "w-3 h-3 shrink-0" } else { "w-3 h-3 shrink-0 rotate-180" },
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5"
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-3",
                class: if is_open_3() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-3",
                div { class: "p-5 border border-t-0 border-gray-200 dark:border-gray-700",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "The main difference is that..."
                    }
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "However, we actually recommend using both..."
                    }
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Learn more about these technologies:"
                    }
                    ul { class: "ps-5 text-gray-500 list-disc dark:text-gray-400",
                        li {
                            a {
                                href: "https://flowbite.com/pro/",
                                class: "text-blue-600 dark:text-blue-500 hover:underline",
                                "Flowbite Pro"
                            }
                        }
                        li {
                            a {
                                href: "https://tailwindui.com/",
                                class: "text-blue-600 dark:text-blue-500 hover:underline",
                                "Tailwind UI"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn AccordionAlwaysOpen() -> Element {
    rsx! {
        div { class: "p-2",
            div { class: "flex flex-row gap-3 mb-2",
                Link { to: Route::Accordion {},
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
                    "Always Open"
                }
            }
        }
    }
}
