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
        // CSS/X11 named colors (full matplotlib compatibility)
        "aliceblue" => Some(Color::new(240, 248, 255, 255)),
        "antiquewhite" => Some(Color::new(250, 235, 215, 255)),
        "aqua" => Some(Color::new(0, 255, 255, 255)),
        "aquamarine" => Some(Color::new(127, 255, 212, 255)),
        "azure" => Some(Color::new(240, 255, 255, 255)),
        "beige" => Some(Color::new(245, 245, 220, 255)),
        "bisque" => Some(Color::new(255, 228, 196, 255)),
        "blanchedalmond" => Some(Color::new(255, 235, 205, 255)),
        "blueviolet" => Some(Color::new(138, 43, 226, 255)),
        "burlywood" => Some(Color::new(222, 184, 135, 255)),
        "cadetblue" => Some(Color::new(95, 158, 160, 255)),
        "chartreuse" => Some(Color::new(127, 255, 0, 255)),
        "chocolate" => Some(Color::new(210, 105, 30, 255)),
        "coral" => Some(Color::new(255, 127, 80, 255)),
        "cornflowerblue" => Some(Color::new(100, 149, 237, 255)),
        "cornsilk" => Some(Color::new(255, 248, 220, 255)),
        "crimson" => Some(Color::new(220, 20, 60, 255)),
        "darkblue" => Some(Color::new(0, 0, 139, 255)),
        "darkcyan" => Some(Color::new(0, 139, 139, 255)),
        "darkgoldenrod" => Some(Color::new(184, 134, 11, 255)),
        "darkgray" | "darkgrey" => Some(Color::new(169, 169, 169, 255)),
        "darkgreen" => Some(Color::new(0, 100, 0, 255)),
        "darkkhaki" => Some(Color::new(189, 183, 107, 255)),
        "darkmagenta" => Some(Color::new(139, 0, 139, 255)),
        "darkolivegreen" => Some(Color::new(85, 107, 47, 255)),
        "darkorange" => Some(Color::new(255, 140, 0, 255)),
        "darkorchid" => Some(Color::new(153, 50, 204, 255)),
        "darkred" => Some(Color::new(139, 0, 0, 255)),
        "darksalmon" => Some(Color::new(233, 150, 122, 255)),
        "darkseagreen" => Some(Color::new(143, 188, 143, 255)),
        "darkslateblue" => Some(Color::new(72, 61, 139, 255)),
        "darkslategray" | "darkslategrey" => Some(Color::new(47, 79, 79, 255)),
        "darkturquoise" => Some(Color::new(0, 206, 209, 255)),
        "darkviolet" => Some(Color::new(148, 0, 211, 255)),
        "deeppink" => Some(Color::new(255, 20, 147, 255)),
        "deepskyblue" => Some(Color::new(0, 191, 255, 255)),
        "dimgray" | "dimgrey" => Some(Color::new(105, 105, 105, 255)),
        "dodgerblue" => Some(Color::new(30, 144, 255, 255)),
        "firebrick" => Some(Color::new(178, 34, 34, 255)),
        "floralwhite" => Some(Color::new(255, 250, 240, 255)),
        "forestgreen" => Some(Color::new(34, 139, 34, 255)),
        "fuchsia" => Some(Color::new(255, 0, 255, 255)),
        "gainsboro" => Some(Color::new(220, 220, 220, 255)),
        "ghostwhite" => Some(Color::new(248, 248, 255, 255)),
        "gold" => Some(Color::new(255, 215, 0, 255)),
        "goldenrod" => Some(Color::new(218, 165, 32, 255)),
        "greenyellow" => Some(Color::new(173, 255, 47, 255)),
        "honeydew" => Some(Color::new(240, 255, 240, 255)),
        "hotpink" => Some(Color::new(255, 105, 180, 255)),
        "indianred" => Some(Color::new(205, 92, 92, 255)),
        "indigo" => Some(Color::new(75, 0, 130, 255)),
        "ivory" => Some(Color::new(255, 255, 240, 255)),
        "khaki" => Some(Color::new(240, 230, 140, 255)),
        "lavender" => Some(Color::new(230, 230, 250, 255)),
        "lavenderblush" => Some(Color::new(255, 240, 245, 255)),
        "lawngreen" => Some(Color::new(124, 252, 0, 255)),
        "lemonchiffon" => Some(Color::new(255, 250, 205, 255)),
        "lightblue" => Some(Color::new(173, 216, 230, 255)),
        "lightcoral" => Some(Color::new(240, 128, 128, 255)),
        "lightcyan" => Some(Color::new(224, 255, 255, 255)),
        "lightgoldenrodyellow" => Some(Color::new(250, 250, 210, 255)),
        "lightgray" | "lightgrey" => Some(Color::new(211, 211, 211, 255)),
        "lightgreen" => Some(Color::new(144, 238, 144, 255)),
        "lightpink" => Some(Color::new(255, 182, 193, 255)),
        "lightsalmon" => Some(Color::new(255, 160, 122, 255)),
        "lightseagreen" => Some(Color::new(32, 178, 170, 255)),
        "lightskyblue" => Some(Color::new(135, 206, 250, 255)),
        "lightslategray" | "lightslategrey" => Some(Color::new(119, 136, 153, 255)),
        "lightsteelblue" => Some(Color::new(176, 196, 222, 255)),
        "lightyellow" => Some(Color::new(255, 255, 224, 255)),
        "limegreen" => Some(Color::new(50, 205, 50, 255)),
        "linen" => Some(Color::new(250, 240, 230, 255)),
        "maroon" => Some(Color::new(128, 0, 0, 255)),
        "mediumaquamarine" => Some(Color::new(102, 205, 170, 255)),
        "mediumblue" => Some(Color::new(0, 0, 205, 255)),
        "mediumorchid" => Some(Color::new(186, 85, 211, 255)),
        "mediumpurple" => Some(Color::new(147, 112, 219, 255)),
        "mediumseagreen" => Some(Color::new(60, 179, 113, 255)),
        "mediumslateblue" => Some(Color::new(123, 104, 238, 255)),
        "mediumspringgreen" => Some(Color::new(0, 250, 154, 255)),
        "mediumturquoise" => Some(Color::new(72, 209, 204, 255)),
        "mediumvioletred" => Some(Color::new(199, 21, 133, 255)),
        "midnightblue" => Some(Color::new(25, 25, 112, 255)),
        "mintcream" => Some(Color::new(245, 255, 250, 255)),
        "mistyrose" => Some(Color::new(255, 228, 225, 255)),
        "moccasin" => Some(Color::new(255, 228, 181, 255)),
        "navajowhite" => Some(Color::new(255, 222, 173, 255)),
        "oldlace" => Some(Color::new(253, 245, 230, 255)),
        "olivedrab" => Some(Color::new(107, 142, 35, 255)),
        "orangered" => Some(Color::new(255, 69, 0, 255)),
        "orchid" => Some(Color::new(218, 112, 214, 255)),
        "palegoldenrod" => Some(Color::new(238, 232, 170, 255)),
        "palegreen" => Some(Color::new(152, 251, 152, 255)),
        "paleturquoise" => Some(Color::new(175, 238, 238, 255)),
        "palevioletred" => Some(Color::new(219, 112, 147, 255)),
        "papayawhip" => Some(Color::new(255, 239, 213, 255)),
        "peachpuff" => Some(Color::new(255, 218, 185, 255)),
        "peru" => Some(Color::new(205, 133, 63, 255)),
        "plum" => Some(Color::new(221, 160, 221, 255)),
        "powderblue" => Some(Color::new(176, 224, 230, 255)),
        "rosybrown" => Some(Color::new(188, 143, 143, 255)),
        "royalblue" => Some(Color::new(65, 105, 225, 255)),
        "saddlebrown" => Some(Color::new(139, 69, 19, 255)),
        "salmon" => Some(Color::new(250, 128, 114, 255)),
        "sandybrown" => Some(Color::new(244, 164, 96, 255)),
        "seagreen" => Some(Color::new(46, 139, 87, 255)),
        "seashell" => Some(Color::new(255, 245, 238, 255)),
        "sienna" => Some(Color::new(160, 82, 45, 255)),
        "silver" => Some(Color::new(192, 192, 192, 255)),
        "skyblue" => Some(Color::new(135, 206, 235, 255)),
        "slateblue" => Some(Color::new(106, 90, 205, 255)),
        "slategray" | "slategrey" => Some(Color::new(112, 128, 144, 255)),
        "snow" => Some(Color::new(255, 250, 250, 255)),
        "springgreen" => Some(Color::new(0, 255, 127, 255)),
        "steelblue" => Some(Color::new(70, 130, 180, 255)),
        "tan" => Some(Color::new(210, 180, 140, 255)),
        "thistle" => Some(Color::new(216, 191, 216, 255)),
        "tomato" => Some(Color::new(255, 99, 71, 255)),
        "turquoise" => Some(Color::new(64, 224, 208, 255)),
        "violet" => Some(Color::new(238, 130, 238, 255)),
        "wheat" => Some(Color::new(245, 222, 179, 255)),
        "whitesmoke" => Some(Color::new(245, 245, 245, 255)),
        "yellowgreen" => Some(Color::new(154, 205, 50, 255)),
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
