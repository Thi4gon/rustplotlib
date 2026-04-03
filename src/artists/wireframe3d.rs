use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D wireframe plot.
pub struct Wireframe3D {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<Vec<f64>>,
    pub z: Vec<Vec<f64>>,
    pub color: Color,
    pub linewidth: f32,
}

impl Artist3D for Wireframe3D {
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

        let mut paint = Paint::default();
        paint.set_color(self.color.to_tiny_skia());
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = self.linewidth;

        let ts = tiny_skia::Transform::identity();

        // Project all points
        let mut projected: Vec<Vec<(f32, f32)>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                let (nx, ny, nz) = bounds.normalize(self.x[i][j], self.y[i][j], self.z[i][j]);
                let (sx, sy, _) = camera.project(nx, ny, nz);
                row.push(plot_area.map_to_pixel(sx, sy));
            }
            projected.push(row);
        }

        // Draw row lines (constant i, varying j)
        for i in 0..rows {
            let mut pb = PathBuilder::new();
            pb.move_to(projected[i][0].0, projected[i][0].1);
            for j in 1..cols {
                pb.line_to(projected[i][j].0, projected[i][j].1);
            }
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }

        // Draw column lines (constant j, varying i)
        for j in 0..cols {
            let mut pb = PathBuilder::new();
            pb.move_to(projected[0][j].0, projected[0][j].1);
            for i in 1..rows {
                pb.line_to(projected[i][j].0, projected[i][j].1);
            }
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
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
        self.color
    }
}
