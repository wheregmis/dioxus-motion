use dioxus::prelude::*;
use dioxus_motion::prelude::*;

// Configure different transition settings based on platform
#[cfg(feature = "web")]
const STIFFNESS: f32 = 200.0;
#[cfg(feature = "web")]
const DAMPING: f32 = 20.0;

#[cfg(not(feature = "web"))]
const STIFFNESS: f32 = 300.0;
#[cfg(not(feature = "web"))]
const DAMPING: f32 = 30.0;

fn main() {
    launch(app);
}

// Dashboard components
#[component]
fn DashboardStatsCard() -> Element {
    // Animated stats card with hover effect
    rsx! {
        motion::div {
            class: "stats-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            // Global attributes for accessibility and testing
            role: "region",
            aria_label: "Total users statistics",
            "data-testid": "stats-card-users",
            tabindex: 0,

            h3 { class: "stats-title", "Total Users" }
            div { class: "stats-value", "24,532" }
            div { class: "stats-change positive", "+12% from last month" }
        }

        motion::div {
            class: "stats-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.1),
            ),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            h3 { class: "stats-title", "Revenue" }
            div { class: "stats-value", "$48,271" }
            div { class: "stats-change positive", "+8% from last month" }
        }

        motion::div {
            class: "stats-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.2),
            ),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            h3 { class: "stats-title", "Active Projects" }
            div { class: "stats-value", "16" }
            div { class: "stats-change negative", "-2 from last month" }
        }
    }
}

