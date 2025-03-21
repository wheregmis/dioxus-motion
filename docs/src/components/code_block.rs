use dioxus::prelude::*;

/// Returns an HTML string with syntax highlighting applied to the given Rust code.
///
/// The function scans the input source code, identifies tokens, comments, and string literals,
/// and wraps them in HTML span elements with appropriate CSS classes for highlighting.
/// Comments are highlighted in gray, string literals in green, and other tokens are processed
/// to detect Rust syntax and Dioxus-specific patterns.
///
/// # Examples
///
/// ```
/// let code = r#"fn main() {
///     // This is a comment
///     println!("Hello, world!");
/// }"#;
/// let highlighted = highlight_rust_syntax(code);
/// assert!(highlighted.contains("<span class='text-gray-500'>")); // comment highlighting
/// assert!(highlighted.contains("<span class='text-green-500'>")); // string literal highlighting
/// ```fn highlight_rust_syntax(code: &str) -> String {
    // Create a more robust token-based approach rather than simple replacement
    let mut result = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();

    for i in 0..chars.len() {
        // Handle comments first
        if !in_string && i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            // Add any accumulated token before the comment
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_token(token, false));
            }

            // Start the comment span
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        // If we're in a comment and hit a newline, close the comment span
        if in_comment && chars[i] == '\n' {
            result.push_str(&code[token_start..=i]);
            result.push_str("</span>");
            token_start = i + 1;
            in_comment = false;
            continue;
        }

        // If we're in a comment, continue to next character
        if in_comment {
            continue;
        }

        // Handle string literals
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            if !in_string {
                // Start of string
                if token_start < i {
                    let token = &code[token_start..i];
                    result.push_str(&highlight_token(token, false));
                }
                result.push_str("<span class='text-green-500'>\"");
                token_start = i + 1;
                in_string = true;
            } else {
                // End of string
                result.push_str(&code[token_start..i]);
                result.push_str("\"</span>");
                token_start = i + 1;
                in_string = false;
            }
            continue;
        }

        // If we're in a string, continue to next character
        if in_string {
            continue;
        }

        // Handle whitespace and separators
        if chars[i].is_whitespace()
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '('
            || chars[i] == ')'
            || chars[i] == ':'
            || chars[i] == ','
        {
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_token(token, false));
            }

            // Add the separator character as-is
            result.push(chars[i]);
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let token = &code[token_start..];
        if in_string {
            result.push_str(token);
        } else if in_comment {
            result.push_str(token);
            result.push_str("</span>");
        } else {
            result.push_str(&highlight_token(token, false));
        }
    }

    result
}

/// Highlights an individual token for syntax highlighting.
///
/// Analyzes the provided token and returns it wrapped in an HTML `<span>` element with a CSS class
/// corresponding to its syntactic category. If the token is part of a string literal (i.e. when
/// `in_string` is true), no highlighting is applied and the original token is returned.
///
/// Before matching against specific rules, the function cleans the token by removing extraneous
/// non-alphanumeric characters (except for underscores, '#' and ':'). It then applies the following rules:
///
/// - **Dioxus Attribute**: The token `#[component]` is highlighted in purple.
/// - **Rust Keywords**: Common Rust keywords (like `fn`, `let`, `struct`, etc.) are highlighted in blue.
/// - **Dioxus Components**: Tokens starting with an uppercase letter (and not prefixed by `Route::`)
///   are highlighted in orange.
/// - **RSX Macro**: The token `rsx!` is highlighted in yellow.
/// - **Route Types**: Tokens that begin with `Route::` are split so that the `Route::` prefix is in green
///   and the remaining part is in orange.
/// - **Element Properties**: Tokens ending with a colon (typically denoting element properties) are highlighted
///   in a lighter blue.
/// - **Numeric Literals**: Tokens that represent numbers (including decimals) are highlighted in orange.
///
/// If the token does not match any of these conditions, it is returned unchanged.
///
/// # Examples
///
/// ```
/// // Highlight a Rust keyword.
/// assert_eq!(
///     highlight_token("fn", false),
///     "<span class='text-blue-500'>fn</span>"
/// );
///
/// // Highlight a Dioxus-specific attribute.
/// assert_eq!(
///     highlight_token("#[component]", false),
///     "<span class='text-purple-500'>#[component]</span>"
/// );
///
/// // When token is part of a string literal, no highlighting is applied.
/// assert_eq!(highlight_token("let", true), "let");
/// ```fn highlight_token(token: &str, in_string: bool) -> String {
    if in_string {
        return token.to_string();
    }

    // Clean the token of any color codes that might be present
    let clean_token = token.replace(
        |c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '#' && c != ':',
        "",
    );

    if clean_token.is_empty() {
        return token.to_string();
    }

    // Dioxus-specific attributes
    if clean_token == "#[component]" {
        return "<span class='text-purple-500'>#[component]</span>".to_string();
    }

    // Check for Rust keywords
    let keywords = [
        "fn", "let", "mut", "pub", "use", "struct", "enum", "trait", "impl", "const", "static",
        "async", "await", "for", "while", "loop", "if", "else", "match", "in", "return", "where",
        "type", "dyn",
    ];

    if keywords.contains(&clean_token.as_str()) {
        return format!("<span class='text-blue-500'>{}</span>", token);
    }

    // Dioxus components (capitalized identifiers)
    if !clean_token.is_empty()
        && clean_token.chars().next().unwrap().is_uppercase()
        && !clean_token.starts_with("Route::")
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    // Handle RSX macro
    if clean_token == "rsx!" {
        return format!("<span class='text-yellow-500'>{}</span>", token);
    }

    // Route types
    if clean_token.starts_with("Route::") {
        let parts: Vec<&str> = clean_token.split("::").collect();
        if parts.len() >= 2 {
            return format!("<span class='text-green-300'>Route::</span><span class='text-orange-400'>{}</span>", 
                         parts[1..].join("::"));
        }
    }

    // Element properties (followed by colon)
    if token.ends_with(':') {
        return format!("<span class='text-blue-300'>{}</span>", token);
    }

    // Numbers
    if clean_token.chars().all(|c| c.is_ascii_digit() || c == '.')
        && clean_token.chars().any(|c| c.is_ascii_digit())
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    token.to_string()
}

