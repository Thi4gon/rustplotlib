use pyo3::prelude::*;

/// Compute "nice" tick positions for an axis range [vmin, vmax].
/// Uses step sizes from the set {1, 2, 2.5, 5, 10} * 10^n.
pub fn compute_auto_ticks(vmin: f64, vmax: f64, max_ticks: usize) -> Vec<f64> {
    if (vmax - vmin).abs() < 1e-15 {
        return vec![vmin];
    }

    let (lo, hi) = if vmin < vmax {
        (vmin, vmax)
    } else {
        (vmax, vmin)
    };

    let range = hi - lo;

    // Find the best "nice" step size
    let rough_step = range / (max_ticks as f64 - 1.0).max(1.0);
    let step = nice_step(rough_step);

    // Compute tick start and end, snapped to multiples of step
    let tick_start = (lo / step).floor() * step;
    let tick_end = (hi / step).ceil() * step;

    let mut ticks = Vec::new();
    let mut t = tick_start;
    // Safety limit to prevent infinite loops due to floating point
    let max_iters = (max_ticks + 10) * 2;
    let mut iters = 0;

    while t <= tick_end + step * 0.001 && iters < max_iters {
        // Only include ticks within (or very close to) the data range
        if t >= lo - step * 0.001 && t <= hi + step * 0.001 {
            // Round to avoid floating point noise
            let rounded = round_to_step(t, step);
            ticks.push(rounded);
        }
        t += step;
        iters += 1;
    }

    // Ensure the endpoints are included if they're very close to a tick
    if !ticks.is_empty() {
        if (ticks[0] - lo).abs() > step * 0.5 {
            ticks.insert(0, lo);
        }
        if let Some(&last) = ticks.last() {
            if (last - hi).abs() > step * 0.5 {
                ticks.push(hi);
            }
        }
    }

    ticks
}

/// Find a "nice" step size close to the given rough step.
fn nice_step(rough: f64) -> f64 {
    let exponent = rough.log10().floor();
    let fraction = rough / 10.0_f64.powf(exponent);

    let nice_fraction = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 2.5 {
        2.5
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };

    nice_fraction * 10.0_f64.powf(exponent)
}

/// Round a value to remove floating point noise relative to the step size.
fn round_to_step(val: f64, step: f64) -> f64 {
    // Determine number of decimal places in step
    let decimals = if step >= 1.0 {
        0i32
    } else {
        (-step.log10().floor()) as i32 + 1
    };
    let factor = 10.0_f64.powi(decimals);
    (val * factor).round() / factor
}

/// Format a tick value as a clean string (no trailing zeros beyond what's needed).
pub fn format_tick_value(val: f64) -> String {
    // Handle negative zero
    let v = if val == 0.0 { 0.0 } else { val };

    // If the value is effectively an integer, format without decimal point
    if (v - v.round()).abs() < 1e-9 && v.abs() < 1e15 {
        return format!("{}", v.round() as i64);
    }

    // Otherwise, format with enough precision and trim trailing zeros
    let s = format!("{:.15}", v);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    s.to_string()
}

/// Python-exposed function: compute auto ticks for a range.
#[pyfunction]
pub fn auto_ticks(vmin: f64, vmax: f64) -> Vec<f64> {
    compute_auto_ticks(vmin, vmax, 10)
}

/// Python-exposed function: format a tick value.
#[pyfunction]
pub fn format_tick(val: f64) -> String {
    format_tick_value(val)
}