#[component]
fn DashboardActivityChart() -> Element {
    // Animated chart bars
    let bar_heights = [65, 40, 85, 30, 55, 60, 45];
    let bars = bar_heights.iter().enumerate().map(|(i, &height)| {
        let delay = i as f32 * 0.05;
        rsx! {
            motion::div {
                key: "bar-{i}",
                class: "chart-bar",
                style: "height: {height}%",
                initial: Some(AnimationTarget::new().scale(0.0).opacity(0.0)),
                animate: Some(AnimationTarget::new().scale(1.0).opacity(1.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(delay),
                ),
            }
        }
    });

    rsx! {
        motion::div {
            class: "chart-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.3),
            ),
            while_hover: Some(AnimationTarget::new().scale(1.01)),

            // Global attributes for accessibility and testing
            role: "figure",
            aria_label: "Weekly activity chart showing data for each day",
            "data-testid": "weekly-activity-chart",
            tabindex: 0,

            h3 { class: "chart-title", "Weekly Activity" }
            div { class: "chart-container", {bars} }
            div { class: "chart-labels",
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

#[derive(Clone, Debug)]
struct ActivityItem {
    id: usize,
    title: String,
    desc: String,
    time: String,
    position: (f32, f32), // Current position (x, y)
    is_dragging: bool,    // Whether this item is being dragged
    is_drop_target: bool, // Whether this item is a potential drop target
}

#[component]
fn DashboardRecentActivity() -> Element {
    // Activity items with staggered animation
    let mut activities = use_signal(|| {
        vec![
            ActivityItem {
                id: 0,
                title: "User signup".to_string(),
                desc: "John Smith created an account".to_string(),
                time: "10 min ago".to_string(),
                position: (0.0, 0.0),
                is_dragging: false,
                is_drop_target: false,
            },
            ActivityItem {
                id: 1,
                title: "New order".to_string(),
                desc: "Order #12345 was placed".to_string(),
                time: "25 min ago".to_string(),
                position: (0.0, 0.0),
                is_dragging: false,
                is_drop_target: false,
            },
            ActivityItem {
                id: 2,
                title: "Payment received".to_string(),
                desc: "Payment for order #12342 received".to_string(),
                time: "1 hour ago".to_string(),
                position: (0.0, 0.0),
                is_dragging: false,
                is_drop_target: false,
            },
            ActivityItem {
                id: 3,
                title: "Support ticket".to_string(),
                desc: "New support ticket #987 opened".to_string(),
                time: "2 hours ago".to_string(),
                position: (0.0, 0.0),
                is_dragging: false,
                is_drop_target: false,
            },
        ]
    });

    // Track which item is being dragged and drag state
    let mut dragging_item_id = use_signal(|| None::<usize>);
    let mut drag_start_pos = use_signal(|| (0.0, 0.0));
    let mut drag_current_pos = use_signal(|| (0.0, 0.0));

    // We'll implement drag and drop directly in the event handlers

    // Create a collection of activity item elements
    let mut activity_items = Vec::new();

    // Iterate through activities to create the elements
    for (i, item) in activities.read().iter().enumerate() {
        let delay = 0.4 + (i as f32 * 0.1);
        let id = item.id;
        let is_dragging = item.is_dragging;
        let is_drop_target = item.is_drop_target;
        let (offset_x, offset_y) = item.position;

        activity_items.push(rsx! {
            div {
                key: "activity-container-{id}",
                class: if is_dragging { "activity-item-container dragging" } else { "activity-item-container" },
                "data-is-drop-target": is_drop_target.to_string(),
                // Remove onmousedown from the container div

                motion::div {
                    key: "activity-{id}",
                    class: "activity-item",
                    initial: Some(AnimationTarget::new().opacity(0.0).x(-10.0).y(5.0).scale(0.98)),
                    // Set all properties explicitly to ensure proper reset
                    animate: Some(
                        AnimationTarget::new()
                            .opacity(1.0)
                            .x(offset_x)
                            .y(offset_y)
                            .scale(if is_dragging { 1.03 } else { 1.0 })
                            .rotate(0.0)
                            .background_color(if is_dragging { "#e6f0ff" } else { "transparent" }),
                    ),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(if is_dragging { 100.0 } else { 200.0 })
                            .damping(if is_dragging { 10.0 } else { 18.0 })
                            .mass(if is_dragging { 0.3 } else { 0.2 })
                            .velocity(0.0)
                            .delay(if is_dragging { 0.0 } else { delay }),
                    ),
                    while_hover: Some(AnimationTarget::new().background_color("#f5f9ff").scale(1.005)),

                    // Drag handle visual indicator
                    div {
                        class: "drag-handle",
                        onmousedown: move |e: Event<MouseData>| {
                            let coords = e.data().client_coordinates();
                            dragging_item_id.set(Some(id));
                            drag_start_pos.set((coords.x as f32, coords.y as f32));
                            drag_current_pos.set((coords.x as f32, coords.y as f32));
                            activities
                                .with_mut(|items| {
                                    if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                        item.is_dragging = true;
                                    }
                                });
                            e.stop_propagation();
                        },

                        // Six-dot drag handle icon
                        div { class: "drag-dots",
                            div { class: "dot" }
                            div { class: "dot" }
                            div { class: "dot" }
                            div { class: "dot" }
                            div { class: "dot" }
                            div { class: "dot" }
                        }
                    }

                    div { class: "activity-content",
                        div { class: "activity-title", "{item.title}" }
                        div { class: "activity-desc", "{item.desc}" }
                    }
                    div { class: "activity-time", "{item.time}" }
                }
            }
        });
    }

    // Global mouse event handlers
    let mousemove = {
        let dragging_item_id = dragging_item_id;
        let drag_start_pos = drag_start_pos;
        let mut drag_current_pos = drag_current_pos;
        let mut activities = activities;

        move |e: Event<MouseData>| {
            if let Some(id) = dragging_item_id() {
                let coords = e.data().client_coordinates();
                let x = coords.x as f32;
                let y = coords.y as f32;

                drag_current_pos.set((x, y));

                // Calculate the delta from start position
                let (start_x, start_y) = drag_start_pos();
                let delta_x = x - start_x;
                let delta_y = y - start_y;

                // Limit the horizontal movement to a small range to prevent items from going off-screen
                let constrained_delta_x = delta_x.clamp(-20.0, 20.0);

                // Update the position of the dragged item
                activities.with_mut(|items| {
                    // First, reset all drop targets
                    for item in items.iter_mut() {
                        item.is_drop_target = false;
                    }

                    // Update the dragged item's position
                    if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                        item.position = (constrained_delta_x, delta_y);
                    }

                    // Calculate which item would be the drop target
                    let dragged_index = items.iter().position(|item| item.id == id).unwrap();

                    // Get the positions of all items to determine where we're hovering
                    let item_positions: Vec<(usize, f32, f32)> = items
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| {
                            // Calculate the approximate y position of each item
                            // Each item is about 60px tall plus 12px gap
                            let y_pos = idx as f32 * 72.0;
                            (idx, y_pos, y_pos + 60.0)
                        })
                        .collect();

                    // Find which item we're hovering over based on the current mouse position
                    let mouse_y = start_y + delta_y;
                    let mut target_index = dragged_index;

                    for (idx, top, bottom) in item_positions {
                        if idx != dragged_index && mouse_y >= top && mouse_y <= bottom {
                            target_index = idx;
                            break;
                        }
                    }

                    // Mark the target item as a drop target if it's different from the dragged item
                    if target_index != dragged_index {
                        items[target_index].is_drop_target = true;
                        println!("Drop target: {}", target_index);
                    }
                });

                e.stop_propagation();
            }
        }
    };

    let mouseup = {
        let mut dragging_item_id = dragging_item_id;
        let drag_start_pos = drag_start_pos;
        let drag_current_pos = drag_current_pos;
        let mut activities = activities;

        move |e: Event<MouseData>| {
            if let Some(dragged_id) = dragging_item_id() {
                // Get the current drag position
                let (_, delta_y) = drag_current_pos();
                let (_, start_y) = drag_start_pos();
                let drag_distance = delta_y - start_y;

                // Estimate how many positions to move (each item is ~70px tall with gap)
                // Using a smaller value for more precise control
                let item_height = 60.0;
                let positions_to_move = (drag_distance / item_height).round() as isize;

                // Debug output
                println!(
                    "Drag distance: {}, Positions to move: {}",
                    drag_distance, positions_to_move
                );

                // Reset drag state immediately to prevent further movement
                dragging_item_id.set(None);

                // Get the current state of items
                let dragged_index;
                let target_index;

                {
                    let items = activities.read();
                    dragged_index = items.iter().position(|item| item.id == dragged_id).unwrap();

                    // Find the drop target (if any)
                    target_index = items
                        .iter()
                        .position(|item| item.is_drop_target)
                        .unwrap_or(dragged_index);
                }

                println!(
                    "Dragged index: {}, Target index: {}",
                    dragged_index, target_index
                );

                // First, reset all positions and states
                activities.with_mut(|items| {
                    for item in items.iter_mut() {
                        item.position = (0.0, 0.0);
                        item.is_dragging = false;
                        item.is_drop_target = false;
                    }
                });

                // Reorder items if position changed
                if target_index != dragged_index {
                    activities.with_mut(|items| {
                        // Remove the item from its current position
                        let item = items.remove(dragged_index);

                        // Insert it at the target position
                        items.insert(target_index, item);
                    });
                }

                e.stop_propagation();
            }
        }
    };

    rsx! {
        div {
            class: "activity-card-container",
            // Global mouse event handlers
            onmousemove: mousemove,
            onmouseup: mouseup,
            onmouseleave: mouseup,

            motion::div {
                class: "activity-card",
                initial: Some(AnimationTarget::new().opacity(0.0).y(15.0).scale(0.98)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0).scale(1.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(300.0)
                        .damping(26.0)
                        .mass(0.8)
                        .delay(0.2),
                ),

                h3 { class: "activity-card-title", "Recent Activity (Drag to Reorder)" }
                div { class: "activity-list", {activity_items.into_iter()} }
            }
        }
    }
}

