use crate::{components::BackButtonWTitleWDivLine, Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn Forms() -> Element {
    rsx! {
        div { class: "flex flex-col p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Home {}, title: "Forms" }

            div { class: "grid sm:grid-cols-4 gap-3 p-2 items-center",
                Link { to: Route::DefaultForm {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Default form"
                        }
                    }
                }
                Link { to: Route::FloatingLabelsForm {},
                    button { class: "relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
                        span { class: "relative px-5 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0",
                            "Floating labels"
                        }
                    }
                }
                Link { to: Route::InputSizesForm {},
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
        div { class: "p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Forms {}, title: "Default form" }

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

#[component]
pub(crate) fn FloatingLabelsForm() -> Element {
    rsx! {
        div { class: "p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Forms {}, title: "Floating labels" }

            form { class: "max-w-md mx-auto",
                div { class: "relative z-0 w-full mb-5 group",
                    input {
                        r#type: "email",
                        name: "floating_email",
                        id: "floating_email",
                        class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                        placeholder: " ",
                        required: true
                    }
                    label {
                        r#for: "floating_email",
                        class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                        "Email address"
                    }
                }
                div { class: "relative z-0 w-full mb-5 group",
                    input {
                        r#type: "password",
                        name: "floating_password",
                        id: "floating_password",
                        class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                        placeholder: " ",
                        required: true
                    }
                    label {
                        r#for: "floating_password",
                        class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                        "Password"
                    }
                }
                div { class: "relative z-0 w-full mb-5 group",
                    input {
                        r#type: "password",
                        name: "repeat_password",
                        id: "floating_repeat_password",
                        class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                        placeholder: " ",
                        required: true
                    }
                    label {
                        r#for: "floating_repeat_password",
                        class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                        "Confirm password"
                    }
                }
                div { class: "grid md:grid-cols-2 md:gap-6",
                    div { class: "relative z-0 w-full mb-5 group",
                        input {
                            r#type: "text",
                            name: "floating_first_name",
                            id: "floating_first_name",
                            class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                            placeholder: " ",
                            required: true
                        }
                        label {
                            r#for: "floating_first_name",
                            class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                            "First name"
                        }
                    }
                    div { class: "relative z-0 w-full mb-5 group",
                        input {
                            r#type: "text",
                            name: "floating_last_name",
                            id: "floating_last_name",
                            class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                            placeholder: " ",
                            required: true
                        }
                        label {
                            r#for: "floating_last_name",
                            class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                            "Last name"
                        }
                    }
                }
                div { class: "grid md:grid-cols-2 md:gap-6",
                    div { class: "relative z-0 w-full mb-5 group",
                        input {
                            r#type: "tel",
                            pattern: "[0-9]{3}-[0-9]{3}-[0-9]{4}",
                            name: "floating_phone",
                            id: "floating_phone",
                            class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                            placeholder: " ",
                            required: true
                        }
                        label {
                            r#for: "floating_phone",
                            class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                            "Phone number (123-456-7890)"
                        }
                    }
                    div { class: "relative z-0 w-full mb-5 group",
                        input {
                            r#type: "text",
                            name: "floating_company",
                            id: "floating_company",
                            class: "block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                            placeholder: " ",
                            required: true
                        }
                        label {
                            r#for: "floating_company",
                            class: "peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                            "Company (Ex. Google)"
                        }
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

#[component]
pub(crate) fn InputSizesForm() -> Element {
    rsx! {
        div { class: "p-2 w-full min-h-screen dark:bg-gray-700",
            BackButtonWTitleWDivLine { route: Route::Forms {}, title: "Input Sizes" }
            form { class: "max-w-sm mx-auto",
                div { class: "mb-5",
                    label {
                        r#for: "large-input",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Large input"
                    }
                    input {
                        r#type: "text",
                        id: "large-input",
                        class: "block w-full p-4 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-base focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    }
                }
                div { class: "mb-5",
                    label {
                        r#for: "base-input",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Base input"
                    }
                    input {
                        r#type: "text",
                        id: "base-input",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    }
                }
                div {
                    label {
                        r#for: "small-input",
                        class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        "Small input"
                    }
                    input {
                        r#type: "text",
                        id: "small-input",
                        class: "block w-full p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 text-xs focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    }
                }
            }
        }
    }
}
