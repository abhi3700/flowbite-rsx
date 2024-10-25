//! Alerts
//!
//! https://flowbite.com/docs/components/alerts/

use crate::{
    components::{BackButton, DividerLine},
    Route,
};
use dioxus::prelude::*;

#[component]
pub(crate) fn Alerts() -> Element {
    // TODO: complete the code
    rsx! {
        div { class: "flex flex-col p-2",
            div { class: "flex flex-row",
                BackButton { route: Route::Home {} }
                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Alerts"
                }
            }
            // Divider Line
            DividerLine {}
        }
    }
}