#[component]
fn DashboardTasks() -> Element {
    // Task list with interactive animations
    let tasks = use_signal(|| {
        vec![
            ("Finalize project proposal", false),
            ("Review client feedback", true),
            ("Update documentation", false),
            ("Schedule team meeting", false),
        ]
    });

    let tasks_data = tasks.read();
    let task_items = tasks_data
        .iter()
        .enumerate()
        .map(|(i, &(task, completed))| {
            let delay = 0.5 + (i as f32 * 0.1);
            let i_clone = i;
            let mut tasks_clone = tasks;

            rsx! {
                motion::div {
                    key: "task-{i}",
                    class: if completed { "task-item completed" } else { "task-item" },
                    initial: Some(AnimationTarget::new().opacity(0.0).y(10.0)),
                    // Set all properties explicitly to ensure proper reset
                    animate: Some(AnimationTarget::new().opacity(1.0).x(0.0).y(0.0).scale(1.0).rotate(0.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(delay),
                    ),
                    while_hover: Some(AnimationTarget::new().x(5.0)),

                    div { class: "task-checkbox",
                        input {
                            r#type: "checkbox",
                            checked: completed,
                            onchange: move |_| {
                                tasks_clone
                                    .with_mut(|t| {
                                        t[i_clone].1 = !t[i_clone].1;
                                    });
                            },
                        }
                    }
                    div { class: "task-name", "{task}" }
                }
            }
        });

    rsx! {
        motion::div {
            class: "tasks-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.4),
            ),

            h3 { class: "tasks-title", "Tasks" }
            div { class: "tasks-list", {task_items} }

            motion::button {
                class: "add-task-btn",
                animate: Some(AnimationTarget::new().scale(1.0).background_color("#3b82f6")), // Explicitly set default state
                while_hover: Some(AnimationTarget::new().scale(1.05).background_color("#0070f3")),
                while_tap: Some(AnimationTarget::new().scale(0.95)),

                // Global attributes for accessibility and testing
                disabled: false,
                title: "Add a new task",
                "data-testid": "add-task-button",
                aria_label: "Add a new task to your list",

                "+ Add Task"
            }
        }
    }
}

