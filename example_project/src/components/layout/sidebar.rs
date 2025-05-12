use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn Sidebar(sidebar_collapsed: Signal<bool>, active_section: Signal<String>) -> Element {
    rsx! {
        div { class: if sidebar_collapsed() { "w-20 h-screen bg-gray-800 text-white transition-all duration-300 flex flex-col shadow-lg" } else { "w-64 h-screen bg-gray-800 text-white transition-all duration-300 flex flex-col shadow-lg" },

            // Logo and collapse button
            div { class: "flex items-center justify-between p-4 border-b border-gray-700",
                // Logo with icon for collapsed state
                div { class: "flex items-center",
                    span { class: "text-2xl mr-3", "📊" }
                    h2 { class: if sidebar_collapsed() { "hidden" } else { "text-xl font-semibold" },
                        "Motion Dashboard"
                    }
                }

                motion::button {
                    class: "p-2 rounded-full hover:bg-gray-700 focus:outline-none",
                    onclick: move |_| sidebar_collapsed.set(!sidebar_collapsed()),
                    while_hover: Some(AnimationTarget::new().scale(1.1)),
                    while_tap: Some(AnimationTarget::new().scale(0.9)),

                    if sidebar_collapsed() {
                        "→"
                    } else {
                        "←"
                    }
                }
            }

            // Navigation links
            nav { class: "flex-1 py-4",
                motion::div {
                    class: if active_section() == "dashboard" { "flex items-center px-4 py-3 text-white bg-blue-600 cursor-pointer" } else { "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 cursor-pointer" },
                    onclick: move |_| active_section.set("dashboard".to_string()),
                    while_hover: Some(AnimationTarget::new().x(5.0)),

                    // Icon and text in separate spans for better control
                    span { class: "text-xl mr-3", "📊" }
                    span { class: if sidebar_collapsed() { "hidden" } else { "text-sm" }, "Dashboard" }
                }

                motion::div {
                    class: if active_section() == "analytics" { "flex items-center px-4 py-3 text-white bg-blue-600 cursor-pointer" } else { "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 cursor-pointer" },
                    onclick: move |_| active_section.set("analytics".to_string()),
                    while_hover: Some(AnimationTarget::new().x(5.0)),

                    span { class: "text-xl mr-3", "📈" }
                    span { class: if sidebar_collapsed() { "hidden" } else { "text-sm" }, "Analytics" }
                }

                motion::div {
                    class: if active_section() == "reports" { "flex items-center px-4 py-3 text-white bg-blue-600 cursor-pointer" } else { "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 cursor-pointer" },
                    onclick: move |_| active_section.set("reports".to_string()),
                    while_hover: Some(AnimationTarget::new().x(5.0)),

                    span { class: "text-xl mr-3", "📝" }
                    span { class: if sidebar_collapsed() { "hidden" } else { "text-sm" }, "Reports" }
                }

                motion::div {
                    class: if active_section() == "settings" { "flex items-center px-4 py-3 text-white bg-blue-600 cursor-pointer" } else { "flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 cursor-pointer" },
                    onclick: move |_| active_section.set("settings".to_string()),
                    while_hover: Some(AnimationTarget::new().x(5.0)),

                    span { class: "text-xl mr-3", "⚙️" }
                    span { class: if sidebar_collapsed() { "hidden" } else { "text-sm" }, "Settings" }
                }
            }
        }
    }
}
