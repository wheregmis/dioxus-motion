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

#[component]
fn DashboardRecentActivity() -> Element {
    // Activity items with staggered animation
    let activities = [
        ("User signup", "John Smith created an account", "10 min ago"),
        ("New order", "Order #12345 was placed", "25 min ago"),
        (
            "Payment received",
            "Payment for order #12342 received",
            "1 hour ago",
        ),
        (
            "Support ticket",
            "New support ticket #987 opened",
            "2 hours ago",
        ),
    ];

    let activity_items = activities
        .iter()
        .enumerate()
        .map(|(i, &(title, desc, time))| {
            let delay = 0.4 + (i as f32 * 0.1);
            rsx! {
                motion::div {
                    key: "activity-{i}",
                    class: "activity-item",
                    initial: Some(AnimationTarget::new().opacity(0.0).x(-20.0)),
                    // Set all properties explicitly to ensure proper reset
                    animate: Some(
                        AnimationTarget::new()
                            .opacity(1.0)
                            .x(0.0)
                            .y(0.0)
                            .scale(1.0)
                            .rotate(0.0)
                            .background_color("transparent"),
                    ),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(delay),
                    ),
                    while_hover: Some(AnimationTarget::new().background_color("#f5f9ff")),

                    div { class: "activity-content",
                        div { class: "activity-title", "{title}" }
                        div { class: "activity-desc", "{desc}" }
                    }
                    div { class: "activity-time", "{time}" }
                }
            }
        });

    rsx! {
        motion::div {
            class: "activity-card",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.3),
            ),

            h3 { class: "activity-card-title", "Recent Activity" }
            div { class: "activity-list", {activity_items} }
        }
    }
}

#[component]
fn DashboardTasks() -> Element {
    // Task list with interactive animations
    let mut tasks = use_signal(|| {
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
                .activity-card {{ background-color: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.05); margin-bottom: 1rem; }}
                .activity-card-title {{ font-size: 1rem; margin-bottom: 1rem; }}
                .activity-list {{ display: flex; flex-direction: column; gap: 0.5rem; }}
                .activity-item {{ display: flex; justify-content: space-between; padding: 0.75rem; border-radius: 4px; border-left: 3px solid #3b82f6; }}
                .activity-content {{ flex: 1; }}
                .activity-title {{ font-weight: 500; margin-bottom: 0.25rem; }}
                .activity-desc {{ font-size: 0.85rem; color: #64748b; }}
                .activity-time {{ font-size: 0.75rem; color: #94a3b8; }}

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
