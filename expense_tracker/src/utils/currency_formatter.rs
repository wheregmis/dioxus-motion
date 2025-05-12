/// Formats a number as a currency string (e.g., "$123.45")
pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

/// Formats a number as a currency string with thousands separators (e.g., "$1,234.56")
pub fn format_currency_with_commas(amount: f64) -> String {
    let mut result = String::new();
    let amount_str = format!("{:.2}", amount);
    let parts: Vec<&str> = amount_str.split('.').collect();
    
    let integer_part = parts[0];
    let decimal_part = parts[1];
    
    // Add thousands separators to the integer part
    let mut chars = integer_part.chars().rev().collect::<Vec<_>>();
    for i in 1..chars.len() {
        if i % 3 == 0 {
            chars.insert(i, ',');
        }
    }
    
    let formatted_integer = chars.iter().rev().collect::<String>();
    result.push_str("$");
    result.push_str(&formatted_integer);
    result.push_str(".");
    result.push_str(decimal_part);
    
    result
}

/// Parses a currency string (e.g., "$123.45" or "123.45") into a float
pub fn parse_currency(currency_str: &str) -> Option<f64> {
    // Remove currency symbol, commas, and whitespace
    let cleaned = currency_str
        .replace('$', "")
        .replace(',', "")
        .trim()
        .to_string();
    
    // Parse as float
    cleaned.parse::<f64>().ok()
}