fn app() -> Element {
    // State for sidebar collapse
    let mut sidebar_collapsed = use_signal(|| false);

    // State for active section
    let mut active_section = use_signal(|| "dashboard".to_string());

    rsx! {
        div { class: "app-container",
            // Sidebar
            div { class: if sidebar_collapsed() { "sidebar collapsed" } else { "sidebar" },

                // Logo and collapse button
                div { class: "sidebar-header",
                    // Logo with icon for collapsed state
                    div { class: "logo-container",
                        span { class: "logo-icon", "📊" }
                        h2 { class: "logo-text", "Motion Dashboard" }
                    }

                    motion::button {
                        class: "collapse-btn",
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
                nav { class: "sidebar-nav",
                    motion::div {
                        class: if active_section() == "dashboard" { "nav-item active" } else { "nav-item" },
                        onclick: move |_| active_section.set("dashboard".to_string()),
                        while_hover: Some(AnimationTarget::new().x(5.0)),

                        // Icon and text in separate spans for better control
                        span { class: "nav-icon", "📊" }
                        span { class: "nav-text", "Dashboard" }
                    }

                    motion::div {
                        class: if active_section() == "analytics" { "nav-item active" } else { "nav-item" },
                        onclick: move |_| active_section.set("analytics".to_string()),
                        while_hover: Some(AnimationTarget::new().x(5.0)),

                        span { class: "nav-icon", "📈" }
                        span { class: "nav-text", "Analytics" }
                    }

                    motion::div {
                        class: if active_section() == "reports" { "nav-item active" } else { "nav-item" },
                        onclick: move |_| active_section.set("reports".to_string()),
                        while_hover: Some(AnimationTarget::new().x(5.0)),

                        span { class: "nav-icon", "📝" }
                        span { class: "nav-text", "Reports" }
                    }

                    motion::div {
                        class: if active_section() == "settings" { "nav-item active" } else { "nav-item" },
                        onclick: move |_| active_section.set("settings".to_string()),
                        while_hover: Some(AnimationTarget::new().x(5.0)),

                        span { class: "nav-icon", "⚙️" }
                        span { class: "nav-text", "Settings" }
                    }
                }
            }

            // Main content area
            div { class: "main-content",
                // Header
                div { class: "header",
                    h1 {
                        if active_section() == "dashboard" {
                            "Dashboard"
                        } else if active_section() == "analytics" {
                            "Analytics"
                        } else if active_section() == "reports" {
                            "Reports"
                        } else {
                            "Settings"
                        }
                    }
                }

                // Content area - will add more components here
                // Content area with dashboard components
                div { class: "content",
                    // Only show content for the active section
                    if active_section() == "dashboard" {
                        // Dashboard overview
                        div { class: "dashboard-grid",
                            // Stats cards
                            DashboardStatsCard {}

                            // Activity chart
                            DashboardActivityChart {}

                            // Recent activity list
                            DashboardRecentActivity {}

                            // Tasks card
                            DashboardTasks {}
                        }
                    } else if active_section() == "analytics" {
                        div { class: "analytics-content",
                            h2 { "Analytics Content" }
                            p { "This is the analytics section." }
                        }
                    } else if active_section() == "reports" {
                        div { class: "reports-content",
                            h2 { "Reports Content" }
                            p { "This is the reports section." }
                        }
                    } else {
                        div { class: "settings-content",
                            h2 { "Settings Content" }
                            p { "This is the settings section." }
                        }
                    }
                }
            }

            // Basic styles
            style {
                "
                * {{ margin: 0; padding: 0; box-sizing: border-box; }}
                body {{ font-family: system-ui, -apple-system, sans-serif; background-color: #f5f7fa; color: #333; }}

                .app-container {{ display: flex; min-height: 100vh; }}

                .sidebar {{ width: 250px; background-color: #2c3e50; color: white; padding: 1rem; transition: all 0.3s ease; }}
                .sidebar.collapsed {{ width: 80px; overflow: hidden; }}

                /* Logo styling */
                .sidebar-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }}
                .logo-container {{ display: flex; align-items: center; overflow: hidden; }}
                .logo-icon {{ font-size: 1.5rem; margin-right: 0.75rem; }}
                .logo-text {{ font-size: 1.2rem; white-space: nowrap; }}
                .sidebar.collapsed .logo-text {{ display: none; }}
                .collapse-btn {{ background: none; border: none; color: white; cursor: pointer; font-size: 1.2rem; }}

                /* Navigation styling */
                .sidebar-nav {{ display: flex; flex-direction: column; gap: 0.5rem; }}
                .nav-item {{ display: flex; align-items: center; padding: 0.75rem 1rem; border-radius: 4px; cursor: pointer; white-space: nowrap; }}
                .nav-icon {{ font-size: 1.2rem; margin-right: 0.75rem; display: inline-block; min-width: 1.5rem; text-align: center; }}
                .nav-text {{ transition: opacity 0.3s ease; }}
                .sidebar.collapsed .nav-item {{ justify-content: center; padding: 0.75rem 0.5rem; }}
                .sidebar.collapsed .nav-text {{ display: none; }}
                .sidebar.collapsed .nav-icon {{ margin-right: 0; }}
                .nav-item:hover {{ background-color: #34495e; }}
                .nav-item.active {{ background-color: #3498db; }}

                .main-content {{ flex: 1; padding: 1rem; }}
                .header {{ margin-bottom: 2rem; padding-bottom: 1rem; border-bottom: 1px solid #e1e5e9; }}
                .content {{ padding: 1rem; background-color: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }}

                /* Dashboard Grid Layout */
                .dashboard-grid {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; }}
                .chart-card, .activity-card, .tasks-card {{ grid-column: span 3; }}

                /* Stats Cards */
                .stats-card {{ background-color: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.05); margin-bottom: 1rem; }}
                .stats-title {{ font-size: 0.9rem; color: #64748b; margin-bottom: 0.5rem; }}
                .stats-value {{ font-size: 1.8rem; font-weight: bold; margin-bottom: 0.5rem; }}
                .stats-change {{ font-size: 0.8rem; }}
                .stats-change.positive {{ color: #10b981; }}
                .stats-change.negative {{ color: #ef4444; }}

                /* Chart */
                .chart-card {{ background-color: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.05); margin-bottom: 1rem; }}
                .chart-title {{ font-size: 1rem; margin-bottom: 1rem; }}
                .chart-container {{ display: flex; align-items: flex-end; height: 200px; gap: 10px; margin-bottom: 0.5rem; }}
                .chart-bar {{ background-color: #3b82f6; flex: 1; border-radius: 4px 4px 0 0; }}
                .chart-labels {{ display: flex; justify-content: space-between; }}
                .chart-labels div {{ font-size: 0.8rem; color: #64748b; flex: 1; text-align: center; }}

                /* Activity List */
                .activity-card-container {{ width: 100%; }}
                .activity-card {{ background-color: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.05); margin-bottom: 1rem; width: 100%; }}
                .activity-card-title {{ font-size: 1rem; margin-bottom: 1rem; display: flex; align-items: center; width: 100%; }}
                .activity-card-title::after {{ content: "
                "; flex: 1; margin-left: 1rem; height: 1px; background-color: #e5e7eb; }}
                .activity-list {{ display: flex; flex-direction: column; gap: 0.75rem; width: 100%; }}
                .activity-item-container {{ border-radius: 6px; transition: all 0.2s ease; position: relative; }}
                .activity-item-container:hover {{ background-color: #f9fafb; }}
                .activity-item-container.dragging {{ z-index: 100; position: relative; }}
                .activity-item {{ display: flex; align-items: center; padding: 0.85rem; border-radius: 6px; border-left: 3px solid #3b82f6; position: relative; background-color: white; box-shadow: 0 1px 3px rgba(0,0,0,0.05); transition: all 0.2s ease; width: 100%; }}
                .activity-item-container.dragging .activity-item {{
                    box-shadow: 0 8px 20px rgba(59, 130, 246, 0.2);
                    position: absolute;
                    width: calc(100% - 10px);
                    left: 5px;
                    top: 0;
                }}

                /* Drop target indicator */
                .activity-item-container::before {{ content: "
                "; position: absolute; left: 0; right: 0; height: 3px; background-color: transparent; top: -2px; border-radius: 3px; transition: all 0.2s ease; }}
                .activity-item-container::after {{ content: "
                "; position: absolute; left: 0; right: 0; height: 3px; background-color: transparent; bottom: -2px; border-radius: 3px; transition: all 0.2s ease; }}
                .activity-content {{ flex: 1; margin: 0 0.5rem; }}
                .activity-title {{ font-weight: 600; margin-bottom: 0.25rem; color: #1f2937; }}
                .activity-desc {{ font-size: 0.85rem; color: #64748b; }}
                .activity-time {{ font-size: 0.75rem; color: #94a3b8; padding-left: 0.5rem; border-left: 1px solid #e5e7eb; }}
                .drag-handle {{
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-right: 0.5rem;
                    width: 28px;
                    height: 28px;
                    border-radius: 4px;
                    transition: all 0.2s ease;
                    cursor: grab;
                    background-color: #f5f7fa;
                }}
                .drag-dots {{ display: grid; grid-template-columns: repeat(2, 1fr); grid-gap: 3px; width: 14px; height: 20px; }}
                .dot {{ width: 5px; height: 5px; border-radius: 50%; background-color: #cbd5e1; transition: all 0.2s ease; }}
                .drag-handle:hover {{ background-color: #e6f0ff; }}
                .drag-handle:hover .dot {{ background-color: #3b82f6; }}
                .activity-item-container.dragging .drag-handle {{ background-color: #e6f0ff; cursor: grabbing; }}
                .activity-item-container.dragging .dot {{ background-color: #3b82f6; }}

                /* Drop target styles */
                .activity-item-container[data-is-drop-target='true']::before {{ background-color: #3b82f6; height: 3px; }}
                .activity-item-container[data-is-drop-target='true'] {{ background-color: #f0f7ff; border: 1px dashed #3b82f6; border-radius: 6px; }}

                /* Tasks */
                .tasks-card {{ background-color: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.05); }}
                .tasks-title {{ font-size: 1rem; margin-bottom: 1rem; }}
                .tasks-list {{ display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem; }}
                .task-item {{ display: flex; align-items: center; padding: 0.75rem; border-radius: 4px; background-color: #f8fafc; }}
                .task-item.completed .task-name {{ text-decoration: line-through; color: #94a3b8; }}
                .task-checkbox {{ margin-right: 0.75rem; }}
                .task-name {{ font-size: 0.9rem; }}
                .add-task-btn {{ background-color: #3b82f6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; font-size: 0.9rem; }}
                "
            }
        }
    }
}
