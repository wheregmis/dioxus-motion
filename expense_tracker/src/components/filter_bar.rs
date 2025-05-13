use chrono::{Datelike, Local};
use dioxus::prelude::*;

use crate::context::FilterType;
use crate::models::Category;
use crate::utils::{first_day_of_month, last_day_of_month, month_names, parse_currency};

#[derive(Props, Clone, PartialEq)]
pub struct FilterBarProps {
    current_filter: FilterType,
    on_filter_change: EventHandler<FilterType>,
    on_clear_filter: EventHandler<()>,
}

#[component]
pub fn FilterBar(props: FilterBarProps) -> Element {
    let mut show_date_filter =
        use_signal(|| matches!(props.current_filter, FilterType::DateRange(_, _)));
    let mut show_category_filter =
        use_signal(|| matches!(props.current_filter, FilterType::Category(_)));
    let mut show_amount_filter =
        use_signal(|| matches!(props.current_filter, FilterType::AmountRange(_, _)));

    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();

    let mut selected_year = use_signal(|| {
        if let FilterType::DateRange(start_date, _) = props.current_filter {
            start_date.year()
        } else {
            current_year
        }
    });

    let mut selected_month = use_signal(|| {
        if let FilterType::DateRange(start_date, _) = props.current_filter {
            start_date.month()
        } else {
            current_month
        }
    });

    let mut selected_category = use_signal(|| {
        if let FilterType::Category(category) = &props.current_filter {
            category.clone()
        } else {
            Category::Food
        }
    });

    let mut min_amount = use_signal(|| {
        if let FilterType::AmountRange(min, _) = props.current_filter {
            format!("{:.2}", min)
        } else {
            "0.00".to_string()
        }
    });

    let mut max_amount = use_signal(|| {
        if let FilterType::AmountRange(_, max) = props.current_filter {
            format!("{:.2}", max)
        } else {
            "1000.00".to_string()
        }
    });

    let handle_filter_type_change = move |evt: Event<FormData>| match evt.value().as_str() {
        "date" => {
            show_date_filter.set(true);
            show_category_filter.set(false);
            show_amount_filter.set(false);

            let start_date = first_day_of_month(*selected_year.read(), *selected_month.read());
            let end_date = last_day_of_month(*selected_year.read(), *selected_month.read());
            props
                .on_filter_change
                .call(FilterType::DateRange(start_date, end_date));
        }
        "category" => {
            show_date_filter.set(false);
            show_category_filter.set(true);
            show_amount_filter.set(false);

            props
                .on_filter_change
                .call(FilterType::Category(selected_category.read().clone()));
        }
        "amount" => {
            show_date_filter.set(false);
            show_category_filter.set(false);
            show_amount_filter.set(true);

            let min = parse_currency(&min_amount.read()).unwrap_or(0.0);
            let max = parse_currency(&max_amount.read()).unwrap_or(1000.0);
            props
                .on_filter_change
                .call(FilterType::AmountRange(min, max));
        }
        _ => {
            show_date_filter.set(false);
            show_category_filter.set(false);
            show_amount_filter.set(false);
            props.on_clear_filter.call(());
        }
    };

    let handle_year_change = move |evt: Event<FormData>| {
        if let Ok(year) = evt.value().parse::<i32>() {
            selected_year.set(year);

            let start_date = first_day_of_month(year, *selected_month.read());
            let end_date = last_day_of_month(year, *selected_month.read());
            props
                .on_filter_change
                .call(FilterType::DateRange(start_date, end_date));
        }
    };

    let handle_month_change = move |evt: Event<FormData>| {
        if let Ok(month) = evt.value().parse::<u32>() {
            selected_month.set(month);

            let start_date = first_day_of_month(*selected_year.read(), month);
            let end_date = last_day_of_month(*selected_year.read(), month);
            props
                .on_filter_change
                .call(FilterType::DateRange(start_date, end_date));
        }
    };

    let handle_category_change = move |evt: Event<FormData>| {
        let category = Category::from_string(&evt.value());
        selected_category.set(category.clone());
        props.on_filter_change.call(FilterType::Category(category));
    };

    let handle_min_amount_change = move |evt: Event<FormData>| {
        min_amount.set(evt.value().clone());

        if let (Some(min), Some(max)) = (
            parse_currency(&min_amount.read()),
            parse_currency(&max_amount.read()),
        ) {
            props
                .on_filter_change
                .call(FilterType::AmountRange(min, max));
        }
    };

    let handle_max_amount_change = move |evt: Event<FormData>| {
        max_amount.set(evt.value().clone());

        if let (Some(min), Some(max)) = (
            parse_currency(&min_amount.read()),
            parse_currency(&max_amount.read()),
        ) {
            props
                .on_filter_change
                .call(FilterType::AmountRange(min, max));
        }
    };

    let handle_clear_filter = move |_| {
        show_date_filter.set(false);
        show_category_filter.set(false);
        show_amount_filter.set(false);
        props.on_clear_filter.call(());
    };

    let filter_type = match props.current_filter {
        FilterType::None => "none",
        FilterType::DateRange(_, _) => "date",
        FilterType::Category(_) => "category",
        FilterType::AmountRange(_, _) => "amount",
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm p-4 mb-6 border border-gray-200 dark:border-gray-700",

            div { class: "flex flex-wrap items-center justify-between gap-4",

                div { class: "flex-1 min-w-[200px]",

                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                        "Filter by"
                    }

                    select {
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                        value: "{filter_type}",
                        oninput: handle_filter_type_change,

                        option { value: "none", "No filter" }
                        option { value: "date", "Date" }
                        option { value: "category", "Category" }
                        option { value: "amount", "Amount" }
                    }
                }

                if *show_date_filter.read() {
                    div { class: "flex flex-wrap gap-4 flex-1 min-w-[300px]",

                        div { class: "w-1/2 min-w-[120px]",

                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Year"
                            }

                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                value: "{selected_year}",
                                oninput: handle_year_change,

                                for year in (current_year - 5)..=(current_year) {
                                    option { value: "{year}", "{year}" }
                                }
                            }
                        }

                        div { class: "w-1/2 min-w-[120px]",

                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Month"
                            }

                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                value: "{selected_month}",
                                oninput: handle_month_change,

                                for (i , month) in month_names().iter().enumerate() {
                                    option { value: "{i + 1}", "{month}" }
                                }
                            }
                        }
                    }
                }

                if *show_category_filter.read() {
                    div { class: "flex-1 min-w-[200px]",

                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                            "Category"
                        }

                        select {
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                            value: "{selected_category.read().display_name()}",
                            oninput: handle_category_change,

                            for cat in Category::all() {
                                option { value: "{cat.display_name()}", "{cat.display_name()}" }
                            }
                        }
                    }
                }

                if *show_amount_filter.read() {
                    div { class: "flex flex-wrap gap-4 flex-1 min-w-[300px]",

                        div { class: "w-1/2 min-w-[120px]",

                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Min Amount"
                            }

                            input {
                                r#type: "text",
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                placeholder: "0.00",
                                value: "{min_amount}",
                                oninput: handle_min_amount_change,
                            }
                        }

                        div { class: "w-1/2 min-w-[120px]",

                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                "Max Amount"
                            }

                            input {
                                r#type: "text",
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                                placeholder: "1000.00",
                                value: "{max_amount}",
                                oninput: handle_max_amount_change,
                            }
                        }
                    }
                }

                if props.current_filter != FilterType::None {
                    button {
                        class: "px-4 py-2 text-sm text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300",
                        onclick: handle_clear_filter,

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-4 w-4 inline-block mr-1",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }

                        "Clear Filter"
                    }
                }
            }
        }
    }
}
