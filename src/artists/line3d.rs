use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D line plot.
pub struct Line3D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub color: Color,
    pub linewidth: f32,
    pub label: Option<String>,
}

impl Artist3D for Line3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        if self.x.len() < 2 {
            return;
        }

        let mut paint = Paint::default();
        paint.set_color(self.color.to_tiny_skia());
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = self.linewidth;

        let ts = tiny_skia::Transform::identity();

        // Project all points
        let projected: Vec<(f32, f32)> = self
            .x
            .iter()
            .zip(self.y.iter())
            .zip(self.z.iter())
            .map(|((&x, &y), &z)| {
                let (nx, ny, nz) = bounds.normalize(x, y, z);
                let (sx, sy, _) = camera.project(nx, ny, nz);
                plot_area.map_to_pixel(sx, sy)
            })
            .collect();

        // Draw line segments
        let mut pb = PathBuilder::new();
        pb.move_to(projected[0].0, projected[0].1);
        for &(px, py) in &projected[1..] {
            pb.line_to(px, py);
        }
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &paint, &stroke, ts, None);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let xmin = self.x.iter().cloned().fold(f64::INFINITY, f64::min);
        let xmax = self.x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ymin = self.y.iter().cloned().fold(f64::INFINITY, f64::min);
        let ymax = self.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let zmin = self.z.iter().cloned().fold(f64::INFINITY, f64::min);
        let zmax = self.z.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
