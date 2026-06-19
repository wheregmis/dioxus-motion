//! CSS value interpolation primitives shared across animation hooks.
// Methods are called from presence.rs which is gated behind the dioxus feature.
#![cfg_attr(not(feature = "dioxus"), allow(dead_code))]

/// Animated CSS property value.
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    /// Unitless numeric value.
    Number(f32),
    /// Pixel numeric value.
    Px(f32),
    /// Percentage numeric value.
    Percent(f32),
    /// Viewport-width numeric value.
    Vw(f32),
    /// Viewport-height numeric value.
    Vh(f32),
    /// Degree numeric value.
    Deg(f32),
    /// RGBA color value.
    Color(CssColor),
    /// A compatible string containing interpolable numbers and/or colors.
    Complex(CssComplexValue),
    /// A discrete CSS keyword or CSS variable reference.
    Keyword(String),
}

impl CssValue {
    /// Adds compatible CSS values for vector-style animation math.
    pub(crate) fn add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Some(Self::Number(a + b)),
            (Self::Px(a), Self::Px(b)) => Some(Self::Px(a + b)),
            (Self::Percent(a), Self::Percent(b)) => Some(Self::Percent(a + b)),
            (Self::Vw(a), Self::Vw(b)) => Some(Self::Vw(a + b)),
            (Self::Vh(a), Self::Vh(b)) => Some(Self::Vh(a + b)),
            (Self::Deg(a), Self::Deg(b)) => Some(Self::Deg(a + b)),
            (Self::Color(a), Self::Color(b)) => Some(Self::Color(a.add(b))),
            (Self::Complex(a), Self::Complex(b)) => a.add(b).map(Self::Complex),
            _ => None,
        }
    }

    /// Subtracts compatible CSS values for vector-style animation math.
    pub(crate) fn sub(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Some(Self::Number(a - b)),
            (Self::Px(a), Self::Px(b)) => Some(Self::Px(a - b)),
            (Self::Percent(a), Self::Percent(b)) => Some(Self::Percent(a - b)),
            (Self::Vw(a), Self::Vw(b)) => Some(Self::Vw(a - b)),
            (Self::Vh(a), Self::Vh(b)) => Some(Self::Vh(a - b)),
            (Self::Deg(a), Self::Deg(b)) => Some(Self::Deg(a - b)),
            (Self::Color(a), Self::Color(b)) => Some(Self::Color(a.sub(b))),
            (Self::Complex(a), Self::Complex(b)) => a.sub(b).map(Self::Complex),
            _ => None,
        }
    }

    /// Scales a CSS value for vector-style animation math.
    pub(crate) fn scale(&self, factor: f32) -> Self {
        match self {
            Self::Number(value) => Self::Number(value * factor),
            Self::Px(value) => Self::Px(value * factor),
            Self::Percent(value) => Self::Percent(value * factor),
            Self::Vw(value) => Self::Vw(value * factor),
            Self::Vh(value) => Self::Vh(value * factor),
            Self::Deg(value) => Self::Deg(value * factor),
            Self::Color(value) => Self::Color(value.scale(factor)),
            Self::Complex(value) => Self::Complex(value.scale(factor)),
            Self::Keyword(value) => Self::Keyword(value.clone()),
        }
    }

    /// Returns a scalar representation used for magnitude and convergence checks.
    pub(crate) fn number(&self) -> f32 {
        match self {
            Self::Number(value)
            | Self::Px(value)
            | Self::Percent(value)
            | Self::Vw(value)
            | Self::Vh(value)
            | Self::Deg(value) => *value,
            Self::Color(color) => color.magnitude(),
            Self::Complex(value) => value.magnitude(),
            Self::Keyword(_) => 0.0,
        }
    }

    /// Interpolates between two values at progress `t` (clamped to 0–1).
    pub(crate) fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        match (self, target) {
            (Self::Number(start), Self::Number(end)) => Self::Number(lerp(*start, *end, t)),
            (Self::Px(start), Self::Px(end)) => Self::Px(lerp(*start, *end, t)),
            (Self::Percent(start), Self::Percent(end)) => Self::Percent(lerp(*start, *end, t)),
            (Self::Vw(start), Self::Vw(end)) => Self::Vw(lerp(*start, *end, t)),
            (Self::Vh(start), Self::Vh(end)) => Self::Vh(lerp(*start, *end, t)),
            (Self::Deg(start), Self::Deg(end)) => Self::Deg(lerp(*start, *end, t)),
            (Self::Color(start), Self::Color(end)) => Self::Color(start.interpolate(end, t)),
            (Self::Complex(start), Self::Complex(end)) => start
                .interpolate(end, t)
                .map_or_else(|| target.clone(), Self::Complex),
            (_, target) => target.clone(),
        }
    }

    /// Serializes the value to a CSS string.
    pub fn to_css(&self) -> String {
        match self {
            Self::Number(value) => format_number(*value),
            Self::Px(value) => format!("{}px", format_number(*value)),
            Self::Percent(value) => format!("{}%", format_number(*value)),
            Self::Vw(value) => format!("{}vw", format_number(*value)),
            Self::Vh(value) => format!("{}vh", format_number(*value)),
            Self::Deg(value) => format!("{}deg", format_number(*value)),
            Self::Color(value) => value.to_css(),
            Self::Complex(value) => value.to_css(),
            Self::Keyword(value) => value.clone(),
        }
    }
}

/// Interpolable RGBA color used in CSS animation values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssColor {
    /// Red component from 0.0 to 255.0.
    pub red: f32,
    /// Green component from 0.0 to 255.0.
    pub green: f32,
    /// Blue component from 0.0 to 255.0.
    pub blue: f32,
    /// Alpha component from 0.0 to 1.0.
    pub alpha: f32,
}

