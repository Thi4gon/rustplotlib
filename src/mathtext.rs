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
