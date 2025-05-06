use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn DashboardActivityChart() -> Element {
    rsx! {
        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.3),
            ),

            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-4",
                "Activity Overview"
            }
            div { class: "h-48 flex items-end justify-between space-x-2",
                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 40%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.4),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 65%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.5),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 85%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.6),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 55%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.7),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 70%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.8),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 90%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.9),
                        ),
                    }
                }

                div { class: "relative w-full h-full flex items-end",

                    motion::div {
                        class: "w-full bg-blue-500 dark:bg-blue-600 rounded-t-md",
                        style: "height: 75%;",
                        initial: Some(AnimationTarget::new().scale(0.0)),
                        animate: Some(AnimationTarget::new().scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(1.0),
                        ),
                    }
                }
            }
            div { class: "flex justify-between mt-2 text-xs text-gray-600 dark:text-gray-400",
                div { "Mon" }
                div { "Tue" }
                div { "Wed" }
                div { "Thu" }
                div { "Fri" }
                div { "Sat" }
                div { "Sun" }
            }
        }
    }
}
