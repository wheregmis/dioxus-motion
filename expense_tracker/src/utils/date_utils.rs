use chrono::{Datelike, Local, NaiveDate};

/// Formats a date as a human-readable string (e.g., "April 15, 2023")
pub fn format_date(date: NaiveDate) -> String {
    date.format("%B %d, %Y").to_string()
}

/// Formats a date as a short string (e.g., "Apr 15, 2023")
pub fn format_date_short(date: NaiveDate) -> String {
    date.format("%b %d, %Y").to_string()
}

/// Formats a date as a compact string (e.g., "04/15/2023")
pub fn format_date_compact(date: NaiveDate) -> String {
    date.format("%m/%d/%Y").to_string()
}

/// Gets the first day of the current month
pub fn first_day_of_current_month() -> NaiveDate {
    let now = Local::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap()
}

/// Gets the last day of the current month
pub fn last_day_of_current_month() -> NaiveDate {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    
    // Get the first day of the next month
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    
    let first_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    
    // Subtract one day to get the last day of the current month
    first_of_next_month.pred_opt().unwrap()
}

/// Gets the first day of a specific month
pub fn first_day_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, 1).unwrap()
}

/// Gets the last day of a specific month
pub fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    // Get the first day of the next month
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    
    let first_of_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    
    // Subtract one day to get the last day of the current month
    first_of_next_month.pred_opt().unwrap()
}

/// Gets a list of month names
pub fn month_names() -> Vec<&'static str> {
    vec![
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ]
}

/// Gets the month name from a month number (1-12)
pub fn month_name(month: u32) -> &'static str {
    let names = month_names();
    names[(month as usize) - 1]
}

/// Parses a date string in the format "YYYY-MM-DD"
pub fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}

/// Gets today's date
pub fn today() -> NaiveDate {
    Local::now().date_naive()
}
