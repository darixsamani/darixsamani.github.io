use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        header { class: "fixed top-0 left-0 w-full bg-gray-900/80 backdrop-blur-md border-b border-gray-700/50 z-50 px-4 sm:px-6 py-3",
            div { class: "max-w-6xl mx-auto flex items-center justify-between",
                // Logo / name
                a {
                    href: "/",
                    class: "text-xl sm:text-2xl font-bold text-white hover:text-blue-400 transition-colors",
                    "Darix SAMANI"
                }
                // Desktop nav
                nav { class: "hidden sm:flex items-center gap-6",
                    a {
                        href: "#aboutme",
                        class: "text-gray-300 hover:text-white transition-colors",
                        "About Me"
                    }
                    a {
                        href: "#skills",
                        class: "text-gray-300 hover:text-white transition-colors",
                        "Skills"
                    }

                    a {
                        href: "#projects",
                        class: "text-gray-300 hover:text-white transition-colors",
                        "Projects"
                    }
                    a {
                        href: "#experience",
                        class: "text-gray-300 hover:text-white transition-colors",
                        "Work Experiences"
                    }
                }
                // Mobile hamburger button
                button {
                    class: "sm:hidden text-gray-300 hover:text-white focus:outline-none",
                    onclick: move |_| is_menu_open.set(!is_menu_open()),
                    if *is_menu_open.read() {
                        // Close icon (X)
                        svg {
                            class: "w-6 h-6 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M18.3 5.71a1 1 0 0 0-1.42 0L12 10.59 7.11 5.7a1 1 0 0 0-1.42 1.42L10.59 12l-4.9 4.88a1 1 0 0 0 1.42 1.42L12 13.41l4.88 4.89a1 1 0 0 0 1.42-1.42L13.41 12l4.89-4.88a1 1 0 0 0 0-1.42z" }
                        }
                    } else {
                        // Hamburger icon
                        svg {
                            class: "w-6 h-6 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M4 6h16v2H4V6zm0 5h16v2H4v-2zm0 5h16v2H4v-2z" }
                        }
                    }
                }
            }
            // Mobile menu dropdown
            div {
                class: "sm:hidden overflow-hidden transition-all duration-300 ease-in-out",
                style: if *is_menu_open.read() { "max-height: 300px; opacity: 1;" } else { "max-height: 0; opacity: 0;" },
                nav { class: "flex flex-col items-center gap-4 py-4 border-t border-gray-700/50 mt-2",
                    a {
                        href: "#aboutme",
                        class: "text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| is_menu_open.set(false),
                        "About Me"
                    }
                    a {
                        href: "#skills",
                        class: "text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| is_menu_open.set(false),
                        "Skills"
                    }

                    a {
                        href: "#projects",
                        class: "text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| is_menu_open.set(false),
                        "Skills"
                    }
                    a {
                        href: "#experience",
                        class: "text-gray-300 hover:text-white transition-colors",
                        onclick: move |_| is_menu_open.set(false),
                        "Work Experiences"
                    }
                }
            }
        }
    }
}