//! Geographic map projections for rustplotlib.
//!
//! Implements Hammer, Aitoff, Mollweide, Lambert Conformal Conic,
//! and Stereographic projections.
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

/// Project (longitude, latitude) using the Lambert Conformal Conic projection.
///
/// Parameters:
/// - `lon`, `lat`: point coordinates in radians
/// - `lat1`, `lat2`: standard parallels in radians (where the cone intersects the globe)
///
/// The central meridian is lon=0 and the latitude of origin is the average of lat1 and lat2.
/// Output: (x, y) in projected coordinates.
#[pyfunction]
pub fn lambert_project(lon: f64, lat: f64, lat1: f64, lat2: f64) -> (f64, f64) {
    // Compute the cone constant n
    let n = if (lat1 - lat2).abs() < 1e-10 {
        lat1.sin()
    } else {
        (lat1.cos().ln() - lat2.cos().ln())
            / ((PI / 4.0 + lat2 / 2.0).tan().ln() - (PI / 4.0 + lat1 / 2.0).tan().ln())
    };

    // F constant
    let f = lat1.cos() * (PI / 4.0 + lat1 / 2.0).tan().powf(n) / n;

    // Latitude of origin = midpoint of the two standard parallels
    let lat0 = (lat1 + lat2) / 2.0;

    // rho for the given latitude
    let rho = f / (PI / 4.0 + lat / 2.0).tan().powf(n);
    // rho0 for the latitude of origin
    let rho0 = f / (PI / 4.0 + lat0 / 2.0).tan().powf(n);

    let theta = n * lon;

    let x = rho * theta.sin();
    let y = rho0 - rho * theta.cos();
    (x, y)
}

/// Batch project an array of (lon, lat) pairs using Lambert Conformal Conic projection.
#[pyfunction]
pub fn lambert_project_batch(lons: Vec<f64>, lats: Vec<f64>, lat1: f64, lat2: f64) -> (Vec<f64>, Vec<f64>) {
    let n = lons.len().min(lats.len());
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = lambert_project(lons[i], lats[i], lat1, lat2);
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

/// Project (longitude, latitude) using the Stereographic projection.
///
/// Parameters:
/// - `lon`, `lat`: point coordinates in radians
/// - `lon0`, `lat0`: center of projection in radians
///
/// This is the general oblique stereographic (azimuthal) projection.
/// Output: (x, y) in projected coordinates. The scale factor R=1.
#[pyfunction]
pub fn stereographic_project(lon: f64, lat: f64, lon0: f64, lat0: f64) -> (f64, f64) {
    let dlon = lon - lon0;
    let k_denom = 1.0 + lat0.sin() * lat.sin() + lat0.cos() * lat.cos() * dlon.cos();

    // If the point is antipodal to the center, clamp to avoid division by zero
    let k = if k_denom.abs() < 1e-15 {
        1e15 // effectively infinity — point maps to the edge
    } else {
        2.0 / k_denom
    };

    let x = k * lat.cos() * dlon.sin();
    let y = k * (lat0.cos() * lat.sin() - lat0.sin() * lat.cos() * dlon.cos());
    (x, y)
}

/// Batch project an array of (lon, lat) pairs using Stereographic projection.
#[pyfunction]
pub fn stereographic_project_batch(lons: Vec<f64>, lats: Vec<f64>, lon0: f64, lat0: f64) -> (Vec<f64>, Vec<f64>) {
    let n = lons.len().min(lats.len());
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = stereographic_project(lons[i], lats[i], lon0, lat0);
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

    // For Lambert and Stereographic we need to wrap the projection into a 2-arg closure
    // that captures the extra parameters with sensible defaults.
    // Lambert defaults: standard parallels at 30° and 60° N
    // Stereographic defaults: center at (0, 0)
    let proj: Box<dyn Fn(f64, f64) -> (f64, f64)> = match projection {
        "hammer" => Box::new(|lon, lat| hammer_project(lon, lat)),
        "aitoff" => Box::new(|lon, lat| aitoff_project(lon, lat)),
        "mollweide" => Box::new(|lon, lat| mollweide_project(lon, lat)),
        "lambert" => {
            let lat1 = 30.0_f64.to_radians();
            let lat2 = 60.0_f64.to_radians();
            Box::new(move |lon, lat| lambert_project(lon, lat, lat1, lat2))
        }
        "stereographic" => {
            Box::new(|lon, lat| stereographic_project(lon, lat, 0.0, 0.0))
        }
        _ => Box::new(|lon, lat| hammer_project(lon, lat)),
    };

    // Meridians (vertical lines at fixed longitudes)
    for i in 0..=n_meridians {
        let lon = -PI + 2.0 * PI * i as f64 / n_meridians as f64;
        let mut xs = Vec::with_capacity(n_points);
        let mut ys = Vec::with_capacity(n_points);
        for j in 0..=n_points {
            let lat = -PI / 2.0 + PI * j as f64 / n_points as f64;
            let (x, y) = proj(lon, lat);
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
            let (x, y) = proj(lon, lat);
            xs.push(x);
            ys.push(y);
        }
        lines.push((xs, ys));
    }

    lines
}
