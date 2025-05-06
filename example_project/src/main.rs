use dioxus::prelude::*;
use dioxus_motion::prelude::*;

mod components;
mod tailwind;

use components::*;
use tailwind::include_tailwind_stylesheet;

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

fn app() -> Element {
    // State for sidebar collapse
    let mut sidebar_collapsed = use_signal(|| false);

    // State for active section
    let mut active_section = use_signal(|| "dashboard".to_string());

    rsx! {
        // Include Tailwind CSS stylesheet
        {include_tailwind_stylesheet()}
        
        div { class: "flex h-screen bg-gray-100 dark:bg-gray-900",
            // Sidebar component
            Sidebar { sidebar_collapsed: sidebar_collapsed, active_section: active_section }

            // Main content area
            div { class: "flex-1 flex flex-col overflow-hidden",
                // Header component
                Header { active_section: active_section }

                // Content area with components
                div { class: "flex-1 overflow-y-auto p-6",
                    // Only show content for the active section
                    if active_section() == "dashboard" {
                        // Dashboard overview
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
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
                        // Analytics page component
                        AnalyticsPage {}
                    } else if active_section() == "reports" {
                        // Reports page component
                        ReportsPage {}
                    } else {
                        // Settings page component
                        SettingsPage {}
                    }
                }
            }
        }
    }
}
