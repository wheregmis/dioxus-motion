use charming::{
    component::{Axis, Legend, Title},
    element::{ItemStyle, Tooltip},
    series::{Bar, Pie, PieRoseType},
    Chart, HtmlRenderer,
};
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::models::Category;

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseChartProps {
    total_amount: f64,
    category_totals: Vec<(Category, f64)>,
    on_category_click: EventHandler<Category>,
}

#[component]
pub fn ExpenseChart(props: ExpenseChartProps) -> Element {
    let mut chart_opacity = use_motion(0.0f32);
    let mut chart_type = use_signal(|| "pie".to_string());

    // Animate the chart entry
    use_effect(move || {
        chart_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    });

    // Debug log for chart type
    tracing::info!("Current chart type: {}", chart_type.read());

    // Create chart data
    let chart_data = {
        let mut data = Vec::new();
        let mut categories = Vec::new();
        let mut values = Vec::new();

        for (category, amount) in &props.category_totals {
            data.push((*amount, category.display_name()));
            categories.push(category.display_name());
            values.push(*amount);
        }

        (data, categories, values)
    };

    // Create pie chart
    let pie_chart = {
        let (data, _, _) = &chart_data;

        let chart = Chart::new()
            .title(Title::new().text("Expense by Category").left("center"))
            .tooltip(Tooltip::new())
            .legend(Legend::new().top("bottom").left("center"))
            .series(
                Pie::new()
                    .name("Amount")
                    .radius(vec!["40%", "70%"])
                    .center(vec!["50%", "50%"])
                    .item_style(ItemStyle::new().border_radius(8))
                    .data(data.clone()),
            );

        let renderer = HtmlRenderer::new("expense-chart", 400, 300);
        renderer.render(&chart).unwrap_or_default()
    };

    // Create bar chart
    let bar_chart = {
        let (_, categories, values) = &chart_data;

        let chart = Chart::new()
            .title(Title::new().text("Expense by Category").left("center"))
            .tooltip(Tooltip::new())
            .legend(Legend::new().top("bottom").left("center"))
            .x_axis(Axis::new().data(categories.clone()))
            .y_axis(Axis::new())
            .series(
                Bar::new()
                    .name("Amount")
                    .data(values.clone())
                    .item_style(ItemStyle::new().border_radius(4)),
            );

        let renderer = HtmlRenderer::new("expense-chart", 400, 300);
        renderer.render(&chart).unwrap_or_default()
    };

    // Create rose chart
    let rose_chart = {
        let (data, _, _) = &chart_data;

        let chart = Chart::new()
            .title(Title::new().text("Expense by Category").left("center"))
            .tooltip(Tooltip::new())
            .legend(Legend::new().top("bottom").left("center"))
            .series(
                Pie::new()
                    .name("Amount")
                    .radius(vec!["30%", "80%"])
                    .center(vec!["50%", "50%"])
                    .rose_type(PieRoseType::Area)
                    .item_style(ItemStyle::new().border_radius(8))
                    .data(data.clone()),
            );

        let renderer = HtmlRenderer::new("expense-chart", 400, 300);
        renderer.render(&chart).unwrap_or_default()
    };

    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 border border-gray-200 dark:border-gray-700",
            style: "opacity: {chart_opacity.get_value()}",

            div { class: "flex justify-between items-center mb-4",

                h2 { class: "text-xl font-bold text-gray-800 dark:text-white", "Expense Chart" }

                div { class: "flex space-x-2",

                    button {
                        class: if *chart_type.read() == "pie" { "px-2 py-1 text-sm rounded bg-blue-600 text-white" } else { "px-2 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300" },
                        onclick: move |_| chart_type.set("pie".to_string()),
                        "Pie"
                    }

                    button {
                        class: if *chart_type.read() == "bar" { "px-2 py-1 text-sm rounded bg-blue-600 text-white" } else { "px-2 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300" },
                        onclick: move |_| chart_type.set("bar".to_string()),
                        "Bar"
                    }

                    button {
                        class: if *chart_type.read() == "rose" { "px-2 py-1 text-sm rounded bg-blue-600 text-white" } else { "px-2 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300" },
                        onclick: move |_| chart_type.set("rose".to_string()),
                        "Rose"
                    }
                }
            }

            if props.category_totals.is_empty() {
                div { class: "p-8 text-center text-gray-500 dark:text-gray-400",

                    p { class: "text-lg", "No expense data available for chart" }
                }
            } else {
                div { class: "w-full h-[300px] relative",

                    // Render the appropriate chart based on the selected type
                    div {
                        class: if *chart_type.read() != "pie" { "w-full h-full hidden" } else { "w-full h-full" },
                        dangerous_inner_html: "{pie_chart}",
                    }

                    div {
                        class: if *chart_type.read() != "bar" { "w-full h-full hidden" } else { "w-full h-full" },
                        dangerous_inner_html: "{bar_chart}",
                    }

                    div {
                        class: if *chart_type.read() != "rose" { "w-full h-full hidden" } else { "w-full h-full" },
                        dangerous_inner_html: "{rose_chart}",
                    }
                }
            }
        }
    }
}
