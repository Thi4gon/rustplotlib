//! Plot argument parsing — moved from Python to Rust for performance.
//! Handles matplotlib-style format strings like "r--o" and color shorthand.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

/// Parse a matplotlib format string like "r--o" into (color, linestyle, marker).
///
/// Returns a tuple of (Option<color>, Option<linestyle>, Option<marker>).
#[pyfunction]
pub fn parse_fmt(fmt: &str) -> (Option<String>, Option<String>, Option<String>) {
    let color_chars = ['r', 'g', 'b', 'c', 'm', 'y', 'k', 'w'];
    let marker_chars = ['.', 'o', 's', '^', 'v', '+', 'x', 'D', '*',
                        '<', '>', 'p', 'h', 'H', '8', '|', '_', 'P', 'X',
                        '1', '2', '3', '4', 'd'];

    let mut color: Option<String> = None;
    let mut linestyle: Option<String> = None;
    let mut marker: Option<String> = None;

    let mut remaining = fmt.to_string();

    // Extract color character (first char if it's a color)
    if let Some(first) = remaining.chars().next() {
        if color_chars.contains(&first) {
            color = Some(first.to_string());
            remaining = remaining[1..].to_string();
        }
    }

    // Extract linestyle (longest match first)
    let linestyles = ["--", "-.", ":", "-"];
    for ls in &linestyles {
        if remaining.contains(ls) {
            linestyle = Some(ls.to_string());
            remaining = remaining.replacen(ls, "", 1);
            break;
        }
    }

    // Extract marker character
    for ch in remaining.chars() {
        if marker_chars.contains(&ch) {
            marker = Some(ch.to_string());
            break;
        }
    }

    (color, linestyle, marker)
}

/// Map single-char color to full color name.
#[pyfunction]
pub fn color_char_to_name(c: &str) -> String {
    match c {
        "r" => "red".to_string(),
        "g" => "green".to_string(),
        "b" => "blue".to_string(),
        "c" => "cyan".to_string(),
        "m" => "magenta".to_string(),
        "y" => "yellow".to_string(),
        "k" => "black".to_string(),
        "w" => "white".to_string(),
        _ => c.to_string(),
    }
}

/// Ticker formatting — format a tick value using scalar formatting rules.
///
/// This replaces Python's ScalarFormatter.__call__().
#[pyfunction]
pub fn format_tick_scalar(value: f64, precision: Option<usize>) -> String {
    let prec = precision.unwrap_or(2);

    // Integer values display without decimal
    if value == value.floor() && value.abs() < 1e15 {
        let iv = value as i64;
        return iv.to_string();
    }

    // Small/large values use scientific notation
    if value != 0.0 && (value.abs() < 1e-4 || value.abs() >= 1e7) {
        return format!("{:.prec$e}", value, prec = prec);
    }

    // Normal values: format with precision, trim trailing zeros
    let s = format!("{:.prec$}", value, prec = prec);
    if s.contains('.') {
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    } else {
        s
    }
}

/// Format a tick value as a percentage.
#[pyfunction]
pub fn format_tick_percent(value: f64, xmax: f64, decimals: Option<usize>) -> String {
    let dec = decimals.unwrap_or(0);
    let pct = value / xmax * 100.0;
    if dec == 0 {
        format!("{:.0}%", pct)
    } else {
        format!("{:.prec$}%", pct, prec = dec)
    }
}

/// Format a tick value using engineering notation (1k, 1M, 1G, etc.).
#[pyfunction]
pub fn format_tick_engineering(value: f64, precision: Option<usize>) -> String {
    let prec = precision.unwrap_or(3);

    if value == 0.0 {
        return "0".to_string();
    }

    let prefixes = [
        (1e24, "Y"), (1e21, "Z"), (1e18, "E"), (1e15, "P"),
        (1e12, "T"), (1e9, "G"), (1e6, "M"), (1e3, "k"),
        (1.0, ""), (1e-3, "m"), (1e-6, "µ"), (1e-9, "n"),
        (1e-12, "p"), (1e-15, "f"), (1e-18, "a"),
    ];

    let abs_val = value.abs();
    for &(threshold, prefix) in &prefixes {
        if abs_val >= threshold {
            let scaled = value / threshold;
            let s = format!("{:.prec$}", scaled, prec = prec);
            let trimmed = if s.contains('.') {
                s.trim_end_matches('0').trim_end_matches('.').to_string()
            } else {
                s
            };
            return format!("{}{}", trimmed, prefix);
        }
    }

    format!("{:.prec$e}", value, prec = prec)
}

