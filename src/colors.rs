use pyo3::prelude::*;

/// RGBA color with u8 components.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color from u8 RGBA components.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create a color from f32 components in [0.0, 1.0].
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
            a: (a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    /// Convert to a tiny_skia Color.
    pub fn to_tiny_skia(&self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba8(self.r, self.g, self.b, self.a)
    }
}

/// Look up a named color (full name or single-char shorthand).
fn named_color(name: &str) -> Option<Color> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        "red" | "r" => Some(Color::new(255, 0, 0, 255)),
        "green" | "g" => Some(Color::new(0, 128, 0, 255)),
        "blue" | "b" => Some(Color::new(0, 0, 255, 255)),
        "cyan" | "c" => Some(Color::new(0, 255, 255, 255)),
        "magenta" | "m" => Some(Color::new(255, 0, 255, 255)),
        "yellow" | "y" => Some(Color::new(255, 255, 0, 255)),
        "black" | "k" => Some(Color::new(0, 0, 0, 255)),
        "white" | "w" => Some(Color::new(255, 255, 255, 255)),
        "orange" => Some(Color::new(255, 165, 0, 255)),
        "purple" => Some(Color::new(128, 0, 128, 255)),
        "brown" => Some(Color::new(165, 42, 42, 255)),
        "pink" => Some(Color::new(255, 192, 203, 255)),
        "gray" | "grey" => Some(Color::new(128, 128, 128, 255)),
        "olive" => Some(Color::new(128, 128, 0, 255)),
        "navy" => Some(Color::new(0, 0, 128, 255)),
        "teal" => Some(Color::new(0, 128, 128, 255)),
        "lime" => Some(Color::new(0, 255, 0, 255)),
        _ => None,
    }
}

/// Parse a hex color string (without the '#' prefix).
/// Supports 3-char (#RGB), 6-char (#RRGGBB), and 8-char (#RRGGBBAA) hex.
fn hex_color(hex: &str) -> Option<Color> {
    // Validate all characters are ASCII hex digits
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            Some(Color::new(r * 17, g * 17, b * 17, 255))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::new(r, g, b, 255))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Color::new(r, g, b, a))
        }
        _ => None,
    }
}

/// Parse a color from a plain Rust string (name, hex). Returns black on failure.
pub fn parse_color_str(s: &str) -> Color {
    let trimmed = s.trim();
    if let Some(c) = named_color(trimmed) {
        return c;
    }
    if trimmed.starts_with('#') {
        if let Some(c) = hex_color(&trimmed[1..]) {
            return c;
        }
    }
    Color::new(0, 0, 0, 255)
}

/// Parse a Python color value: string (name, shorthand, hex) or tuple (RGB/RGBA floats).
pub fn parse_color_value(obj: &Bound<PyAny>) -> PyResult<Color> {
    // Try as string first
    if let Ok(s) = obj.extract::<String>() {
        let trimmed = s.trim();

        // Hex color
        if let Some(hex_str) = trimmed.strip_prefix('#') {
            if let Some(c) = hex_color(hex_str) {
                return Ok(c);
            }
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid hex color: {}",
                trimmed
            )));
        }

        // Named / shorthand color
        if let Some(c) = named_color(trimmed) {
            return Ok(c);
        }

        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unknown color name: {}",
            trimmed
        )));
    }

    // Try as tuple of floats
    if let Ok(tuple) = obj.extract::<(f64, f64, f64, f64)>() {
        return Ok(Color::from_f32(
            tuple.0 as f32,
            tuple.1 as f32,
            tuple.2 as f32,
            tuple.3 as f32,
        ));
    }

    if let Ok(tuple) = obj.extract::<(f64, f64, f64)>() {
        return Ok(Color::from_f32(
            tuple.0 as f32,
            tuple.1 as f32,
            tuple.2 as f32,
            1.0,
        ));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Color must be a string (name, hex) or tuple of floats (RGB or RGBA)",
    ))
}

/// Parse a color from Python and return (r, g, b, a) as u8 tuple.
#[pyfunction]
pub fn parse_color(obj: &Bound<PyAny>) -> PyResult<(u8, u8, u8, u8)> {
    let c = parse_color_value(obj)?;
    Ok((c.r, c.g, c.b, c.a))
}