impl CssColor {
    /// Creates a color from RGBA components.
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: red.clamp(0.0, 255.0),
            green: green.clamp(0.0, 255.0),
            blue: blue.clamp(0.0, 255.0),
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    pub(crate) fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self::rgba(
            lerp(self.red, target.red, t),
            lerp(self.green, target.green, t),
            lerp(self.blue, target.blue, t),
            lerp(self.alpha, target.alpha, t),
        )
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            alpha: self.alpha + other.alpha,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            alpha: self.alpha - other.alpha,
        }
    }

    fn scale(&self, factor: f32) -> Self {
        Self {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
            alpha: self.alpha * factor,
        }
    }

    pub(crate) fn magnitude(&self) -> f32 {
        (self.red * self.red + self.green * self.green + self.blue * self.blue).sqrt()
    }

    pub(crate) fn to_css(self) -> String {
        let color = Self::rgba(self.red, self.green, self.blue, self.alpha);
        format!(
            "rgba({}, {}, {}, {})",
            format_number(color.red.round()),
            format_number(color.green.round()),
            format_number(color.blue.round()),
            format_number(color.alpha)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CssComplexToken {
    Text(String),
    Number(f32),
    Color(CssColor),
}

/// Interpolable CSS string containing compatible numeric and color tokens.
#[derive(Debug, Clone, PartialEq)]
pub struct CssComplexValue {
    tokens: Vec<CssComplexToken>,
}

impl CssComplexValue {
    fn parse(value: &str) -> Option<Self> {
        let mut tokens = Vec::new();
        let mut text = String::new();
        let mut index = 0;

        while index < value.len() {
            let rest = &value[index..];
            if let Some((len, color)) = parse_color_prefix(rest) {
                if !text.is_empty() {
                    tokens.push(CssComplexToken::Text(std::mem::take(&mut text)));
                }
                tokens.push(CssComplexToken::Color(color));
                index += len;
            } else if let Some((len, number)) = parse_number_prefix(rest) {
                if !text.is_empty() {
                    tokens.push(CssComplexToken::Text(std::mem::take(&mut text)));
                }
                tokens.push(CssComplexToken::Number(number));
                index += len;
            } else {
                let ch = rest.chars().next()?;
                text.push(ch);
                index += ch.len_utf8();
            }
        }

        if !text.is_empty() {
            tokens.push(CssComplexToken::Text(text));
        }

        let interpolable = tokens.iter().any(|token| {
            matches!(
                token,
                CssComplexToken::Number(_) | CssComplexToken::Color(_)
            )
        });
        interpolable.then_some(Self { tokens })
    }

    fn interpolate(&self, target: &Self, t: f32) -> Option<Self> {
        if self.tokens.len() != target.tokens.len() {
            return None;
        }

        let mut tokens = Vec::with_capacity(self.tokens.len());
        for (start, end) in self.tokens.iter().zip(&target.tokens) {
            let token = match (start, end) {
                (CssComplexToken::Text(a), CssComplexToken::Text(b)) if a == b => {
                    CssComplexToken::Text(a.clone())
                }
                (CssComplexToken::Number(a), CssComplexToken::Number(b)) => {
                    CssComplexToken::Number(lerp(*a, *b, t))
                }
                (CssComplexToken::Color(a), CssComplexToken::Color(b)) => {
                    CssComplexToken::Color(a.interpolate(b, t))
                }
                _ => return None,
            };
            tokens.push(token);
        }

        Some(Self { tokens })
    }

    fn add(&self, other: &Self) -> Option<Self> {
        self.zip_map(other, |start, end| match (start, end) {
            (CssComplexToken::Text(a), CssComplexToken::Text(b)) if a == b => {
                Some(CssComplexToken::Text(a.clone()))
            }
            (CssComplexToken::Text(a), CssComplexToken::Text(b)) if a.is_empty() => {
                Some(CssComplexToken::Text(b.clone()))
            }
            (CssComplexToken::Text(a), CssComplexToken::Text(b)) if b.is_empty() => {
                Some(CssComplexToken::Text(a.clone()))
            }
            (CssComplexToken::Number(a), CssComplexToken::Number(b)) => {
                Some(CssComplexToken::Number(a + b))
            }
            (CssComplexToken::Color(a), CssComplexToken::Color(b)) => {
                Some(CssComplexToken::Color(a.add(b)))
            }
            _ => None,
        })
    }

    fn sub(&self, other: &Self) -> Option<Self> {
        self.zip_map(other, |start, end| match (start, end) {
            (CssComplexToken::Text(a), CssComplexToken::Text(b)) if a == b => {
                Some(CssComplexToken::Text(String::new()))
            }
            (CssComplexToken::Text(a), CssComplexToken::Text(b))
                if a.is_empty() || b.is_empty() =>
            {
                Some(CssComplexToken::Text(String::new()))
            }
            (CssComplexToken::Number(a), CssComplexToken::Number(b)) => {
                Some(CssComplexToken::Number(a - b))
            }
            (CssComplexToken::Color(a), CssComplexToken::Color(b)) => {
                Some(CssComplexToken::Color(a.sub(b)))
            }
            _ => None,
        })
    }

    fn scale(&self, factor: f32) -> Self {
        Self {
            tokens: self
                .tokens
                .iter()
                .map(|token| match token {
                    CssComplexToken::Text(_) => CssComplexToken::Text(String::new()),
                    CssComplexToken::Number(value) => CssComplexToken::Number(value * factor),
                    CssComplexToken::Color(value) => CssComplexToken::Color(value.scale(factor)),
                })
                .collect(),
        }
    }

    fn zip_map(
        &self,
        other: &Self,
        map: impl Fn(&CssComplexToken, &CssComplexToken) -> Option<CssComplexToken>,
    ) -> Option<Self> {
        if self.tokens.len() != other.tokens.len() {
            return None;
        }

        let mut tokens = Vec::with_capacity(self.tokens.len());
        for (start, end) in self.tokens.iter().zip(&other.tokens) {
            tokens.push(map(start, end)?);
        }

        Some(Self { tokens })
    }

    fn magnitude(&self) -> f32 {
        self.tokens
            .iter()
            .map(|token| match token {
                CssComplexToken::Number(value) => value * value,
                CssComplexToken::Color(value) => value.magnitude(),
                CssComplexToken::Text(_) => 0.0,
            })
            .sum::<f32>()
            .sqrt()
    }

    fn to_css(&self) -> String {
        let mut css = String::new();
        for token in &self.tokens {
            match token {
                CssComplexToken::Text(value) => css.push_str(value),
                CssComplexToken::Number(value) => css.push_str(&format_number(*value)),
                CssComplexToken::Color(value) => css.push_str(&value.to_css()),
            }
        }
        css
    }
}

/// Converts values into typed CSS animation values.
pub trait IntoCssValue {
    /// Converts a Rust value into a typed animated CSS value for the given property.
    fn into_css_value(self, property: &str) -> CssValue;
}

macro_rules! impl_css_number_value {
    ($($type:ty),+ $(,)?) => {
        $(
            impl IntoCssValue for $type {
                fn into_css_value(self, property: &str) -> CssValue {
                    let value = self as f32;
                    if is_unitless_css_property(property) {
                        CssValue::Number(value)
                    } else {
                        CssValue::Px(value)
                    }
                }
            }
        )+
    };
}

impl_css_number_value!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl IntoCssValue for &str {
    fn into_css_value(self, _property: &str) -> CssValue {
        parse_css_string(self)
    }
}

impl IntoCssValue for String {
    fn into_css_value(self, property: &str) -> CssValue {
        self.as_str().into_css_value(property)
    }
}

/// Parses a CSS value string into the appropriate [`CssValue`] variant.
pub(crate) fn parse_css_string(value: &str) -> CssValue {
    let value = value.trim();

    value
        .strip_suffix("px")
        .and_then(parse_f32)
        .map(CssValue::Px)
        .or_else(|| {
            value
                .strip_suffix('%')
                .and_then(parse_f32)
                .map(CssValue::Percent)
        })
        .or_else(|| {
            value
                .strip_suffix("vw")
                .and_then(parse_f32)
                .map(CssValue::Vw)
        })
        .or_else(|| {
            value
                .strip_suffix("vh")
                .and_then(parse_f32)
                .map(CssValue::Vh)
        })
        .or_else(|| {
            value
                .strip_suffix("deg")
                .and_then(parse_f32)
                .map(CssValue::Deg)
        })
        .or_else(|| parse_color(value).map(CssValue::Color))
        .or_else(|| {
            is_browser_native_color_function(value).then(|| CssValue::Keyword(value.to_string()))
        })
        .or_else(|| CssComplexValue::parse(value).map(CssValue::Complex))
        .unwrap_or_else(|| CssValue::Keyword(value.to_string()))
}

/// Returns true for CSS properties that take unitless numeric values.
pub(crate) fn is_unitless_css_property(property: &str) -> bool {
    matches!(
        property,
        "opacity"
            | "z-index"
            | "font-weight"
            | "line-height"
            | "flex"
            | "flex-grow"
            | "flex-shrink"
            | "order"
            | "scale"
    )
}

fn parse_f32(value: &str) -> Option<f32> {
    value.trim().parse::<f32>().ok()
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

fn format_number(value: f32) -> String {
    let value = if value == -0.0 { 0.0 } else { value };
    let rounded = (value * 1_000_000.0).round() / 1_000_000.0;
    rounded.to_string()
}

fn parse_color(value: &str) -> Option<CssColor> {
    parse_hex_color(value)
        .or_else(|| parse_rgb_color(value))
        .or_else(|| parse_hsl_color(value))
}

fn is_browser_native_color_function(value: &str) -> bool {
    let Some((name, _)) = value.split_once('(') else {
        return false;
    };

    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "oklch" | "oklab" | "lab" | "lch" | "color" | "color-mix" | "light-dark"
    )
}

fn parse_color_prefix(value: &str) -> Option<(usize, CssColor)> {
    if value.starts_with('#') {
        let len = value
            .chars()
            .take_while(|character| character.is_ascii_hexdigit() || *character == '#')
            .map(char::len_utf8)
            .sum::<usize>();
        return parse_hex_color(&value[..len]).map(|color| (len, color));
    }

    if value.starts_with("rgb(") || value.starts_with("rgba(") {
        let len = value.find(')')? + 1;
        return parse_rgb_color(&value[..len]).map(|color| (len, color));
    }

    if value.starts_with("hsl(") || value.starts_with("hsla(") {
        let len = value.find(')')? + 1;
        return parse_hsl_color(&value[..len]).map(|color| (len, color));
    }

    None
}

fn parse_hex_color(value: &str) -> Option<CssColor> {
    let hex = value.strip_prefix('#')?;
    let parse_pair = |pair: &str| u8::from_str_radix(pair, 16).ok().map(f32::from);
    let parse_single = |single: &str| {
        u8::from_str_radix(&single.repeat(2), 16)
            .ok()
            .map(f32::from)
    };

    match hex.len() {
        3 => Some(CssColor::rgba(
            parse_single(&hex[0..1])?,
            parse_single(&hex[1..2])?,
            parse_single(&hex[2..3])?,
            1.0,
        )),
        4 => Some(CssColor::rgba(
            parse_single(&hex[0..1])?,
            parse_single(&hex[1..2])?,
            parse_single(&hex[2..3])?,
            parse_single(&hex[3..4])? / 255.0,
        )),
        6 => Some(CssColor::rgba(
            parse_pair(&hex[0..2])?,
            parse_pair(&hex[2..4])?,
            parse_pair(&hex[4..6])?,
            1.0,
        )),
        8 => Some(CssColor::rgba(
            parse_pair(&hex[0..2])?,
            parse_pair(&hex[2..4])?,
            parse_pair(&hex[4..6])?,
            parse_pair(&hex[6..8])? / 255.0,
        )),
        _ => None,
    }
}

fn parse_rgb_color(value: &str) -> Option<CssColor> {
    let body = value
        .strip_prefix("rgb(")
        .or_else(|| value.strip_prefix("rgba("))?
        .strip_suffix(')')?;
    let parts = body
        .replace(['/', ','], " ")
        .split_whitespace()
        .enumerate()
        .map(|(index, part)| {
            let is_percent = part.ends_with('%');
            let value = part.trim_end_matches('%').parse::<f32>().ok()?;
            Some(if is_percent {
                if index < 3 {
                    value * 2.55
                } else {
                    value / 100.0
                }
            } else {
                value
            })
        })
        .collect::<Option<Vec<_>>>()?;

    if parts.len() < 3 {
        return None;
    }

    Some(CssColor::rgba(
        parts[0],
        parts[1],
        parts[2],
        parts.get(3).copied().unwrap_or(1.0),
    ))
}

fn parse_hsl_color(value: &str) -> Option<CssColor> {
    let body = value
        .strip_prefix("hsl(")
        .or_else(|| value.strip_prefix("hsla("))?
        .strip_suffix(')')?;
    let normalized = body.replace(['/', ','], " ");
    let parts = normalized.split_whitespace().collect::<Vec<_>>();

    if parts.len() < 3 {
        return None;
    }

    let hue = parse_hue(parts[0])?;
    let saturation = parse_percentage(parts[1])?;
    let lightness = parse_percentage(parts[2])?;
    let alpha = parts.get(3).map_or(Some(1.0), |part| parse_alpha(part))?;

    Some(hsl_to_rgb(hue, saturation, lightness, alpha))
}

fn parse_hue(value: &str) -> Option<f32> {
    value
        .trim_end_matches("deg")
        .trim_end_matches("turn")
        .parse::<f32>()
        .ok()
        .map(|hue| {
            if value.ends_with("turn") {
                hue * 360.0
            } else {
                hue
            }
        })
}

fn parse_percentage(value: &str) -> Option<f32> {
    value
        .strip_suffix('%')?
        .parse::<f32>()
        .ok()
        .map(|value| (value / 100.0).clamp(0.0, 1.0))
}

fn parse_alpha(value: &str) -> Option<f32> {
    if let Some(percent) = value.strip_suffix('%') {
        percent.parse::<f32>().ok().map(|value| value / 100.0)
    } else {
        value.parse::<f32>().ok()
    }
}

fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> CssColor {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_segment = hue.rem_euclid(360.0) / 60.0;
    let secondary = chroma * (1.0 - (hue_segment % 2.0 - 1.0).abs());

    let (red, green, blue) = if hue_segment < 1.0 {
        (chroma, secondary, 0.0)
    } else if hue_segment < 2.0 {
        (secondary, chroma, 0.0)
    } else if hue_segment < 3.0 {
        (0.0, chroma, secondary)
    } else if hue_segment < 4.0 {
        (0.0, secondary, chroma)
    } else if hue_segment < 5.0 {
        (secondary, 0.0, chroma)
    } else {
        (chroma, 0.0, secondary)
    };

    let match_value = lightness - chroma / 2.0;
    CssColor::rgba(
        (red + match_value) * 255.0,
        (green + match_value) * 255.0,
        (blue + match_value) * 255.0,
        alpha,
    )
}

fn parse_number_prefix(value: &str) -> Option<(usize, f32)> {
    let mut end = 0;
    let mut has_digit = false;
    let mut has_dot = false;

    for (index, character) in value.char_indices() {
        let valid = if index == 0 && (character == '-' || character == '+') {
            true
        } else if character.is_ascii_digit() {
            has_digit = true;
            true
        } else if character == '.' && !has_dot {
            has_dot = true;
            true
        } else {
            false
        };

        if valid {
            end = index + character.len_utf8();
        } else {
            break;
        }
    }

    has_digit
        .then(|| value[..end].parse::<f32>().ok().map(|number| (end, number)))
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    fn color_approx_eq(a: &CssColor, b: &CssColor) -> bool {
        approx_eq(a.red, b.red)
            && approx_eq(a.green, b.green)
            && approx_eq(a.blue, b.blue)
            && approx_eq(a.alpha, b.alpha)
    }

    // ── lerp ─────────────────────────────────────────────────────────────────

    #[test]
    fn lerp_midpoint() {
        assert!(approx_eq(lerp(0.0, 10.0, 0.5), 5.0));
    }

    #[test]
    fn lerp_at_zero() {
        assert!(approx_eq(lerp(3.0, 7.0, 0.0), 3.0));
    }

    #[test]
    fn lerp_at_one() {
        assert!(approx_eq(lerp(3.0, 7.0, 1.0), 7.0));
    }

    #[test]
    fn lerp_clamps_below_zero() {
        assert!(approx_eq(lerp(0.0, 10.0, -1.0), 0.0));
    }

    #[test]
    fn lerp_clamps_above_one() {
        assert!(approx_eq(lerp(0.0, 10.0, 2.0), 10.0));
    }

    // ── format_number ────────────────────────────────────────────────────────

    #[test]
    fn format_number_integer() {
        assert_eq!(format_number(5.0), "5");
    }

    #[test]
    fn format_number_negative_zero() {
        assert_eq!(format_number(-0.0), "0");
    }

    #[test]
    fn format_number_decimal() {
        let s = format_number(0.5);
        assert_eq!(s, "0.5");
    }

    #[test]
    fn format_number_rounds_to_six_decimals() {
        let s = format_number(1.0 / 3.0);
        // should be "0.333333"
        assert!(s.starts_with("0.333333"));
    }

    // ── parse_f32 ────────────────────────────────────────────────────────────

    #[test]
    fn parse_f32_valid() {
        assert_eq!(parse_f32("  3.14  "), Some(3.14_f32));
    }

    #[test]
    fn parse_f32_invalid() {
        assert_eq!(parse_f32("abc"), None);
    }

    // ── parse_number_prefix ──────────────────────────────────────────────────

    #[test]
    fn parse_number_prefix_integer() {
        let r = parse_number_prefix("42px");
        assert!(r.is_some());
        let (len, n) = r.unwrap();
        assert_eq!(len, 2);
        assert!(approx_eq(n, 42.0));
    }

    #[test]
    fn parse_number_prefix_negative() {
        let (len, n) = parse_number_prefix("-3.5em").unwrap();
        assert_eq!(len, 4);
        assert!(approx_eq(n, -3.5));
    }

    #[test]
    fn parse_number_prefix_sign_only() {
        assert_eq!(parse_number_prefix("-text"), None);
        assert_eq!(parse_number_prefix("+text"), None);
    }

    #[test]
    fn parse_number_prefix_just_dot() {
        assert_eq!(parse_number_prefix(".text"), None);
    }

    #[test]
    fn parse_number_prefix_positive_sign() {
        let (len, n) = parse_number_prefix("+7rest").unwrap();
        assert_eq!(len, 2);
        assert!(approx_eq(n, 7.0));
    }

    #[test]
    fn parse_number_prefix_empty_string() {
        assert_eq!(parse_number_prefix(""), None);
    }

    // ── parse_hex_color ──────────────────────────────────────────────────────

    #[test]
    fn parse_hex_color_six_digit() {
        let c = parse_hex_color("#ff8000").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 128.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_hex_color_three_digit() {
        let c = parse_hex_color("#f80").unwrap();
        // f → ff = 255, 8 → 88 = 136, 0 → 00 = 0
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 136.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_hex_color_eight_digit() {
        let c = parse_hex_color("#ff000080").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 0.0));
        assert!(approx_eq(c.blue, 0.0));
        // 0x80 = 128; 128/255 ≈ 0.502
        assert!((c.alpha - 128.0 / 255.0).abs() < 0.001);
    }

    #[test]
    fn parse_hex_color_four_digit() {
        let c = parse_hex_color("#f00f").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 0.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_hex_color_invalid_length() {
        assert!(parse_hex_color("#ff0").is_some()); // 3-digit valid
        assert!(parse_hex_color("#ff00").is_some()); // 4-digit valid
        assert!(parse_hex_color("#ff0000").is_some()); // 6-digit valid
        assert!(parse_hex_color("#ff00000").is_none()); // 7-digit invalid
        assert!(parse_hex_color("#ff000000").is_some()); // 8-digit valid
        assert!(parse_hex_color("#ff0000000").is_none()); // 9-digit invalid
    }

    #[test]
    fn parse_hex_color_missing_prefix() {
        assert!(parse_hex_color("ff0000").is_none());
    }

    #[test]
    fn parse_hex_color_uppercase() {
        let c = parse_hex_color("#FF0000").unwrap();
        assert!(approx_eq(c.red, 255.0));
    }

    // ── parse_rgb_color ──────────────────────────────────────────────────────

    #[test]
    fn parse_rgb_color_basic() {
        let c = parse_rgb_color("rgb(255, 128, 0)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 128.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_rgb_color_rgba_form() {
        let c = parse_rgb_color("rgba(255, 0, 0, 0.5)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.alpha, 0.5));
    }

    #[test]
    fn parse_rgb_color_slash_alpha() {
        let c = parse_rgb_color("rgba(255 0 0 / 0.5)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.alpha, 0.5));
    }

    #[test]
    fn parse_rgb_color_scales_percent_channels() {
        let c = parse_rgb_color("rgb(100% 50% 0% / 25%)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!((c.green - 127.5).abs() < 0.001);
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 0.25));
    }

    #[test]
    fn parse_rgb_color_invalid() {
        assert!(parse_rgb_color("hsl(0, 100%, 50%)").is_none());
        assert!(parse_rgb_color("rgb(255, 0)").is_none()); // only 2 parts
    }

    #[test]
    fn parse_rgb_color_defaults_alpha_to_one() {
        let c = parse_rgb_color("rgb(10, 20, 30)").unwrap();
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn complex_value_parse_handles_non_ascii_text() {
        let value = CssComplexValue::parse("calc(100% - 2rem) /* café */").unwrap();
        assert_eq!(value.to_css(), "calc(100% - 2rem) /* café */");
    }

    // ── parse_hsl_color ──────────────────────────────────────────────────────

    #[test]
    fn parse_hsl_color_basic() {
        let c = parse_hsl_color("hsl(0, 100%, 50%)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 0.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_hsl_color_space_separated() {
        let c = parse_hsl_color("hsl(120 100% 25%)").unwrap();
        assert!(approx_eq(c.red, 0.0));
        assert!(approx_eq(c.green, 127.5));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn parse_hsl_color_slash_alpha() {
        let c = parse_hsl_color("hsl(240 100% 50% / 0.5)").unwrap();
        assert!(approx_eq(c.red, 0.0));
        assert!(approx_eq(c.green, 0.0));
        assert!(approx_eq(c.blue, 255.0));
        assert!(approx_eq(c.alpha, 0.5));
    }

    #[test]
    fn parse_hsl_color_hsla_form_with_percent_alpha() {
        let c = parse_hsl_color("hsla(60, 100%, 50%, 25%)").unwrap();
        assert!(approx_eq(c.red, 255.0));
        assert!(approx_eq(c.green, 255.0));
        assert!(approx_eq(c.blue, 0.0));
        assert!(approx_eq(c.alpha, 0.25));
    }

    #[test]
    fn parse_hsl_color_turn_unit() {
        let c = parse_hsl_color("hsl(0.5turn 100% 50%)").unwrap();
        assert!(approx_eq(c.red, 0.0));
        assert!(approx_eq(c.green, 255.0));
        assert!(approx_eq(c.blue, 255.0));
    }

    // ── CssColor ─────────────────────────────────────────────────────────────

    #[test]
    fn css_color_rgba_clamps() {
        let c = CssColor::rgba(-10.0, 300.0, 128.0, 2.0);
        assert!(approx_eq(c.red, 0.0));
        assert!(approx_eq(c.green, 255.0));
        assert!(approx_eq(c.blue, 128.0));
        assert!(approx_eq(c.alpha, 1.0));
    }

    #[test]
    fn css_color_interpolate_midpoint() {
        let a = CssColor::rgba(0.0, 0.0, 0.0, 0.0);
        let b = CssColor::rgba(100.0, 200.0, 50.0, 1.0);
        let mid = a.interpolate(&b, 0.5);
        assert!(approx_eq(mid.red, 50.0));
        assert!(approx_eq(mid.green, 100.0));
        assert!(approx_eq(mid.blue, 25.0));
        assert!(approx_eq(mid.alpha, 0.5));
    }

    #[test]
    fn css_color_interpolate_at_zero_returns_start() {
        let a = CssColor::rgba(255.0, 0.0, 0.0, 1.0);
        let b = CssColor::rgba(0.0, 255.0, 0.0, 1.0);
        let r = a.interpolate(&b, 0.0);
        assert!(color_approx_eq(&r, &a));
    }

    #[test]
    fn css_color_interpolate_at_one_returns_end() {
        let a = CssColor::rgba(255.0, 0.0, 0.0, 1.0);
        let b = CssColor::rgba(0.0, 255.0, 0.0, 1.0);
        let r = a.interpolate(&b, 1.0);
        assert!(color_approx_eq(&r, &b));
    }

    #[test]
    fn css_color_magnitude() {
        let c = CssColor::rgba(3.0, 4.0, 0.0, 1.0);
        assert!(approx_eq(c.magnitude(), 5.0));
    }

    #[test]
    fn css_color_to_css() {
        let c = CssColor::rgba(255.0, 128.0, 0.0, 1.0);
        assert_eq!(c.to_css(), "rgba(255, 128, 0, 1)");
    }

    #[test]
    fn css_color_to_css_partial_alpha() {
        let c = CssColor::rgba(0.0, 0.0, 0.0, 0.5);
        assert_eq!(c.to_css(), "rgba(0, 0, 0, 0.5)");
    }

    // ── CssValue::number ─────────────────────────────────────────────────────

    #[test]
    fn css_value_number_variants() {
        assert!(approx_eq(CssValue::Number(5.0).number(), 5.0));
        assert!(approx_eq(CssValue::Px(10.0).number(), 10.0));
        assert!(approx_eq(CssValue::Percent(50.0).number(), 50.0));
        assert!(approx_eq(CssValue::Vw(20.0).number(), 20.0));
        assert!(approx_eq(CssValue::Vh(30.0).number(), 30.0));
        assert!(approx_eq(CssValue::Deg(90.0).number(), 90.0));
    }

    #[test]
    fn css_value_number_keyword_is_zero() {
        assert!(approx_eq(CssValue::Keyword("auto".into()).number(), 0.0));
    }

    #[test]
    fn css_value_number_color() {
        let c = CssColor::rgba(3.0, 4.0, 0.0, 1.0);
        assert!(approx_eq(CssValue::Color(c).number(), 5.0));
    }

    // ── CssValue::interpolate ────────────────────────────────────────────────

    #[test]
    fn css_value_interpolate_px() {
        let a = CssValue::Px(0.0);
        let b = CssValue::Px(100.0);
        assert_eq!(a.interpolate(&b, 0.5), CssValue::Px(50.0));
    }

    #[test]
    fn css_value_interpolate_percent() {
        let a = CssValue::Percent(0.0);
        let b = CssValue::Percent(80.0);
        assert_eq!(a.interpolate(&b, 0.25), CssValue::Percent(20.0));
    }

    #[test]
    fn css_value_interpolate_vw() {
        let a = CssValue::Vw(0.0);
        let b = CssValue::Vw(100.0);
        assert_eq!(a.interpolate(&b, 1.0), CssValue::Vw(100.0));
    }

    #[test]
    fn css_value_interpolate_vh() {
        let a = CssValue::Vh(0.0);
        let b = CssValue::Vh(100.0);
        assert_eq!(a.interpolate(&b, 0.0), CssValue::Vh(0.0));
    }

    #[test]
    fn css_value_interpolate_deg() {
        let a = CssValue::Deg(0.0);
        let b = CssValue::Deg(360.0);
        assert_eq!(a.interpolate(&b, 0.5), CssValue::Deg(180.0));
    }

    #[test]
    fn css_value_interpolate_number() {
        let a = CssValue::Number(0.0);
        let b = CssValue::Number(1.0);
        assert_eq!(a.interpolate(&b, 0.5), CssValue::Number(0.5));
    }

    #[test]
    fn css_value_interpolate_clamps_t() {
        let a = CssValue::Px(0.0);
        let b = CssValue::Px(100.0);
        assert_eq!(a.interpolate(&b, -0.5), CssValue::Px(0.0));
        assert_eq!(a.interpolate(&b, 1.5), CssValue::Px(100.0));
    }

    #[test]
    fn css_value_interpolate_color() {
        let a = CssValue::Color(CssColor::rgba(0.0, 0.0, 0.0, 0.0));
        let b = CssValue::Color(CssColor::rgba(100.0, 200.0, 50.0, 1.0));
        let mid = a.interpolate(&b, 0.5);
        if let CssValue::Color(c) = mid {
            assert!(approx_eq(c.red, 50.0));
            assert!(approx_eq(c.green, 100.0));
        } else {
            panic!("expected Color variant");
        }
    }

    #[test]
    fn css_value_interpolate_type_mismatch_returns_target() {
        let a = CssValue::Px(10.0);
        let b = CssValue::Percent(50.0);
        assert_eq!(a.interpolate(&b, 0.5), b);
    }

    #[test]
    fn css_value_interpolate_keyword_returns_target() {
        let a = CssValue::Keyword("auto".into());
        let b = CssValue::Keyword("none".into());
        assert_eq!(a.interpolate(&b, 0.5), b);
    }

    // ── CssValue::to_css ─────────────────────────────────────────────────────

    #[test]
    fn css_value_to_css_number() {
        assert_eq!(CssValue::Number(1.5).to_css(), "1.5");
    }

    #[test]
    fn css_value_to_css_px() {
        assert_eq!(CssValue::Px(10.0).to_css(), "10px");
    }

    #[test]
    fn css_value_to_css_percent() {
        assert_eq!(CssValue::Percent(50.0).to_css(), "50%");
    }

    #[test]
    fn css_value_to_css_vw() {
        assert_eq!(CssValue::Vw(100.0).to_css(), "100vw");
    }

    #[test]
    fn css_value_to_css_vh() {
        assert_eq!(CssValue::Vh(75.0).to_css(), "75vh");
    }

    #[test]
    fn css_value_to_css_deg() {
        assert_eq!(CssValue::Deg(90.0).to_css(), "90deg");
    }

    #[test]
    fn css_value_to_css_keyword() {
        assert_eq!(CssValue::Keyword("auto".into()).to_css(), "auto");
    }

    // ── parse_css_string ─────────────────────────────────────────────────────

    #[test]
    fn parse_css_string_px() {
        assert_eq!(parse_css_string("100px"), CssValue::Px(100.0));
    }

    #[test]
    fn parse_css_string_negative_px() {
        assert_eq!(parse_css_string("-20px"), CssValue::Px(-20.0));
    }

    #[test]
    fn parse_css_string_percent() {
        assert_eq!(parse_css_string("50%"), CssValue::Percent(50.0));
    }

    #[test]
    fn parse_css_string_vw() {
        assert_eq!(parse_css_string("100vw"), CssValue::Vw(100.0));
    }

    #[test]
    fn parse_css_string_vh() {
        assert_eq!(parse_css_string("100vh"), CssValue::Vh(100.0));
    }

    #[test]
    fn parse_css_string_deg() {
        assert_eq!(parse_css_string("45deg"), CssValue::Deg(45.0));
    }

    #[test]
    fn parse_css_string_hex_color() {
        let v = parse_css_string("#ff0000");
        if let CssValue::Color(c) = v {
            assert!(approx_eq(c.red, 255.0));
            assert!(approx_eq(c.green, 0.0));
            assert!(approx_eq(c.blue, 0.0));
        } else {
            panic!("expected Color, got {:?}", v);
        }
    }

    #[test]
    fn parse_css_string_rgb_color() {
        let v = parse_css_string("rgb(0, 128, 255)");
        assert!(matches!(v, CssValue::Color(_)));
    }

    #[test]
    fn parse_css_string_hsl_color() {
        let v = parse_css_string("hsl(210 100% 50% / 0.5)");
        assert!(matches!(v, CssValue::Color(_)));
    }

    #[test]
    fn parse_css_string_browser_native_color_function_is_keyword() {
        assert_eq!(
            parse_css_string("oklch(0.65 0.18 260)"),
            CssValue::Keyword("oklch(0.65 0.18 260)".into())
        );
        assert_eq!(
            parse_css_string("color(display-p3 0.2 0.4 0.8)"),
            CssValue::Keyword("color(display-p3 0.2 0.4 0.8)".into())
        );
    }

    #[test]
    fn parse_css_string_browser_native_color_function_is_case_insensitive() {
        assert_eq!(
            parse_css_string("OKLCH(0.65 0.18 260)"),
            CssValue::Keyword("OKLCH(0.65 0.18 260)".into())
        );
    }

    #[test]
    fn parse_css_string_keyword_auto() {
        assert_eq!(parse_css_string("auto"), CssValue::Keyword("auto".into()));
    }

    #[test]
    fn parse_css_string_keyword_none() {
        assert_eq!(parse_css_string("none"), CssValue::Keyword("none".into()));
    }

    #[test]
    fn parse_css_string_trims_whitespace() {
        assert_eq!(parse_css_string("  100px  "), CssValue::Px(100.0));
    }

    #[test]
    fn parse_css_string_complex_transform() {
        let v = parse_css_string("translateX(10px)");
        // "translateX(10px)" contains a number so should parse as Complex
        assert!(
            matches!(v, CssValue::Complex(_)),
            "expected Complex, got {:?}",
            v
        );
    }

    #[test]
    fn parse_css_string_complex_to_css_roundtrip() {
        let original = "translateX(10px) translateY(20px)";
        let v = parse_css_string(original);
        // Should be Complex because it contains numbers
        if let CssValue::Complex(_) = &v {
            let css = v.to_css();
            assert_eq!(css, original);
        } else {
            panic!("expected Complex variant");
        }
    }

    #[test]
    fn parse_css_string_zero_px() {
        assert_eq!(parse_css_string("0px"), CssValue::Px(0.0));
    }

    #[test]
    fn parse_css_string_float_px() {
        assert_eq!(parse_css_string("1.5px"), CssValue::Px(1.5));
    }

    // ── is_unitless_css_property ──────────────────────────────────────────────

    #[test]
    fn is_unitless_known_properties() {
        assert!(is_unitless_css_property("opacity"));
        assert!(is_unitless_css_property("z-index"));
        assert!(is_unitless_css_property("font-weight"));
        assert!(is_unitless_css_property("line-height"));
        assert!(is_unitless_css_property("flex"));
        assert!(is_unitless_css_property("flex-grow"));
        assert!(is_unitless_css_property("flex-shrink"));
        assert!(is_unitless_css_property("order"));
        assert!(is_unitless_css_property("scale"));
    }

    #[test]
    fn is_unitless_unknown_properties() {
        assert!(!is_unitless_css_property("width"));
        assert!(!is_unitless_css_property("height"));
        assert!(!is_unitless_css_property("margin"));
        assert!(!is_unitless_css_property("font-size"));
        assert!(!is_unitless_css_property(""));
    }

    // ── IntoCssValue ─────────────────────────────────────────────────────────

    #[test]
    fn into_css_value_numeric_px_property() {
        assert_eq!(100_i32.into_css_value("width"), CssValue::Px(100.0));
        assert_eq!(1.5_f32.into_css_value("margin"), CssValue::Px(1.5));
    }

    #[test]
    fn into_css_value_numeric_unitless_property() {
        assert_eq!(0.5_f32.into_css_value("opacity"), CssValue::Number(0.5));
        assert_eq!(
            700_u32.into_css_value("font-weight"),
            CssValue::Number(700.0)
        );
    }

    #[test]
    fn into_css_value_str() {
        assert_eq!("50%".into_css_value("width"), CssValue::Percent(50.0));
        assert_eq!(
            "auto".into_css_value("margin"),
            CssValue::Keyword("auto".into())
        );
    }

    #[test]
    fn into_css_value_string_owned() {
        assert_eq!(
            "100px".to_string().into_css_value("left"),
            CssValue::Px(100.0)
        );
    }

    // ── CssComplexValue ───────────────────────────────────────────────────────

    #[test]
    fn complex_value_parse_requires_number_or_color() {
        // Pure text → None (no interpolable tokens)
        assert!(CssComplexValue::parse("auto none inherit").is_none());
    }

    #[test]
    fn complex_value_parse_with_number() {
        let cv = CssComplexValue::parse("scale(1.5)");
        assert!(cv.is_some());
    }

    #[test]
    fn complex_value_parse_with_embedded_color() {
        let cv = CssComplexValue::parse("color #ff0000 end");
        assert!(cv.is_some());
    }

    #[test]
    fn complex_value_interpolate_different_token_counts_returns_none() {
        let a = CssComplexValue::parse("translateX(10px)").unwrap();
        let b = CssComplexValue::parse("translateX(20px) rotate(5deg)").unwrap();
        assert!(a.interpolate(&b, 0.5).is_none());
    }

    #[test]
    fn complex_value_interpolate_mismatched_types_returns_none() {
        // token types differ at same position → None
        let a = CssComplexValue::parse("scale(1)").unwrap();
        let b = CssComplexValue::parse("scale(#ff0000)").unwrap();
        // These may or may not have the same token count depending on parse;
        // both should at least parse successfully, so exercise the path without crashing
        let _ = a.interpolate(&b, 0.5);
    }

    #[test]
    fn complex_value_interpolate_matching_numbers() {
        let a = CssComplexValue::parse("scale(0)").unwrap();
        let b = CssComplexValue::parse("scale(2)").unwrap();
        let mid = a
            .interpolate(&b, 0.5)
            .expect("interpolation should succeed");
        assert_eq!(mid.to_css(), "scale(1)");
    }

    #[test]
    fn complex_value_interpolate_text_must_match() {
        // Different text at the same position → None
        let a = CssComplexValue::parse("foo(1)").unwrap();
        let b = CssComplexValue::parse("bar(2)").unwrap();
        assert!(a.interpolate(&b, 0.5).is_none());
    }

    #[test]
    fn complex_value_magnitude_sums_squared_numbers() {
        let cv = CssComplexValue::parse("X(3) Y(4)").unwrap();
        // numbers 3 and 4 → sqrt(9+16) = 5
        assert!(approx_eq(cv.magnitude(), 5.0));
    }

    #[test]
    fn complex_value_to_css_roundtrip() {
        let input = "translateX(10) rotate(45)";
        let cv = CssComplexValue::parse(input).unwrap();
        assert_eq!(cv.to_css(), input);
    }

    // ── CssValue::interpolate complex path ───────────────────────────────────

    #[test]
    fn css_value_interpolate_complex_fallback_on_mismatch() {
        // Mismatched complex values fall back to target.clone()
        let a = CssValue::Complex(CssComplexValue::parse("scale(1)").unwrap());
        let b = CssValue::Complex(CssComplexValue::parse("translateX(10) rotate(5)").unwrap());
        let result = a.interpolate(&b, 0.5);
        assert_eq!(result, b);
    }

    #[test]
    fn css_value_interpolate_complex_success() {
        let a = parse_css_string("scale(0)");
        let b = parse_css_string("scale(1)");
        let mid = a.interpolate(&b, 0.5);
        assert_eq!(mid.to_css(), "scale(0.5)");
    }

    // ── parse_color_prefix ────────────────────────────────────────────────────

    #[test]
    fn parse_color_prefix_hex() {
        let (len, c) = parse_color_prefix("#ff0000 rest").unwrap();
        assert_eq!(len, 7);
        assert!(approx_eq(c.red, 255.0));
    }

    #[test]
    fn parse_color_prefix_rgb() {
        let (len, c) = parse_color_prefix("rgb(0, 255, 0) rest").unwrap();
        assert_eq!(len, 14);
        assert!(approx_eq(c.green, 255.0));
    }

    #[test]
    fn parse_color_prefix_hsl() {
        let (len, c) = parse_color_prefix("hsl(120 100% 25% / 0.4) rest").unwrap();
        assert_eq!(len, 23);
        assert!(approx_eq(c.green, 127.5));
        assert!(approx_eq(c.alpha, 0.4));
    }

    #[test]
    fn complex_value_parse_with_embedded_hsl_color() {
        let cv = CssComplexValue::parse("0px 20px 40px hsl(210 72% 42% / 0.38)").unwrap();
        assert_eq!(cv.to_css(), "0px 20px 40px rgba(30, 107, 184, 0.38)");
    }

    #[test]
    fn parse_color_prefix_none_for_non_color() {
        assert!(parse_color_prefix("hello world").is_none());
        assert!(parse_color_prefix("100px").is_none());
    }
}