/// Format a tick value using log scale formatting.
#[pyfunction]
pub fn format_tick_log(value: f64, base: Option<f64>) -> String {
    let b = base.unwrap_or(10.0);

    if value <= 0.0 {
        return "0".to_string();
    }

    let exp = value.log(b);
    if (exp - exp.round()).abs() < 1e-10 {
        let exp_int = exp.round() as i64;
        if b == 10.0 {
            format!("$10^{{{}}}$", exp_int)
        } else {
            format!("{:.0}^{}", b, exp_int)
        }
    } else {
        format_tick_scalar(value, Some(2))
    }
}

/// Generate tick values for MultipleLocator.
#[pyfunction]
pub fn tick_values_multiple(vmin: f64, vmax: f64, base: f64) -> Vec<f64> {
    if base <= 0.0 || vmin >= vmax {
        return vec![];
    }
    let start = (vmin / base).ceil() as i64;
    let end = (vmax / base).floor() as i64;
    (start..=end).map(|i| i as f64 * base).collect()
}

/// Generate tick values for LogLocator.
#[pyfunction]
pub fn tick_values_log(vmin: f64, vmax: f64, base: Option<f64>, numdecs: Option<usize>) -> Vec<f64> {
    let b = base.unwrap_or(10.0);
    let nd = numdecs.unwrap_or(10);

    if vmin <= 0.0 || vmax <= 0.0 || vmin >= vmax {
        return vec![];
    }

    let log_min = vmin.log(b).floor() as i64;
    let log_max = vmax.log(b).ceil() as i64;

    let mut ticks = Vec::new();
    for exp in log_min..=log_max {
        let val = b.powi(exp as i32);
        if val >= vmin && val <= vmax {
            ticks.push(val);
        }
        if ticks.len() >= nd {
            break;
        }
    }
    ticks
}

/// Generate tick values for LinearLocator.
#[pyfunction]
pub fn tick_values_linear(vmin: f64, vmax: f64, numticks: Option<usize>) -> Vec<f64> {
    let n = numticks.unwrap_or(11);
    if n <= 1 {
        return vec![vmin];
    }
    let step = (vmax - vmin) / (n - 1) as f64;
    (0..n).map(|i| vmin + i as f64 * step).collect()
}

/// Hit test: check if a point (mx, my) is within tolerance of any point in (xs, ys).
/// Returns indices of points within tolerance.
#[pyfunction]
pub fn hit_test_points(xs: Vec<f64>, ys: Vec<f64>, mx: f64, my: f64, tolerance: f64) -> Vec<usize> {
    let tol_sq = tolerance * tolerance;
    let n = xs.len().min(ys.len());
    let mut indices = Vec::new();
    for i in 0..n {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        if dx * dx + dy * dy <= tol_sq {
            indices.push(i);
        }
    }
    indices
}

/// Hit test for line segments: check if point is within tolerance of any segment.
/// Returns indices of closest line point.
#[pyfunction]
pub fn hit_test_line(xs: Vec<f64>, ys: Vec<f64>, mx: f64, my: f64, tolerance: f64) -> Vec<usize> {
    let n = xs.len().min(ys.len());
    if n == 0 {
        return vec![];
    }

    let tol_sq = tolerance * tolerance;
    let mut indices = Vec::new();

    // Check distance to each point
    for i in 0..n {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        if dx * dx + dy * dy <= tol_sq {
            indices.push(i);
        }
    }

    // If no point hit, check distance to line segments
    if indices.is_empty() && n >= 2 {
        for i in 0..n - 1 {
            let (x1, y1) = (xs[i], ys[i]);
            let (x2, y2) = (xs[i + 1], ys[i + 1]);

            let dx = x2 - x1;
            let dy = y2 - y1;
            let len_sq = dx * dx + dy * dy;

            if len_sq < 1e-20 {
                continue;
            }

            // Project point onto line segment
            let t = ((mx - x1) * dx + (my - y1) * dy) / len_sq;
            let t = t.clamp(0.0, 1.0);

            let proj_x = x1 + t * dx;
            let proj_y = y1 + t * dy;

            let dist_sq = (mx - proj_x).powi(2) + (my - proj_y).powi(2);
            if dist_sq <= tol_sq {
                indices.push(i);
            }
        }
    }

    indices
}
