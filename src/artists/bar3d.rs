use tiny_skia::{Paint, PathBuilder, Pixmap, FillRule, Stroke};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D bar chart.
pub struct Bar3D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub dx: Vec<f64>,
    pub dy: Vec<f64>,
    pub dz: Vec<f64>,
    pub color: Color,
    pub alpha: f32,
}

/// A single face with projected vertices, depth, and color.
struct Face {
    vertices: [(f32, f32); 4],
    depth: f64,
    color: Color,
    alpha: f32,
}

impl Artist3D for Bar3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        let n = self.x.len();
        if n == 0 {
            return;
        }

        let ts = tiny_skia::Transform::identity();
        let mut all_faces: Vec<Face> = Vec::new();

        for i in 0..n {
            let x0 = self.x[i];
            let y0 = self.y[i];
            let z0 = self.z[i];
            let dx = if i < self.dx.len() { self.dx[i] } else { 0.8 };
            let dy = if i < self.dy.len() { self.dy[i] } else { 0.8 };
            let dz = if i < self.dz.len() { self.dz[i] } else { 1.0 };
            let x1 = x0 + dx;
            let y1 = y0 + dy;
            let z1 = z0 + dz;

            // 8 corners of the box
            let corners = [
                (x0, y0, z0), // 0
                (x1, y0, z0), // 1
                (x1, y1, z0), // 2
                (x0, y1, z0), // 3
                (x0, y0, z1), // 4
                (x1, y0, z1), // 5
                (x1, y1, z1), // 6
                (x0, y1, z1), // 7
            ];

            // Project corners
            let proj: Vec<(f32, f32, f64)> = corners
                .iter()
                .map(|&(cx, cy, cz)| {
                    let (nx, ny, nz) = bounds.normalize(cx, cy, cz);
                    let (sx, sy, depth) = camera.project(nx, ny, nz);
                    let (px, py) = plot_area.map_to_pixel(sx, sy);
                    (px, py, depth)
                })
                .collect();

            // 6 faces of the box (each defined by 4 corner indices)
            let face_defs: [(usize, usize, usize, usize, f32, f32, f32); 6] = [
                // indices, color brightness adjustments (r_factor, g_factor, b_factor)
                (0, 1, 2, 3, 0.6, 0.6, 0.6), // bottom
                (4, 5, 6, 7, 1.0, 1.0, 1.0), // top
                (0, 1, 5, 4, 0.8, 0.8, 0.8), // front
                (2, 3, 7, 6, 0.7, 0.7, 0.7), // back
                (0, 3, 7, 4, 0.75, 0.75, 0.75), // left
                (1, 2, 6, 5, 0.85, 0.85, 0.85), // right
            ];

            for &(a, b, c, d, rf, gf, bf) in &face_defs {
                let vertices = [
                    (proj[a].0, proj[a].1),
                    (proj[b].0, proj[b].1),
                    (proj[c].0, proj[c].1),
                    (proj[d].0, proj[d].1),
                ];
                let avg_depth = (proj[a].2 + proj[b].2 + proj[c].2 + proj[d].2) / 4.0;
                let face_color = Color::new(
                    (self.color.r as f32 * rf).min(255.0) as u8,
                    (self.color.g as f32 * gf).min(255.0) as u8,
                    (self.color.b as f32 * bf).min(255.0) as u8,
                    self.color.a,
                );
                all_faces.push(Face {
                    vertices,
                    depth: avg_depth,
                    color: face_color,
                    alpha: self.alpha,
                });
            }
        }

        // Sort faces back to front
        all_faces.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));

        // Draw faces
        for face in &all_faces {
            let mut pb = PathBuilder::new();
            pb.move_to(face.vertices[0].0, face.vertices[0].1);
            pb.line_to(face.vertices[1].0, face.vertices[1].1);
            pb.line_to(face.vertices[2].0, face.vertices[2].1);
            pb.line_to(face.vertices[3].0, face.vertices[3].1);
            pb.close();

            if let Some(path) = pb.finish() {
                // Fill
                let mut fill_paint = Paint::default();
                let mut c = face.color;
                c.a = (face.alpha * 255.0) as u8;
                fill_paint.set_color(c.to_tiny_skia());
                fill_paint.anti_alias = true;
                pixmap.fill_path(&path, &fill_paint, FillRule::Winding, ts, None);

                // Edge
                let mut edge_paint = Paint::default();
                edge_paint.set_color_rgba8(60, 60, 60, (face.alpha * 200.0) as u8);
                edge_paint.anti_alias = true;
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &edge_paint, &stroke, ts, None);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let n = self.x.len();
        if n == 0 {
            return (0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        }

        let mut xmin = f64::INFINITY;
        let mut xmax = f64::NEG_INFINITY;
        let mut ymin = f64::INFINITY;
        let mut ymax = f64::NEG_INFINITY;
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;

        for i in 0..n {
            let dx = if i < self.dx.len() { self.dx[i] } else { 0.8 };
            let dy = if i < self.dy.len() { self.dy[i] } else { 0.8 };
            let dz = if i < self.dz.len() { self.dz[i] } else { 1.0 };
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] + dx > xmax { xmax = self.x[i] + dx; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] + dy > ymax { ymax = self.y[i] + dy; }
            if self.z[i] < zmin { zmin = self.z[i]; }
            if self.z[i] + dz > zmax { zmax = self.z[i] + dz; }
        }

        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
