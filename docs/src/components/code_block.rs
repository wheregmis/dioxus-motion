use dioxus::prelude::*;

/// Highlights Rust source code by applying HTML spans for syntax elements such as comments and strings.
///
/// This function processes the provided Rust code, identifying comments (e.g. starting with `//`),
/// string literals (enclosed in unescaped `"` characters), and token separators. It wraps detected
/// comments in a gray-colored span and string literals in a green-colored span, while other tokens
/// are processed to apply additional styling relevant to Dioxus patterns.
///
/// # Examples
///
/// ```
/// let code = r#"fn main() {
///     // This is a comment
///     println!("Hello, Dioxus!");
/// }"#;
///
/// let highlighted = highlight_rust_syntax(code);
/// assert!(highlighted.contains("<span class='text-gray-500'>"));
/// assert!(highlighted.contains("<span class='text-green-500'>"));
fn highlight_rust_syntax(code: &str) -> String {
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

/// Highlights a token for Rust syntax highlighting by wrapping it in HTML span elements with CSS classes
///
/// This function determines the appropriate syntax highlighting for a given token. When the token is part of a
/// string literal (indicated by `in_string` being true), the token is returned unmodified. Otherwise, the token
/// is sanitized and then compared against a series of rules to apply the correct CSS styling:
/// - Tokens equal to `#[component]` are styled in purple.
/// - Rust keywords (e.g., `fn`, `let`, `mut`, etc.) are styled in blue.
/// - Capitalized identifiers (that are not part of a `Route::` expression) are styled in orange for Dioxus components.
/// - The RSX macro `rsx!` is styled in yellow.
/// - Tokens starting with `Route::` are split so that the `"Route::"` prefix appears in green and the rest in orange.
/// - Tokens ending with a colon are styled in a lighter blue.
/// - Numeric tokens (including decimals) are styled in orange.
///
/// # Examples
///
/// ```
/// let highlighted_fn = highlight_token("fn", false);
/// assert!(highlighted_fn.contains("text-blue-500"));
///
/// let token_in_string = highlight_token("any_token", true);
/// // When the token is inside a string literal, no highlighting is applied
/// assert_eq!(token_in_string, "any_token");
fn highlight_token(token: &str, in_string: bool) -> String {
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

/// Highlights TOML syntax by wrapping tokens with HTML span elements for visual styling.
///
/// This function processes a TOML code snippet, identifying and styling comments,
/// string literals, and various separators. It wraps comments (starting with `#`)
/// in a span with a gray color, string literals in a span with a green color,
/// and brackets in a span with blue coloring.
///
/// # Examples
///
/// ```
/// let toml_code = "\
/// [section]\n\
/// key = \"value\" # This is a comment\n\
/// ";
///
/// let highlighted = highlight_toml_syntax(toml_code);
/// assert!(highlighted.contains("text-green-500"));
/// assert!(highlighted.contains("text-gray-500"));
fn highlight_toml_syntax(code: &str) -> String {
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

/// Highlights a TOML token by wrapping it in HTML span elements with appropriate styling based on its content.
///
/// This function applies syntax highlighting to a TOML token unless it is part of a string literal (as indicated by `in_string`).
/// When not in a string:
/// - Tokens representing section headers (starting with '[' and ending with ']') are styled with a blue class.
/// - Tokens containing an '=' are treated as key-value pairs, with the key highlighted in purple and the value processed recursively.
/// - Tokens ending with '=' (indicating keys) are highlighted in purple.
/// - Tokens consisting solely of digits, dots, or quotes (e.g., version numbers or literals) are styled with an orange class.
///   If the token is empty after trimming or if `in_string` is true, the original token is returned unmodified.
/// # Examples
///
/// ```
/// // Highlight a section header
/// let token = "[section]";
/// let highlighted = highlight_toml_token(token, false);
/// assert_eq!(highlighted, "<span class='text-blue-400'>[section]</span>");
///
/// // Highlight a version number or literal
/// let token_literal = "1.2.3";
/// let highlighted_literal = highlight_toml_token(token_literal, false);
/// assert_eq!(highlighted_literal, "<span class='text-orange-400'>1.2.3</span>");
///
/// // Token inside a string is returned unchanged
/// assert_eq!(highlight_toml_token("data", true), "data");
/// ```
fn highlight_toml_token(token: &str, in_string: bool) -> String {
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

/// Processes a TOML value and returns it wrapped in an HTML span with styling based on its content.
///
/// Boolean values ("true" or "false") and numerically formatted values are highlighted with an orange color,
/// while quoted strings are highlighted with a green color. If the value does not match any of these patterns,
/// it is returned unmodified.
///
/// # Examples
///
/// ```
/// let highlighted_bool = highlight_toml_value("true");
/// assert_eq!(highlighted_bool, "<span class='text-orange-400'>true</span>");
///
/// let highlighted_num = highlight_toml_value("123.45");
/// assert_eq!(highlighted_num, "<span class='text-orange-400'>123.45</span>");
///
/// let highlighted_string = highlight_toml_value("\"hello\"");
/// assert_eq!(highlighted_string, "<span class='text-green-500'>\"hello\"</span>");
///
/// let unchanged = highlight_toml_value("example");
/// assert_eq!(unchanged, "example");
fn highlight_toml_value(value: &str) -> String {
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
/// Renders a syntax-highlighted code block as a Dioxus component.
///
/// This component applies syntax highlighting to the provided code snippet based on the specified language.
/// For "rust" and "toml", it uses the appropriate highlighter; for any other language, the code is rendered without modification.
/// The output is wrapped in a `<pre>` element styled for overflow control, background appearance, and a monospaced font.
///
/// # Arguments
///
/// * `code` - The code snippet to highlight.
/// * `language` - The language identifier (e.g., "rust", "toml"). This value is case-insensitive.
///
/// # Returns
///
/// An `Element` that displays the highlighted code block.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// // Example: Render a Rust code snippet with syntax highlighting.
/// let code = String::from("fn main() { println!(\"Hello, world!\"); }");
/// let language = String::from("rust");
/// let code_block = CodeBlock(code, language);
///
/// // The `code_block` component can be included in a Dioxus application's view.
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
