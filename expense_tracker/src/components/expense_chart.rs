use charming::{
    component::{Axis, Legend, Title},
    element::{AxisType, Tooltip},
    series::{Bar, Line, Pie},
    Chart,
};
use dioxus::prelude::*;
use fastrand;

use crate::models::Category;

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseChartProps {
    pub total_amount: f64,
    pub category_totals: Vec<(Category, f64)>,
    pub on_category_click: EventHandler<Category>,
}

/// Chart component for displaying expense data
#[component]
pub fn ExpenseChart(props: ExpenseChartProps) -> Element {
    let chart_id = format!("expense-chart-{}", fastrand::u64(..));
    let chart_type = use_signal(|| "pie".to_string());

    // Create simple demo chart data
    let chart_data = {
        let line_chart = Chart::new()
            .tooltip(Tooltip::new().formatter("Value: {c}"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));
        line_chart.to_string()
    };

    let mount_code = format!(
        r#"
        (function() {{
            console.log('Initializing chart: {chart_id}');
            if (typeof echarts === 'undefined') {{
                console.error('ECharts library not loaded!');
                return;
            }}
            
            var chartDom = document.getElementById('{chart_id}');
            if (!chartDom) {{
                console.error('Chart container not found');
                return;
            }}
            
            var chart = echarts.init(chartDom);
            var option = {chart_data};
            option.animation = false;
            chart.setOption(option);
        }})();
        "#,
        chart_id = chart_id,
        chart_data = chart_data
    );

    rsx! {
        // Load ECharts library with correct file name
        document::Script { src: "/assets/echarts-5.5.1.min.js" }
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 border border-gray-200 dark:border-gray-700",
            h2 { class: "text-xl font-bold text-gray-800 dark:text-white mb-4", "Expense Chart (Demo)" }

            div {
                id: "{chart_id}",
                class: "w-full h-[300px]",
                onmounted: move |_| {
                    document::eval(&mount_code);
                }
            }
        }
    }
}
