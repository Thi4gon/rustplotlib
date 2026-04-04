use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D contour plot — draws contour lines on a fixed Z-plane in 3D space.
pub struct Contour3D {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<Vec<f64>>,
    pub z: Vec<Vec<f64>>,
    pub levels: Vec<f64>,
    pub z_offset: f64,     // Z level at which contour lines are drawn
    pub filled: bool,      // true = contourf3d, false = contour3d
    pub linewidth: f32,
    pub cmap: String,
    pub alpha: f32,
}

impl Contour3D {
    pub fn new(
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        levels: Option<Vec<f64>>,
        z_offset: Option<f64>,
        filled: bool,
        linewidth: f32,
        cmap: String,
        alpha: f32,
    ) -> Self {
        let levels = levels.unwrap_or_else(|| auto_levels(&z, 10));
        let z_off = z_offset.unwrap_or_else(|| {
            // Default: draw at the minimum Z value
            let mut zmin = f64::INFINITY;
            for row in &z {
                for &v in row {
                    if v < zmin { zmin = v; }
                }
            }
            if zmin.is_infinite() { 0.0 } else { zmin }
        });

        Contour3D {
            x, y, z, levels,
            z_offset: z_off,
            filled,
            linewidth,
            cmap,
            alpha,
        }
    }
}

/// Auto-generate equally spaced levels from the Z data range.
fn auto_levels(z: &[Vec<f64>], n: usize) -> Vec<f64> {
    let mut zmin = f64::INFINITY;
    let mut zmax = f64::NEG_INFINITY;
    for row in z {
        for &v in row {
            if v.is_finite() {
                if v < zmin { zmin = v; }
                if v > zmax { zmax = v; }
            }
        }
    }
    if !zmin.is_finite() || !zmax.is_finite() || (zmax - zmin).abs() < 1e-15 {
        return vec![0.0];
    }
    let step = (zmax - zmin) / (n as f64 + 1.0);
    (1..=n).map(|i| zmin + i as f64 * step).collect()
}

