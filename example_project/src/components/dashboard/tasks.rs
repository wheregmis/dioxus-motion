use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationTarget, TransitionConfig, TransitionType};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    name: String,
    completed: bool,
}

#[component]
pub fn DashboardTasks() -> Element {
    let mut tasks = use_signal(|| {
        vec![
            Task {
                id: 1,
                name: "Complete website redesign".to_string(),
                completed: true,
            },
            Task {
                id: 2,
                name: "Review analytics dashboard".to_string(),
                completed: false,
            },
            Task {
                id: 3,
                name: "Prepare client presentation".to_string(),
                completed: false,
            },
            Task {
                id: 4,
                name: "Update documentation".to_string(),
                completed: false,
            },
            Task {
                id: 5,
                name: "Optimize performance".to_string(),
                completed: false,
            },
            Task {
                id: 6,
                name: "Fix layout bugs".to_string(),
                completed: false,
            },
        ]
    });

    // Add view mode toggle (grid or list)
    let mut is_grid = use_signal(|| false);

    let mut next_id = use_signal(|| 7);
    let mut new_task_name = use_signal(String::new);

    // Toggle task completion status
    let mut toggle_task = move |id: usize| {
        let mut updated_tasks = tasks.read().clone();
        if let Some(task) = updated_tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            tasks.set(updated_tasks);
        }
    };

    // Add a new task
    let add_task = move |_: Event<MouseData>| {
        let name = new_task_name.read().trim().to_string();
        if !name.is_empty() {
            let mut updated_tasks = tasks.read().clone();
            updated_tasks.push(Task {
                id: next_id(),
                name,
                completed: false,
            });
            tasks.set(updated_tasks);
            next_id.set(next_id() + 1);
            new_task_name.set(String::new());
        }
    };

    // Shuffle tasks to demonstrate layout animations
    let shuffle_tasks = move |_: Event<MouseData>| {
        let mut updated_tasks = tasks.read().clone();
        updated_tasks.shuffle(&mut thread_rng());
        tasks.set(updated_tasks);
        println!("DEBUG: Tasks shuffled");
    };

    // Sort tasks by completion status
    let sort_tasks = move |_: Event<MouseData>| {
        let mut updated_tasks = tasks.read().clone();
        updated_tasks.sort_by(|a, b| a.completed.cmp(&b.completed));
        tasks.set(updated_tasks);
        println!("DEBUG: Tasks sorted by completion status");
    };

    rsx! {
        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6",
            initial: AnimationTarget::default().opacity(0.0).y(30.0),
            animate: AnimationTarget::default().opacity(1.0).y(0.0),
            transition: TransitionConfig::default()
                .type_(TransitionType::Spring)
                .stiffness(80.0)   // Lower stiffness for more fluid motion
                .damping(12.0)     // Lower damping for more bounce/weight
                .duration(1.0)     // Longer duration to make animation more noticeable
                .delay(0.2),       // Shorter delay to start animation sooner

            // Header with controls
            div { class: "flex justify-between items-center mb-4",
                h3 { class: "text-lg font-semibold text-gray-800 dark:text-white",
                    "Tasks"
                }

                // Layout controls
                div { class: "flex space-x-2",
                    // Toggle grid/list view
                    motion::button {
                        class: "px-3 py-1 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-white rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        onclick: move |_| is_grid.set(!is_grid()),
                        while_hover: AnimationTarget::default().scale(1.08),
                        while_tap: AnimationTarget::default().scale(0.92),
                        transition: TransitionConfig::default()
                            .type_(TransitionType::Spring)
                            .stiffness(300.0)
                            .damping(15.0),

                        if is_grid() { "List View" } else { "Grid View" }
                    }

                    // Shuffle button
                    motion::button {
                        class: "px-3 py-1 bg-purple-500 text-white rounded-md hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2",
                        onclick: shuffle_tasks,
                        while_hover: AnimationTarget::default().scale(1.08),
                        while_tap: AnimationTarget::default().scale(0.92),
                        transition: TransitionConfig::default()
                            .type_(TransitionType::Spring)
                            .stiffness(300.0)
                            .damping(15.0),

                        "Shuffle"
                    }

                    // Sort button
                    motion::button {
                        class: "px-3 py-1 bg-green-500 text-white rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2",
                        onclick: sort_tasks,
                        while_hover: AnimationTarget::default().scale(1.08),
                        while_tap: AnimationTarget::default().scale(0.92),
                        transition: TransitionConfig::default()
                            .type_(TransitionType::Spring)
                            .stiffness(300.0)
                            .damping(15.0),

                        "Sort"
                    }
                }
            }

            // Task list with animations
            div {
                class: if is_grid() { "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4" } else { "flex flex-col gap-2 mb-4" },

                {
                    tasks
                        .read()
                        .iter()
                        .map(|task| {
                            let id = task.id;
                            let completed = task.completed;
                            rsx! {
                                motion::div {
                                    key: "{id}",
                                    class: if completed {
                                        "flex items-center p-3 rounded-md bg-gray-50 dark:bg-gray-800 opacity-70"
                                    } else {
                                        "flex items-center p-3 rounded-md bg-gray-50 dark:bg-gray-800"
                                    },
                                    // Use staggered animation with task ID for natural feel
                                    initial: AnimationTarget::default().opacity(0.2).scale(0.95),
                                    animate: AnimationTarget::default().opacity(1.0).scale(1.0),
                                    transition: TransitionConfig::default()
                                        .type_(TransitionType::Spring)
                                        .stiffness(140.0)  // Lower stiffness for more fluid motion
                                        .damping(18.0)     // Lower damping for more bounce/weight
                                        .duration(0.9)     // Longer duration to make animation more noticeable
                                        .delay((id % 10) as f32 * 0.08),  // Increase delay between items for more visible staggering

                                    input {
                                        class: "form-checkbox h-5 w-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500 mr-3",
                                        r#type: "checkbox",
                                        checked: completed,
                                        onclick: move |_| toggle_task(id),
                                    }
                                    span {
                                        class: if completed {
                                            "text-gray-500 dark:text-gray-400 line-through flex-1"
                                        } else {
                                            "text-gray-800 dark:text-white flex-1"
                                        },
                                        "{task.name}"
                                    }

                                    // Task ID badge for visual identification during animations
                                    span {
                                        class: "text-xs text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded-full",
                                        "#{id}"
                                    }
                                }
                            }
                        })
                }
            }

            // Add task form
            div { class: "flex mt-4",
                input {
                    class: "flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white mr-2",
                    r#type: "text",
                    placeholder: "Add a new task...",
                    value: new_task_name,
                    oninput: move |e| new_task_name.set(e.value().clone()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            let name = new_task_name.read().trim().to_string();
                            if !name.is_empty() {
                                let mut updated_tasks = tasks.read().clone();
                                updated_tasks
                                    .push(Task {
                                        id: next_id(),
                                        name,
                                        completed: false,
                                    });
                                tasks.set(updated_tasks);
                                next_id.set(next_id() + 1);
                                new_task_name.set(String::new());
                            }
                        }
                    },
                }

                motion::button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                    onclick: add_task,
                    while_hover: AnimationTarget::default().scale(1.08),
                    while_tap: AnimationTarget::default().scale(0.92),
                    transition: TransitionConfig::default()
                        .type_(TransitionType::Spring)
                        .stiffness(300.0)
                        .damping(15.0),

                    "Add Task"
                }
            }
        }
    }
}
