//! Alerts
//!
//! https://flowbite.com/docs/components/alerts/

use crate::{components::BackButtonWTitleWDivLine, Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn Alerts() -> Element {
    // TODO: complete the code
    rsx! {
        div { class: "flex flex-col p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Home {}, title: "Alerts" }
        }
    }
}