/// Map a normalized value [0,1] to a colormap color.
fn colormap_value(t: f64, cmap: &str) -> Color {
    let t = t.clamp(0.0, 1.0) as f32;
    match cmap {
        "coolwarm" => {
            let r = if t < 0.5 { 0.2 + t * 1.2 } else { 1.0 };
            let g = if t < 0.5 { 0.2 + t * 0.8 } else { 1.0 - (t - 0.5) * 1.6 };
            let b = if t < 0.5 { 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
            Color::from_f32(r, g, b, 1.0)
        }
        _ => {
            // Default: viridis-like
            let r = (0.267 + t * 0.6).min(1.0);
            let g = (0.004 + t * 0.87).min(1.0);
            let b = (0.329 + (1.0 - t) * 0.5).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
    }
}

/// Linear interpolation between two points at a threshold.
fn lerp_point(x0: f64, y0: f64, v0: f64, x1: f64, y1: f64, v1: f64, level: f64) -> (f64, f64) {
    let dv = v1 - v0;
    let t = if dv.abs() < 1e-15 { 0.5 } else { (level - v0) / dv };
    (x0 + t * (x1 - x0), y0 + t * (y1 - y0))
}

impl Artist3D for Contour3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        let rows = self.z.len();
        if rows < 2 { return; }
        let cols = self.z[0].len();
        if cols < 2 { return; }

        let ts = tiny_skia::Transform::identity();

        // Find z range for colormap
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;
        for row in &self.z {
            for &v in row {
                if v < zmin { zmin = v; }
                if v > zmax { zmax = v; }
            }
        }
        let z_range = (zmax - zmin).max(1e-15);

        for (_li, &level) in self.levels.iter().enumerate() {
            let t = (level - zmin) / z_range;
            let mut color = colormap_value(t, &self.cmap);
            color.a = (self.alpha * 255.0) as u8;

            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            let mut stroke = Stroke::default();
            stroke.width = self.linewidth;

            // March through each cell using simple marching squares
            for r in 0..rows - 1 {
                for c in 0..cols - 1 {
                    let v00 = self.z[r][c];
                    let v10 = self.z[r][c + 1];
                    let v01 = self.z[r + 1][c];
                    let v11 = self.z[r + 1][c + 1];

                    let x00 = self.x[r][c];
                    let x10 = self.x[r][c + 1];
                    let x01 = self.x[r + 1][c];
                    let x11 = self.x[r + 1][c + 1];

                    let y00 = self.y[r][c];
                    let y10 = self.y[r][c + 1];
                    let y01 = self.y[r + 1][c];
                    let y11 = self.y[r + 1][c + 1];

                    // Build marching squares case index
                    let case = ((v00 >= level) as u8)
                        | (((v10 >= level) as u8) << 1)
                        | (((v11 >= level) as u8) << 2)
                        | (((v01 >= level) as u8) << 3);

                    // Get line segments for this case
                    let segments = marching_squares_segments(
                        case, level,
                        x00, y00, v00,
                        x10, y10, v10,
                        x11, y11, v11,
                        x01, y01, v01,
                    );

                    for (p1, p2) in segments {
                        // Project both endpoints to 3D at z_offset
                        let (nx1, ny1, nz1) = bounds.normalize(p1.0, p1.1, self.z_offset);
                        let (sx1, sy1, _depth1) = camera.project(nx1, ny1, nz1);
                        let (px1, py1) = plot_area.map_to_pixel(sx1, sy1);

                        let (nx2, ny2, nz2) = bounds.normalize(p2.0, p2.1, self.z_offset);
                        let (sx2, sy2, _depth2) = camera.project(nx2, ny2, nz2);
                        let (px2, py2) = plot_area.map_to_pixel(sx2, sy2);

                        let mut pb = PathBuilder::new();
                        pb.move_to(px1, py1);
                        pb.line_to(px2, py2);
                        if let Some(path) = pb.finish() {
                            pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                        }
                    }
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let mut xmin = f64::INFINITY;
        let mut xmax = f64::NEG_INFINITY;
        let mut ymin = f64::INFINITY;
        let mut ymax = f64::NEG_INFINITY;
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;
        for r in 0..self.x.len() {
            for c in 0..self.x[r].len() {
                let x = self.x[r][c];
                let y = self.y[r][c];
                let z = self.z[r][c];
                if x < xmin { xmin = x; }
                if x > xmax { xmax = x; }
                if y < ymin { ymin = y; }
                if y > ymax { ymax = y; }
                if z < zmin { zmin = z; }
                if z > zmax { zmax = z; }
            }
        }
        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }
}

/// Return line segment endpoints for marching squares case.
/// Each cell has 4 edges:
///   bottom: (x00,y00)-(x10,y10)
///   right:  (x10,y10)-(x11,y11)
///   top:    (x11,y11)-(x01,y01)
///   left:   (x01,y01)-(x00,y00)
fn marching_squares_segments(
    case: u8,
    level: f64,
    x00: f64, y00: f64, v00: f64,
    x10: f64, y10: f64, v10: f64,
    x11: f64, y11: f64, v11: f64,
    x01: f64, y01: f64, v01: f64,
) -> Vec<((f64, f64), (f64, f64))> {
    // Interpolation points on each edge
    let bottom = || lerp_point(x00, y00, v00, x10, y10, v10, level);
    let right  = || lerp_point(x10, y10, v10, x11, y11, v11, level);
    let top_e  = || lerp_point(x11, y11, v11, x01, y01, v01, level);
    let left   = || lerp_point(x01, y01, v01, x00, y00, v00, level);

    match case {
        0 | 15 => vec![],
        1 => vec![(bottom(), left())],
        2 => vec![(bottom(), right())],
        3 => vec![(left(), right())],
        4 => vec![(right(), top_e())],
        5 => vec![(bottom(), right()), (left(), top_e())],
        6 => vec![(bottom(), top_e())],
        7 => vec![(left(), top_e())],
        8 => vec![(left(), top_e())],
        9 => vec![(bottom(), top_e())],
        10 => vec![(bottom(), left()), (right(), top_e())],
        11 => vec![(right(), top_e())],
        12 => vec![(left(), right())],
        13 => vec![(bottom(), right())],
        14 => vec![(bottom(), left())],
        _ => vec![],
    }
}