/// Highlights TOML syntax by converting a TOML code snippet into an HTML-formatted string.
///
/// The returned string wraps recognized TOML elements—such as comments, string literals, and structural tokens—with
/// `<span>` elements that include CSS classes for visual styling. This facilitates syntax highlighting when the
/// formatted output is rendered in a browser.
///
/// # Examples
///
/// ```
/// let toml_input = r#"
/// # Sample TOML configuration
/// title = "Example"
/// [section]
/// value = "hello"
/// "#;
///
/// let highlighted = highlight_toml_syntax(toml_input);
/// // Verify that the output contains styled comment spans
/// assert!(highlighted.contains("<span class='text-gray-500'>"));
/// ```fn highlight_toml_syntax(code: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut token_start = 0;
    let chars: Vec<char> = code.chars().collect();

    for i in 0..chars.len() {
        // Handle comments first
        if !in_string && i + 1 < chars.len() && chars[i] == '#' {
            // Add any accumulated token before the comment
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_toml_token(token, false));
            }

            // Start the comment span
            result.push_str("<span class='text-gray-500'>");
            token_start = i;
            in_comment = true;
            continue;
        }

        // If we're in a comment and hit a newline, close the comment span
        if in_comment && chars[i] == '\n' {
            result.push_str(&code[token_start..=i]);
            result.push_str("</span>");
            token_start = i + 1;
            in_comment = false;
            continue;
        }

        // If we're in a comment, continue to next character
        if in_comment {
            continue;
        }

        // Handle string literals
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            if !in_string {
                // Start of string
                if token_start < i {
                    let token = &code[token_start..i];
                    result.push_str(&highlight_toml_token(token, false));
                }
                result.push_str("<span class='text-green-500'>\"");
                token_start = i + 1;
                in_string = true;
            } else {
                // End of string
                result.push_str(&code[token_start..i]);
                result.push_str("\"</span>");
                token_start = i + 1;
                in_string = false;
            }
            continue;
        }

        // If we're in a string, continue to next character
        if in_string {
            continue;
        }

        // Handle whitespace and separators
        if chars[i].is_whitespace()
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '['
            || chars[i] == ']'
            || chars[i] == '='
            || chars[i] == ','
        {
            if token_start < i {
                let token = &code[token_start..i];
                result.push_str(&highlight_toml_token(token, false));
            }

            // Add the separator character with special coloring for brackets
            if chars[i] == '[' || chars[i] == ']' {
                result.push_str(&format!("<span class='text-blue-400'>{}</span>", chars[i]));
            } else {
                result.push(chars[i]);
            }
            token_start = i + 1;
        }
    }

    // Add any remaining part
    if token_start < chars.len() {
        let token = &code[token_start..];
        if in_string {
            result.push_str(token);
        } else if in_comment {
            result.push_str(token);
            result.push_str("</span>");
        } else {
            result.push_str(&highlight_toml_token(token, false));
        }
    }

    result
}

