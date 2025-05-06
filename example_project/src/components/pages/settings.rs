use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    // State for settings
    let mut theme = use_signal(|| "system".to_string());
    let mut notifications = use_signal(|| "all".to_string());
    let mut enable_animations = use_signal(|| true);
    let mut data_sync = use_signal(|| true);
    let mut email_frequency = use_signal(|| "daily".to_string());

    // Handle save settings
    let save_settings = move |_| {
        // In a real app, you would save these settings to a backend or local storage
        // For now, we'll just print a message
        println!(
            "Settings saved: Theme: {}, Notifications: {}, Animations: {}",
            theme(),
            notifications(),
            enable_animations()
        );
    };

    rsx! {
        motion::section {
            class: "p-6 max-w-2xl mx-auto",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),

            // Global attributes for accessibility
            role: "region",
            aria_label: "Settings section",

            motion::h2 {
                class: "text-2xl font-bold text-gray-800 dark:text-white mb-4",
                initial: Some(AnimationTarget::new().opacity(0.0).x(-20.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.1)
                ),
                "Settings"
            }

            motion::p {
                class: "text-gray-600 dark:text-gray-300 mb-6",
                initial: Some(AnimationTarget::new().opacity(0.0).y(10.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.2)
                ),
                "Customize your dashboard experience and preferences."
            }

            // Settings form with motion components
            motion::div {
                class: "bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden",
                initial: Some(AnimationTarget::new().opacity(0.0).y(10.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.3)
                ),

                // Form header
                div { class: "border-b border-gray-200 dark:border-gray-700 px-6 py-4",
                    h3 { class: "text-lg font-medium text-gray-800 dark:text-white", "Preferences" }
                }

                // Form content
                form { class: "px-6 py-4 space-y-6",
                    // Theme setting
                    div { class: "space-y-2",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            r#for: "theme",
                            "Theme"
                        }
                        select {
                            id: "theme",
                            class: "mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-blue-500 focus:border-blue-500 rounded-md shadow-sm",
                            value: theme,
                            oninput: move |e| theme.set(e.value().clone()),

                            option { value: "light", "Light" }
                            option { value: "dark", "Dark" }
                            option { value: "system", "System Default" }
                        }
                        p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            "Choose how the dashboard appears to you."
                        }
                    }

                    // Notifications setting
                    div { class: "space-y-2",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            r#for: "notifications",
                            "Notifications"
                        }
                        select {
                            id: "notifications",
                            class: "mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-blue-500 focus:border-blue-500 rounded-md shadow-sm",
                            value: notifications,
                            oninput: move |e| notifications.set(e.value().clone()),

                            option { value: "all", "All Notifications" }
                            option { value: "important", "Important Only" }
                            option { value: "none", "None" }
                        }
                        p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            "Control which notifications you receive."
                        }
                    }

                    // Email frequency setting
                    div { class: "space-y-2",
                        label {
                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                            r#for: "email-frequency",
                            "Email Reports"
                        }
                        select {
                            id: "email-frequency",
                            class: "mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-blue-500 focus:border-blue-500 rounded-md shadow-sm",
                            value: email_frequency,
                            oninput: move |e| email_frequency.set(e.value().clone()),

                            option { value: "daily", "Daily" }
                            option { value: "weekly", "Weekly" }
                            option { value: "monthly", "Monthly" }
                            option { value: "never", "Never" }
                        }
                        p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            "How often you want to receive email reports."
                        }
                    }

                    // Toggle switches section
                    div { class: "space-y-4 pt-4",
                        h4 { class: "text-sm font-medium text-gray-700 dark:text-gray-300", "Additional Settings" }

                        // Animations toggle
                        div { class: "flex items-center justify-between",
                            div { class: "flex flex-col",
                                span { class: "text-sm font-medium text-gray-700 dark:text-gray-300", "Enable Animations" }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Smoother transitions between pages" }
                            }

                            // Toggle switch
                            label { class: "relative inline-flex items-center cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "sr-only peer",
                                    checked: enable_animations,
                                    oninput: move |_| enable_animations.set(!enable_animations()),
                                }
                                div { class: "w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600" }
                            }
                        }

                        // Data sync toggle
                        div { class: "flex items-center justify-between",
                            div { class: "flex flex-col",
                                span { class: "text-sm font-medium text-gray-700 dark:text-gray-300", "Data Synchronization" }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Keep data in sync across devices" }
                            }

                            // Toggle switch
                            label { class: "relative inline-flex items-center cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "sr-only peer",
                                    checked: data_sync,
                                    oninput: move |_| data_sync.set(!data_sync()),
                                }
                                div { class: "w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600" }
                            }
                        }
                    }
                }

                // Form footer with action buttons
                div { class: "bg-gray-50 dark:bg-gray-800 px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3",
                    motion::button {
                        class: "px-4 py-2 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        while_hover: Some(AnimationTarget::new().scale(1.02)),
                        while_tap: Some(AnimationTarget::new().scale(0.98)),

                        "Reset to Defaults"
                    }

                    motion::button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        onclick: save_settings,
                        while_hover: Some(AnimationTarget::new().scale(1.02)),
                        while_tap: Some(AnimationTarget::new().scale(0.98)),

                        "Save Settings"
                    }
                }
            }
        }
    }
}
