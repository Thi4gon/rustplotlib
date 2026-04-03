use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

pub struct Quiver {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    pub color: Color,
    pub scale: f64,
    pub width: f32,
}

impl Quiver {
    pub fn new(
        x: Vec<f64>,
        y: Vec<f64>,
        u: Vec<f64>,
        v: Vec<f64>,
        color: Color,
        scale: f64,
        width: f32,
    ) -> Self {
        Self { x, y, u, v, color, scale, width }
    }
}

impl Artist for Quiver {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let n = self.x.len()
            .min(self.y.len())
            .min(self.u.len())
            .min(self.v.len());
        if n == 0 {
            return;
        }

        let ts = tiny_skia::Transform::identity();
        let mut paint = Paint::default();
        paint.set_color(self.color.to_tiny_skia());
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = self.width;

        let scale = if self.scale > 0.0 { self.scale } else { 1.0 };

        for i in 0..n {
            let x0 = self.x[i];
            let y0 = self.y[i];
            let x1 = x0 + self.u[i] / scale;
            let y1 = y0 + self.v[i] / scale;

            let (px0, py0) = transform.transform_xy(x0, y0);
            let (px1, py1) = transform.transform_xy(x1, y1);

            // Draw shaft
            let mut pb = PathBuilder::new();
            pb.move_to(px0, py0);
            pb.line_to(px1, py1);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Draw arrowhead
            let dx = px1 - px0;
            let dy = py1 - py0;
            let len = (dx * dx + dy * dy).sqrt();
            if len > 2.0 {
                let ux = dx / len;
                let uy = dy / len;
                let head_size = (self.width * 4.0).max(6.0);
                let perp_x = -uy;
                let perp_y = ux;
                let base_x = px1 - ux * head_size;
                let base_y = py1 - uy * head_size;

                let mut pb = PathBuilder::new();
                pb.move_to(px1, py1);
                pb.line_to(
                    base_x + perp_x * head_size * 0.35,
                    base_y + perp_y * head_size * 0.35,
                );
                pb.line_to(
                    base_x - perp_x * head_size * 0.35,
                    base_y - perp_y * head_size * 0.35,
                );
                pb.close();
                if let Some(path) = pb.finish() {
                    pixmap.fill_path(
                        &path,
                        &paint,
                        tiny_skia::FillRule::Winding,
                        ts,
                        None,
                    );
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let n = self.x.len()
            .min(self.y.len())
            .min(self.u.len())
            .min(self.v.len());
        if n == 0 {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let scale = if self.scale > 0.0 { self.scale } else { 1.0 };
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            let x0 = self.x[i];
            let y0 = self.y[i];
            let x1 = x0 + self.u[i] / scale;
            let y1 = y0 + self.v[i] / scale;
            if x0 < xmin { xmin = x0; }
            if x1 < xmin { xmin = x1; }
            if x0 > xmax { xmax = x0; }
            if x1 > xmax { xmax = x1; }
            if y0 < ymin { ymin = y0; }
            if y1 < ymin { ymin = y1; }
            if y0 > ymax { ymax = y0; }
            if y1 > ymax { ymax = y1; }
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
