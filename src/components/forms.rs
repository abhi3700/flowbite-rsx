use crate::{
    components::{BackButton, DividerLine},
    Route,
};
use dioxus::prelude::*;

#[component]
pub(crate) fn Forms() -> Element {
    rsx! {
        div { class: "flex flex-col p-2",
            div { class: "flex flex-row",
                BackButton { route: Route::Home {} }
                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Forms"
                }
            }
            // Divider Line
            DividerLine {}

            div { class: "grid sm:grid-cols-4 gap-3 p-2 items-center",
                Link { to: Route::DefaultForm {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Default form"
                        }
                    }
                }
                Link { to: Route::DefaultForm {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Floating labels"
                        }
                    }
                }
                Link { to: Route::DefaultForm {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Input Sizes"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn DefaultForm() -> Element {
    rsx! {
        div { class: "p-2",
            form { class: "max-w-sm mx-auto",
                div { class: "mb-5",
                    label {
                        r#for: "email",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Your email"
                    }
                    input {
                        r#type: "email",
                        id: "email",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        placeholder: "name@flowbite.com",
                        required: true
                    }
                }
                div { class: "mb-5",
                    label {
                        r#for: "password",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Your password"
                    }
                    input {
                        r#type: "password",
                        id: "password",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        required: true
                    }
                }
                div { class: "flex items-start mb-5",
                    div { class: "flex items-center h-5",
                        input {
                            id: "remember",
                            r#type: "checkbox",
                            class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800",
                            required: true
                        }
                    }
                    label {
                        r#for: "remember",
                        class: "ms-2 text-sm font-medium text-gray-900 dark:text-gray-300",
                        "Remember me"
                    }
                }
                button {
                    r#type: "submit",
                    class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    "Submit"
                }
            }
        }
    }
}
