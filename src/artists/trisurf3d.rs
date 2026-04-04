use tiny_skia::{Paint, PathBuilder, Pixmap, FillRule};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D triangulated surface plot.
pub struct TriSurf3D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub triangles: Vec<(usize, usize, usize)>, // vertex index triples
    pub cmap: String,
    pub alpha: f32,
}

/// A single projected triangle with average depth and color.
struct Tri {
    vertices: [(f32, f32); 3],
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

impl TriSurf3D {
    /// Create a triangulated surface from flat point arrays.
    /// If no explicit triangles are given, auto-generate by sorting points
    /// and connecting consecutive triples in a sliding window.
    pub fn from_points(x: Vec<f64>, y: Vec<f64>, z: Vec<f64>, cmap: String, alpha: f32) -> Self {
        let n = x.len();
        let mut triangles = Vec::new();
        if n >= 3 {
            // Sort indices by x then y for a simple triangulation
            let mut indices: Vec<usize> = (0..n).collect();
            indices.sort_by(|&a, &b| {
                x[a].partial_cmp(&x[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then(y[a].partial_cmp(&y[b]).unwrap_or(std::cmp::Ordering::Equal))
            });
            // Sliding window: connect sequential triples
            for i in 0..indices.len().saturating_sub(2) {
                triangles.push((indices[i], indices[i + 1], indices[i + 2]));
            }
        }
        Self {
            x,
            y,
            z,
            triangles,
            cmap,
            alpha,
        }
    }

    /// Create with explicit triangle indices.
    pub fn with_triangles(
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        triangles: Vec<(usize, usize, usize)>,
        cmap: String,
        alpha: f32,
    ) -> Self {
        Self {
            x,
            y,
            z,
            triangles,
            cmap,
            alpha,
        }
    }
}

impl Artist3D for TriSurf3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        if self.triangles.is_empty() {
            return;
        }

        // Find z range for colormap
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;
        for &z in &self.z {
            if z < zmin {
                zmin = z;
            }
            if z > zmax {
                zmax = z;
            }
        }
        let zrange = if (zmax - zmin).abs() > 1e-10 {
            zmax - zmin
        } else {
            1.0
        };

        // Build projected triangles
        let mut tris: Vec<Tri> = Vec::new();
        for &(i0, i1, i2) in &self.triangles {
            if i0 >= self.x.len() || i1 >= self.x.len() || i2 >= self.x.len() {
                continue;
            }
            let pts = [(i0, self.x[i0], self.y[i0], self.z[i0]),
                       (i1, self.x[i1], self.y[i1], self.z[i1]),
                       (i2, self.x[i2], self.y[i2], self.z[i2])];

            let mut vertices = [(0.0f32, 0.0f32); 3];
            let mut total_depth = 0.0;
            let mut avg_z = 0.0;

            for (k, &(_, x, y, z)) in pts.iter().enumerate() {
                let (nx, ny, nz) = bounds.normalize(x, y, z);
                let (sx, sy, depth) = camera.project(nx, ny, nz);
                vertices[k] = plot_area.map_to_pixel(sx, sy);
                total_depth += depth;
                avg_z += z;
            }

            let t = (avg_z / 3.0 - zmin) / zrange;
            let color = colormap_value(t, &self.cmap);

            tris.push(Tri {
                vertices,
                depth: total_depth / 3.0,
                color,
            });
        }

        // Sort back to front (ascending depth)
        tris.sort_by(|a, b| {
            a.depth
                .partial_cmp(&b.depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let ts = tiny_skia::Transform::identity();

        // Draw triangles
        for tri in &tris {
            let mut pb = PathBuilder::new();
            pb.move_to(tri.vertices[0].0, tri.vertices[0].1);
            pb.line_to(tri.vertices[1].0, tri.vertices[1].1);
            pb.line_to(tri.vertices[2].0, tri.vertices[2].1);
            pb.close();

            if let Some(path) = pb.finish() {
                let mut fill_paint = Paint::default();
                let mut c = tri.color;
                c.a = (self.alpha * 255.0) as u8;
                fill_paint.set_color(c.to_tiny_skia());
                fill_paint.anti_alias = true;
                pixmap.fill_path(&path, &fill_paint, FillRule::Winding, ts, None);

                // Thin edge
                let mut edge_paint = Paint::default();
                edge_paint.set_color_rgba8(
                    (tri.color.r as u16 * 3 / 4) as u8,
                    (tri.color.g as u16 * 3 / 4) as u8,
                    (tri.color.b as u16 * 3 / 4) as u8,
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

        for i in 0..self.x.len() {
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] > ymax { ymax = self.y[i]; }
            if self.z[i] < zmin { zmin = self.z[i]; }
            if self.z[i] > zmax { zmax = self.z[i]; }
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
