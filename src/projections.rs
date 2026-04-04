//! Geographic map projections for rustplotlib.
//!
//! Implements Hammer, Aitoff, and Mollweide equal-area projections.
//! All projections map (longitude, latitude) in radians to (x, y) in plot coordinates.

use pyo3::prelude::*;
use std::f64::consts::PI;

/// Project (longitude, latitude) using the Hammer projection.
/// Input: lon, lat in radians (-π..π, -π/2..π/2)
/// Output: (x, y) in [-2√2, 2√2] x [-√2, √2]
#[pyfunction]
pub fn hammer_project(lon: f64, lat: f64) -> (f64, f64) {
    let z = (1.0 + lat.cos() * (lon / 2.0).cos()).sqrt();
    let x = 2.0 * f64::sqrt(2.0) * lat.cos() * (lon / 2.0).sin() / z;
    let y = f64::sqrt(2.0) * lat.sin() / z;
    (x, y)
}

/// Project (longitude, latitude) using the Aitoff projection.
/// Input: lon, lat in radians
/// Output: (x, y)
#[pyfunction]
pub fn aitoff_project(lon: f64, lat: f64) -> (f64, f64) {
    let alpha = ((lat.cos() * (lon / 2.0).cos()).acos()).max(1e-15);
    let sinc_alpha = if alpha.abs() < 1e-15 { 1.0 } else { alpha.sin() / alpha };
    let x = 2.0 * lat.cos() * (lon / 2.0).sin() / sinc_alpha;
    let y = lat.sin() / sinc_alpha;
    (x, y)
}

/// Project (longitude, latitude) using the Mollweide projection.
/// Input: lon, lat in radians
/// Output: (x, y) in [-2√2, 2√2] x [-√2, √2]
#[pyfunction]
pub fn mollweide_project(lon: f64, lat: f64) -> (f64, f64) {
    // Solve 2θ + sin(2θ) = π sin(lat) iteratively (Newton's method)
    let target = PI * lat.sin();
    let mut theta = lat;
    for _ in 0..20 {
        let f = 2.0 * theta + (2.0 * theta).sin() - target;
        let df = 2.0 + 2.0 * (2.0 * theta).cos();
        if df.abs() < 1e-15 { break; }
        theta -= f / df;
        if f.abs() < 1e-12 { break; }
    }
    let x = 2.0 * f64::sqrt(2.0) / PI * lon * theta.cos();
    let y = f64::sqrt(2.0) * theta.sin();
    (x, y)
}

/// Batch project an array of (lon, lat) pairs using Hammer projection.
#[pyfunction]
pub fn hammer_project_batch(lons: Vec<f64>, lats: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let n = lons.len().min(lats.len());
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = hammer_project(lons[i], lats[i]);
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

/// Batch project an array of (lon, lat) pairs using Aitoff projection.
#[pyfunction]
pub fn aitoff_project_batch(lons: Vec<f64>, lats: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let n = lons.len().min(lats.len());
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = aitoff_project(lons[i], lats[i]);
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

/// Batch project using Mollweide projection.
#[pyfunction]
pub fn mollweide_project_batch(lons: Vec<f64>, lats: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let n = lons.len().min(lats.len());
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = mollweide_project(lons[i], lats[i]);
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

/// Generate graticule lines (meridians + parallels) for a given projection.
/// Returns list of (xs, ys) polyline segments.
#[pyfunction]
pub fn generate_graticule(projection: &str, n_meridians: usize, n_parallels: usize, n_points: usize)
    -> Vec<(Vec<f64>, Vec<f64>)>
{
    let mut lines = Vec::new();
    let proj_fn: fn(f64, f64) -> (f64, f64) = match projection {
        "hammer" => hammer_project,
        "aitoff" => aitoff_project,
        "mollweide" => mollweide_project,
        _ => hammer_project,
    };

    // Meridians (vertical lines at fixed longitudes)
    for i in 0..=n_meridians {
        let lon = -PI + 2.0 * PI * i as f64 / n_meridians as f64;
        let mut xs = Vec::with_capacity(n_points);
        let mut ys = Vec::with_capacity(n_points);
        for j in 0..=n_points {
            let lat = -PI / 2.0 + PI * j as f64 / n_points as f64;
            let (x, y) = proj_fn(lon, lat);
            xs.push(x);
            ys.push(y);
        }
        lines.push((xs, ys));
    }

    // Parallels (horizontal lines at fixed latitudes)
    for i in 0..=n_parallels {
        let lat = -PI / 2.0 + PI * i as f64 / n_parallels as f64;
        let mut xs = Vec::with_capacity(n_points);
        let mut ys = Vec::with_capacity(n_points);
        for j in 0..=n_points {
            let lon = -PI + 2.0 * PI * j as f64 / n_points as f64;
            let (x, y) = proj_fn(lon, lat);
            xs.push(x);
            ys.push(y);
        }
        lines.push((xs, ys));
    }

    lines
}
