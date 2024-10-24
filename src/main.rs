//! Flowbite using Dioxus Rust 🦀

#![allow(non_snake_case)]

mod components;

use components::{
    accordion::{Accordion, AccordionAlwaysOpen, AccordionDefault},
    alerts::Alerts,
    buttons::{BackButtons, Buttons},
};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/alerts")]
    Alerts {},
    #[route("/accordian")]
    Accordion {},
    #[route("/accordian/default")]
    AccordionDefault {},
    #[route("/accordian/alwaysopen")]
    AccordionAlwaysOpen {},
    #[route("/buttons")]
    Buttons {},
    #[route("/buttons/back")]
    BackButtons {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col p-2 items-center justify-items",
            h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                "Flowbite using Dioxus Rust 🦀"
            }

            // Divider Line
            hr { class: "w-full border-t-2 border-gray-100 my-4" }

            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-4",
                Link { to: Route::Alerts {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Alerts"
                        }
                    }
                }
                Link { to: Route::Accordion {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Accordian"
                        }
                    }
                }
                Link { to: Route::Buttons {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-green-400 to-blue-600 group-hover:from-green-400 group-hover:to-blue-600 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-green-200 dark:focus:ring-green-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Buttons"
                        }
                    }
                }
            }
        }
    }
}
