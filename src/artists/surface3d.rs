use tiny_skia::{Paint, PathBuilder, Pixmap, FillRule};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D surface plot.
pub struct Surface3D {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<Vec<f64>>,
    pub z: Vec<Vec<f64>>,
    pub cmap: String,
    pub alpha: f32,
}

/// A single projected quad with average depth and color.
struct Quad {
    vertices: [(f32, f32); 4],
    depth: f64,
    color: Color,
}

/// Map a value from [0, 1] to a colormap color.
fn colormap_value(t: f64, cmap: &str) -> Color {
    let t = t.clamp(0.0, 1.0);
    match cmap {
        "coolwarm" => {
            let r = if t < 0.5 { 0.2 + t * 1.2 } else { 1.0 };
            let g = if t < 0.5 { 0.2 + t * 0.8 } else { 1.0 - (t - 0.5) * 1.6 };
            let b = if t < 0.5 { 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
            Color::from_f32(r as f32, g as f32, b as f32, 1.0)
        }
        "plasma" => {
            let r = (0.05 + t * 0.9).min(1.0);
            let g = (t * 0.7).min(1.0);
            let b = (0.5 + (1.0 - t) * 0.5).min(1.0);
            Color::from_f32(r as f32, g as f32, b as f32, 1.0)
        }
        "hot" => {
            let r = (t * 3.0).min(1.0);
            let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
            let b = ((t - 0.67) * 3.0).clamp(0.0, 1.0);
            Color::from_f32(r as f32, g as f32, b as f32, 1.0)
        }
        _ => {
            // Default: viridis-like
            let r = (0.267 + t * 0.6).min(1.0);
            let g = (0.004 + t * 0.87).min(1.0);
            let b = (0.329 + (1.0 - t) * 0.5).clamp(0.0, 1.0);
            Color::from_f32(r as f32, g as f32, b as f32, 1.0)
        }
    }
}

impl Artist3D for Surface3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        let rows = self.z.len();
        if rows < 2 {
            return;
        }
        let cols = self.z[0].len();
        if cols < 2 {
            return;
        }

        // Find z range for colormap
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;
        for row in &self.z {
            for &v in row {
                if v < zmin { zmin = v; }
                if v > zmax { zmax = v; }
            }
        }
        let zrange = if (zmax - zmin).abs() > 1e-10 { zmax - zmin } else { 1.0 };

        // Build quads
        let mut quads: Vec<Quad> = Vec::new();
        for i in 0..rows - 1 {
            for j in 0..cols - 1 {
                let corners_3d = [
                    (self.x[i][j], self.y[i][j], self.z[i][j]),
                    (self.x[i][j + 1], self.y[i][j + 1], self.z[i][j + 1]),
                    (self.x[i + 1][j + 1], self.y[i + 1][j + 1], self.z[i + 1][j + 1]),
                    (self.x[i + 1][j], self.y[i + 1][j], self.z[i + 1][j]),
                ];

                let mut total_depth = 0.0;
                let mut avg_z = 0.0;
                let mut vertices = [(0.0f32, 0.0f32); 4];

                for (k, &(x, y, z)) in corners_3d.iter().enumerate() {
                    let (nx, ny, nz) = bounds.normalize(x, y, z);
                    let (sx, sy, depth) = camera.project(nx, ny, nz);
                    vertices[k] = plot_area.map_to_pixel(sx, sy);
                    total_depth += depth;
                    avg_z += z;
                }

                let t = (avg_z / 4.0 - zmin) / zrange;
                let color = colormap_value(t, &self.cmap);

                quads.push(Quad {
                    vertices,
                    depth: total_depth / 4.0,
                    color,
                });
            }
        }

        // Sort back to front (ascending depth)
        quads.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));

        let ts = tiny_skia::Transform::identity();

        // Draw quads
        for quad in &quads {
            let mut pb = PathBuilder::new();
            pb.move_to(quad.vertices[0].0, quad.vertices[0].1);
            pb.line_to(quad.vertices[1].0, quad.vertices[1].1);
            pb.line_to(quad.vertices[2].0, quad.vertices[2].1);
            pb.line_to(quad.vertices[3].0, quad.vertices[3].1);
            pb.close();

            if let Some(path) = pb.finish() {
                let mut fill_paint = Paint::default();
                let mut c = quad.color;
                c.a = (self.alpha * 255.0) as u8;
                fill_paint.set_color(c.to_tiny_skia());
                fill_paint.anti_alias = true;
                pixmap.fill_path(&path, &fill_paint, FillRule::Winding, ts, None);

                // Thin edge
                let mut edge_paint = Paint::default();
                edge_paint.set_color_rgba8(
                    (quad.color.r as u16 * 3 / 4) as u8,
                    (quad.color.g as u16 * 3 / 4) as u8,
                    (quad.color.b as u16 * 3 / 4) as u8,
                    (self.alpha * 200.0) as u8,
                );
                edge_paint.anti_alias = true;
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &edge_paint, &stroke, ts, None);
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

        for (i, row) in self.z.iter().enumerate() {
            for (j, &z) in row.iter().enumerate() {
                let x = self.x[i][j];
                let y = self.y[i][j];
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
        Color::new(31, 119, 180, 255)
    }
}
