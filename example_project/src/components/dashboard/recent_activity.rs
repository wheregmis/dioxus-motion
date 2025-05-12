use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn DashboardRecentActivity() -> Element {
    let mut activity_items = use_signal(|| {
        vec![
            (
                "Website Redesign",
                "New homepage design approved",
                "2 hours ago",
            ),
            (
                "Marketing Campaign",
                "Q3 campaign metrics analyzed",
                "4 hours ago",
            ),
            (
                "Product Update",
                "Version 2.1 released to production",
                "Yesterday",
            ),
            (
                "Client Meeting",
                "Meeting with Acme Corp scheduled",
                "Yesterday",
            ),
            (
                "Team Onboarding",
                "New developer joined the team",
                "2 days ago",
            ),
        ]
    });

    let mut dragging_item = use_signal(|| None::<usize>);
    let mut drop_target = use_signal(|| None::<usize>);

    let mut handle_drag_start = move |index: usize| {
        dragging_item.set(Some(index));
    };

    let mut handle_drag_over = move |index: usize| {
        if dragging_item() != Some(index) {
            drop_target.set(Some(index));
        } else {
            drop_target.set(None);
        }
    };

    let mut handle_drag_end = move || {
        if let (Some(drag_idx), Some(drop_idx)) = (dragging_item(), drop_target()) {
            let mut items = activity_items.read().clone();
            let item = items.remove(drag_idx);
            items.insert(drop_idx, item);
            activity_items.set(items);
        }
        dragging_item.set(None);
        drop_target.set(None);
    };

    rsx! {
        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.4),
            ),

            div { class: "p-6",
                h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-4",
                    "Recent Activity"
                }
                div { class: "space-y-3",
                    {
                        activity_items
                            .read()
                            .iter()
                            .enumerate()
                            .map(|(index, (title, desc, time))| {
                                let is_dragging = dragging_item() == Some(index);
                                let is_drop_target = drop_target() == Some(index);
                                let class_name = if is_dragging {
                                    "relative transform scale-105 shadow-lg z-10 transition-all duration-200"
                                } else if is_drop_target {
                                    "relative border-2 border-dashed border-blue-300 dark:border-blue-500 rounded-md transition-all duration-200"
                                } else {
                                    "relative transition-all duration-200"
                                };
                                rsx! {
                                    div {
                                        key: "{index}",
                                        class: class_name,
                                        onmouseenter: move |_| handle_drag_over(index),
                                        div { class: "flex items-center p-3 bg-gray-50 dark:bg-gray-800 rounded-md",
                                            // Drag handle
                                            div {
                                                class: "cursor-move mr-3 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200",
                                                onmousedown: move |_| handle_drag_start(index),
                                                onmouseup: move |_| handle_drag_end(),
                                                div { class: "flex flex-col items-center",
                                                    div { class: "flex space-x-1 mb-0.5",
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                    }
                                                    div { class: "flex space-x-1 mb-0.5",
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                    }
                                                    div { class: "flex space-x-1",
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                        div { class: "w-1 h-1 rounded-full bg-current" }
                                                    }
                                                }
                                            }
                                            // Content
                                            div { class: "flex-1",
                                                div { class: "font-medium text-gray-800 dark:text-white", "{title}" }
                                                div { class: "text-sm text-gray-600 dark:text-gray-300", "{desc}" }
                                            }
                                            // Time
                                            div { class: "text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap", "{time}" }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}
