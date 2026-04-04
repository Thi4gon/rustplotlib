//! Basic LaTeX math text rendering.
//!
//! Supports: subscript (_), superscript (^), Greek letters (\alpha, etc.),
//! basic symbols (\pm, \times, \cdot, \infty), and simple formatting.

use crate::colors::Color;
use crate::text;

/// Parse a math-mode string and render it to a pixmap.
///
/// Recognizes $...$ delimiters and renders math symbols.
pub fn render_mathtext(
    pixmap: &mut tiny_skia::Pixmap,
    raw_text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    // Check if text contains math ($...$)
    if !raw_text.contains('$') {
        text::draw_text(pixmap, raw_text, x, y, size, color,
            text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
        return;
    }

    // Split into segments: plain and math
    let mut segments: Vec<(String, bool)> = Vec::new(); // (text, is_math)
    let mut in_math = false;
    let mut current = String::new();

    for ch in raw_text.chars() {
        if ch == '$' {
            if !current.is_empty() {
                segments.push((current.clone(), in_math));
                current.clear();
            }
            in_math = !in_math;
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        segments.push((current, in_math));
    }

    // Render each segment
    let mut cursor_x = x;
    for (text, is_math) in &segments {
        if *is_math {
            let rendered = parse_math_symbols(text);
            let char_width = size * 0.6;
            text::draw_text(pixmap, &rendered, cursor_x, y, size, color,
                text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
            cursor_x += rendered.len() as f32 * char_width;
        } else {
            let char_width = size * 0.6;
            text::draw_text(pixmap, text, cursor_x, y, size, color,
                text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
            cursor_x += text.len() as f32 * char_width;
        }
    }
}

/// Render math text with proper layout (fractions stacked, sqrt with bar).
///
/// This is the "full" TeX layout engine that positions elements properly
/// instead of converting to flat Unicode strings.
pub fn render_mathtext_layout(
    pixmap: &mut tiny_skia::Pixmap,
    input: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) {
    let ts = tiny_skia::Transform::identity();
    let mut cursor_x = x;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let mut cmd = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() { cmd.push(c); chars.next(); }
                else { break; }
            }

            if cmd == "frac" {
                // \frac{num}{den} — render stacked with horizontal bar
                let num = extract_brace_content(&mut chars);
                let den = extract_brace_content(&mut chars);

                let frac_size = size * 0.75;
                let num_text = parse_math_symbols(&num);
                let den_text = parse_math_symbols(&den);

                let (num_w, num_h) = text::measure_text(&num_text, frac_size);
                let (den_w, den_h) = text::measure_text(&den_text, frac_size);
                let max_w = num_w.max(den_w);

                // Numerator centered above bar
                let num_x = cursor_x + (max_w - num_w) / 2.0;
                let bar_y = y + size * 0.4;
                text::draw_text(pixmap, &num_text, num_x, bar_y - num_h - 2.0,
                    frac_size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);

                // Horizontal bar
                let mut pb = tiny_skia::PathBuilder::new();
                pb.move_to(cursor_x, bar_y);
                pb.line_to(cursor_x + max_w, bar_y);
                if let Some(path) = pb.finish() {
                    let mut paint = tiny_skia::Paint::default();
                    paint.set_color(color.to_tiny_skia());
                    let mut stroke = tiny_skia::Stroke::default();
                    stroke.width = 0.8;
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }

                // Denominator centered below bar
                let den_x = cursor_x + (max_w - den_w) / 2.0;
                text::draw_text(pixmap, &den_text, den_x, bar_y + 3.0,
                    frac_size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);

                cursor_x += max_w + size * 0.2;
            } else if cmd == "sqrt" {
                // \sqrt{content} — render with radical sign and bar
                let content = extract_brace_content(&mut chars);
                let content_text = parse_math_symbols(&content);
                let (cw, ch) = text::measure_text(&content_text, size);

                // Draw radical symbol (√)
                let rad_w = size * 0.4;
                let bar_top = y;
                let bar_bottom = y + ch + 4.0;

                let mut pb = tiny_skia::PathBuilder::new();
                // V-shape of radical
                pb.move_to(cursor_x, y + ch * 0.6);
                pb.line_to(cursor_x + rad_w * 0.4, bar_bottom);
                pb.line_to(cursor_x + rad_w, bar_top);
                // Horizontal bar over content
                pb.line_to(cursor_x + rad_w + cw + 4.0, bar_top);
                if let Some(path) = pb.finish() {
                    let mut paint = tiny_skia::Paint::default();
                    paint.set_color(color.to_tiny_skia());
                    let mut stroke = tiny_skia::Stroke::default();
                    stroke.width = 1.0;
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }

                // Content text
                text::draw_text(pixmap, &content_text, cursor_x + rad_w + 2.0, y + 2.0,
                    size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);

                cursor_x += rad_w + cw + 6.0;
            } else if cmd == "int" || cmd == "sum" || cmd == "prod" {
                // Large operator with optional limits (from _ and ^)
                let symbol = match cmd.as_str() {
                    "int" => "∫",
                    "sum" => "Σ",
                    "prod" => "∏",
                    _ => "?",
                };

                // Render large symbol
                let large_size = size * 1.5;
                text::draw_text(pixmap, symbol, cursor_x, y - size * 0.15,
                    large_size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
                cursor_x += large_size * 0.7;

                // Check for limits: _lower ^upper
                if chars.peek() == Some(&'_') {
                    chars.next();
                    let lower = extract_next_group(&mut chars);
                    let lower_text = parse_math_symbols(&lower);
                    let small = size * 0.6;
                    text::draw_text(pixmap, &lower_text, cursor_x - large_size * 0.5,
                        y + large_size * 0.8, small, color,
                        text::TextAnchorX::Center, text::TextAnchorY::Top, 0.0);
                }
                if chars.peek() == Some(&'^') {
                    chars.next();
                    let upper = extract_next_group(&mut chars);
                    let upper_text = parse_math_symbols(&upper);
                    let small = size * 0.6;
                    text::draw_text(pixmap, &upper_text, cursor_x - large_size * 0.5,
                        y - small * 0.5, small, color,
                        text::TextAnchorX::Center, text::TextAnchorY::Top, 0.0);
                }
            } else {
                // Regular symbol — use existing lookup
                let sym = match cmd.as_str() {
                    "alpha" => "α", "beta" => "β", "gamma" => "γ", "delta" => "δ",
                    "pi" => "π", "sigma" => "σ", "theta" => "θ", "omega" => "ω",
                    "infty" => "∞", "pm" => "±", "times" => "×", "cdot" => "·",
                    "leq" => "≤", "geq" => "≥", "neq" => "≠", "approx" => "≈",
                    "rightarrow" | "to" => "→", "leftarrow" => "←",
                    "partial" => "∂", "nabla" => "∇",
                    _ => { cursor_x += size * 0.3; continue; }
                };
                text::draw_text(pixmap, sym, cursor_x, y, size, color,
                    text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
                cursor_x += size * 0.6;
            }
        } else if ch == '_' {
            // Subscript
            let sub = extract_next_group(&mut chars);
            let sub_text = parse_math_symbols(&sub);
            let sub_size = size * 0.7;
            text::draw_text(pixmap, &sub_text, cursor_x, y + size * 0.4,
                sub_size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
            cursor_x += sub_text.len() as f32 * sub_size * 0.6;
        } else if ch == '^' {
            // Superscript
            let sup = extract_next_group(&mut chars);
            let sup_text = parse_math_symbols(&sup);
            let sup_size = size * 0.7;
            text::draw_text(pixmap, &sup_text, cursor_x, y - size * 0.3,
                sup_size, color, text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
            cursor_x += sup_text.len() as f32 * sup_size * 0.6;
        } else if ch == '{' || ch == '}' {
            // skip
        } else if ch == ' ' {
            cursor_x += size * 0.3;
        } else {
            text::draw_text(pixmap, &ch.to_string(), cursor_x, y, size, color,
                text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
            cursor_x += size * 0.6;
        }
    }
}

/// Extract content within braces: {content}
fn extract_brace_content(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    if chars.peek() == Some(&'{') {
        chars.next(); // skip {
        let mut depth = 1;
        while let Some(c) = chars.next() {
            if c == '{' { depth += 1; }
            if c == '}' { depth -= 1; if depth == 0 { break; } }
            content.push(c);
        }
    } else if let Some(&c) = chars.peek() {
        chars.next();
        content.push(c);
    }
    content
}

/// Extract next group: either {content} or single char
fn extract_next_group(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    if chars.peek() == Some(&'{') {
        extract_brace_content(chars)
    } else if let Some(c) = chars.next() {
        c.to_string()
    } else {
        String::new()
    }
}

/// Convert LaTeX math commands to Unicode equivalents.
pub fn parse_math_symbols(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Collect command name
            let mut cmd = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() {
                    cmd.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            // Map to Unicode
            let symbol = match cmd.as_str() {
                // Greek lowercase
                "alpha" => "α",
                "beta" => "β",
                "gamma" => "γ",
                "delta" => "δ",
                "epsilon" => "ε",
                "zeta" => "ζ",
                "eta" => "η",
                "theta" => "θ",
                "iota" => "ι",
                "kappa" => "κ",
                "lambda" => "λ",
                "mu" => "μ",
                "nu" => "ν",
                "xi" => "ξ",
                "pi" => "π",
                "rho" => "ρ",
                "sigma" => "σ",
                "tau" => "τ",
                "upsilon" => "υ",
                "phi" => "φ",
                "chi" => "χ",
                "psi" => "ψ",
                "omega" => "ω",
                // Greek uppercase
                "Alpha" => "Α",
                "Beta" => "Β",
                "Gamma" => "Γ",
                "Delta" => "Δ",
                "Theta" => "Θ",
                "Lambda" => "Λ",
                "Xi" => "Ξ",
                "Pi" => "Π",
                "Sigma" => "Σ",
                "Phi" => "Φ",
                "Psi" => "Ψ",
                "Omega" => "Ω",
                // Math operators
                "pm" => "±",
                "mp" => "∓",
                "times" => "×",
                "div" => "÷",
                "cdot" => "·",
                "ast" => "∗",
                "star" => "⋆",
                "circ" => "∘",
                "bullet" => "•",
                // Relations
                "leq" | "le" => "≤",
                "geq" | "ge" => "≥",
                "neq" | "ne" => "≠",
                "approx" => "≈",
                "sim" => "∼",
                "equiv" => "≡",
                "propto" => "∝",
                // Arrows
                "rightarrow" | "to" => "→",
                "leftarrow" => "←",
                "leftrightarrow" => "↔",
                "Rightarrow" => "⇒",
                "Leftarrow" => "⇐",
                // Calculus & algebra
                "infty" => "∞",
                "partial" => "∂",
                "nabla" => "∇",
                "int" => "∫",
                "sum" => "Σ",
                "prod" => "∏",
                "sqrt" => "√",
                // Sets
                "in" => "∈",
                "notin" => "∉",
                "subset" => "⊂",
                "supset" => "⊃",
                "cup" => "∪",
                "cap" => "∩",
                "emptyset" => "∅",
                "forall" => "∀",
                "exists" => "∃",
                // Misc
                "degree" | "deg" => "°",
                "prime" => "′",
                "hbar" => "ℏ",
                "ell" => "ℓ",
                "Re" => "ℜ",
                "Im" => "ℑ",
                // Spacing
                "quad" => "  ",
                "qquad" => "    ",
                " " => " ",
                // Formatting (skip braces content for now)
                "frac" | "mathbf" | "mathit" | "mathrm" | "text" => {
                    // Consume {content} and output content directly
                    if chars.peek() == Some(&'{') {
                        chars.next(); // skip {
                        let mut content = String::new();
                        let mut depth = 1;
                        while let Some(c) = chars.next() {
                            if c == '{' { depth += 1; }
                            if c == '}' { depth -= 1; if depth == 0 { break; } }
                            content.push(c);
                        }
                        // For \frac, try to get second arg
                        if cmd == "frac" {
                            let mut denom = String::new();
                            if chars.peek() == Some(&'{') {
                                chars.next();
                                let mut d = 1;
                                while let Some(c) = chars.next() {
                                    if c == '{' { d += 1; }
                                    if c == '}' { d -= 1; if d == 0 { break; } }
                                    denom.push(c);
                                }
                            }
                            result.push_str(&content);
                            result.push('/');
                            result.push_str(&denom);
                            continue;
                        }
                        result.push_str(&content);
                        continue;
                    }
                    ""
                }
                _ => {
                    // Unknown command — output as-is
                    result.push('\\');
                    result.push_str(&cmd);
                    continue;
                }
            };
            result.push_str(symbol);
        } else if ch == '_' {
            // Subscript: render next char(s) as subscript using Unicode
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // skip {
                    while let Some(c) = chars.next() {
                        if c == '}' { break; }
                        result.push(to_subscript(c));
                    }
                } else {
                    chars.next();
                    result.push(to_subscript(next));
                }
            }
        } else if ch == '^' {
            // Superscript: render next char(s) as superscript using Unicode
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // skip {
                    while let Some(c) = chars.next() {
                        if c == '}' { break; }
                        result.push(to_superscript(c));
                    }
                } else {
                    chars.next();
                    result.push(to_superscript(next));
                }
            }
        } else if ch == '{' || ch == '}' {
            // Skip braces
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert a digit/letter to Unicode subscript.
fn to_subscript(c: char) -> char {
    match c {
        '0' => '₀', '1' => '₁', '2' => '₂', '3' => '₃', '4' => '₄',
        '5' => '₅', '6' => '₆', '7' => '₇', '8' => '₈', '9' => '₉',
        '+' => '₊', '-' => '₋', '=' => '₌', '(' => '₍', ')' => '₎',
        'a' => 'ₐ', 'e' => 'ₑ', 'o' => 'ₒ', 'x' => 'ₓ',
        'i' => 'ᵢ', 'j' => 'ⱼ', 'k' => 'ₖ', 'n' => 'ₙ',
        _ => c, // fallback: return as-is
    }
}

/// Convert a digit/letter to Unicode superscript.
fn to_superscript(c: char) -> char {
    match c {
        '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
        '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
        '+' => '⁺', '-' => '⁻', '=' => '⁼', '(' => '⁽', ')' => '⁾',
        'n' => 'ⁿ', 'i' => 'ⁱ',
        _ => c,
    }
}
