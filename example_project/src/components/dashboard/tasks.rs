use dioxus::prelude::*;
use dioxus_motion::prelude::*;

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
        ]
    });

    let mut next_id = use_signal(|| 5);
    let mut new_task_name = use_signal(String::new);

    let mut toggle_task = move |id: usize| {
        let mut updated_tasks = tasks.read().clone();
        if let Some(task) = updated_tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            tasks.set(updated_tasks);
        }
    };

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

    rsx! {
        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.5),
            ),

            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-4",
                "Tasks"
            }
            div { class: "space-y-2 mb-4",
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
                                    class: if completed { "flex items-center p-2 rounded-md bg-gray-50 dark:bg-gray-800 opacity-70" } else { "flex items-center p-2 rounded-md bg-gray-50 dark:bg-gray-800" },
                                    initial: Some(AnimationTarget::new().opacity(0.0).x(-10.0)),
                                    animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                                    transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),
                                
                                    input {
                                        class: "form-checkbox h-5 w-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500 mr-3",
                                        r#type: "checkbox",
                                        checked: completed,
                                        onclick: move |_| toggle_task(id),
                                    }
                                    span { class: if completed { "text-gray-500 dark:text-gray-400 line-through" } else { "text-gray-800 dark:text-white" },
                                        "{task.name}"
                                    }
                                }
                            }
                        })
                }
            }

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
                    while_hover: Some(AnimationTarget::new().scale(1.05)),
                    while_tap: Some(AnimationTarget::new().scale(0.95)),

                    "Add Task"
                }
            }
        }
    }
}