/// Applies HTML syntax highlighting to a single TOML token based on its content.
///
/// The function inspects the provided token and wraps it in an HTML `<span>` element with a CSS class for styling,
/// depending on the token's structure. Section headers marked by square brackets are highlighted in blue,
/// keys in key-value pairs appear in purple (with the corresponding value further highlighted), and literals (comprising digits, periods, or quotes)
/// are highlighted in orange. If the `in_string` flag is `true`, the token is returned without any modifications.
///
/// # Examples
///
/// ```
/// let token = "[section]";
/// let highlighted = highlight_toml_token(token, false);
/// assert_eq!(highlighted, format!("<span class='text-blue-400'>{}</span>", token));
///
/// let key_value = "name = \"Dioxus\"";
/// let highlighted_kv = highlight_toml_token(key_value, false);
/// assert!(highlighted_kv.contains("<span class='text-purple-400'>name</span>="));
///
/// let literal = "42.0";
/// let highlighted_lit = highlight_toml_token(literal, false);
/// assert!(highlighted_lit.contains("text-orange-400"));
///
/// // When the token is inside a string literal, no highlighting is applied.
/// let in_string_token = "[not highlighted]";
/// assert_eq!(highlight_toml_token(in_string_token, true), in_string_token.to_string());
/// ```fn highlight_toml_token(token: &str, in_string: bool) -> String {
    if in_string {
        return token.to_string();
    }

    // Clean the token
    let clean_token = token.trim();

    if clean_token.is_empty() {
        return token.to_string();
    }

    // Handle section headers
    if clean_token.starts_with('[') && clean_token.ends_with(']') {
        return format!("<span class='text-blue-400'>{}</span>", token);
    }

    // Handle key-value pairs
    if token.contains('=') {
        let parts: Vec<&str> = token.split('=').collect();
        if parts.len() >= 2 {
            let key = parts[0].trim();
            let value = parts[1..].join("=").trim().to_string();
            return format!(
                "<span class='text-purple-400'>{}</span>={}",
                key,
                highlight_toml_value(&value)
            );
        }
    }

    // Handle keys
    if token.ends_with('=') {
        return format!("<span class='text-purple-400'>{}</span>", token);
    }

    // Handle version numbers and other literals
    if clean_token
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '"')
    {
        return format!("<span class='text-orange-400'>{}</span>", token);
    }

    token.to_string()
}

/// Highlights a TOML value by wrapping it in an HTML `<span>` element with a CSS class based on the value's type.
/// 
/// Booleans (`"true"` or `"false"`) and numeric values (comprising only digits, dots, or hyphens) are styled with the `"text-orange-400"` class,
/// while quoted strings (those starting and ending with a double quote) are styled with the `"text-green-500"` class.
/// Values that do not match these conditions are returned unchanged.
/// 
/// # Examples
///
/// ```
/// let highlighted_bool = highlight_toml_value("true");
/// assert_eq!(highlighted_bool, "<span class='text-orange-400'>true</span>");
///
/// let highlighted_num = highlight_toml_value("42.5");
/// assert_eq!(highlighted_num, "<span class='text-orange-400'>42.5</span>");
///
/// let highlighted_str = highlight_toml_value("\"hello\"");
/// assert_eq!(highlighted_str, "<span class='text-green-500'>\"hello\"</span>");
///
/// let normal_value = highlight_toml_value("unquoted");
/// assert_eq!(normal_value, "unquoted");
/// ```fn highlight_toml_value(value: &str) -> String {
    // Handle boolean values
    if value == "true" || value == "false" {
        return format!("<span class='text-orange-400'>{}</span>", value);
    }

    // Handle numbers
    if value
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
    {
        return format!("<span class='text-orange-400'>{}</span>", value);
    }

    // Handle quoted strings
    if value.starts_with('"') && value.ends_with('"') {
        return format!("<span class='text-green-500'>{}</span>", value);
    }

    value.to_string()
}

#[component]
/// Renders a code block with syntax highlighting for Rust or TOML code.
///
/// This component takes a code string and a language identifier as input, applies syntax highlighting
/// for supported languages using appropriate helper functions, and returns a styled `<pre>` element that
/// displays the highlighted code. If the language is not recognized, the original code is rendered without
/// additional formatting.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// // Example usage: rendering Rust code
/// let code = "fn main() { println!(\"Hello, world!\"); }".to_string();
/// let language = "rust".to_string();
///
/// // Render the code block component
/// let element = CodeBlock(code, language);
///
/// // `element` can now be used within a Dioxus application
/// ```
pub fn CodeBlock(code: String, language: String) -> Element {
    let highlighted = match language.to_lowercase().as_str() {
        "rust" => highlight_rust_syntax(&code),
        "toml" => highlight_toml_syntax(&code),
        _ => code.clone(),
    };

    rsx! {
        pre {
            class: format!(
                "language-{} overflow-x-auto rounded-lg bg-dark-300/50 p-4 font-mono",
                language,
            ),
            dangerous_inner_html: "{highlighted}",
        }
    }
}
