use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap};

/// Fill between two x-curves for a shared set of y values (horizontal bands).
pub struct FillBetweenX {
    pub y: Vec<f64>,
    pub x1: Vec<f64>,
    pub x2: Vec<f64>,
    pub color: Color,
    pub alpha: f32,
    pub label: Option<String>,
}

impl FillBetweenX {
    pub fn new(y: Vec<f64>, x1: Vec<f64>, x2: Vec<f64>, color: Color, alpha: f32) -> Self {
        Self {
            y,
            x1,
            x2,
            color,
            alpha,
            label: None,
        }
    }
}

impl Artist for FillBetweenX {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.y.is_empty() || self.x1.is_empty() || self.x2.is_empty() {
            return;
        }
        let n = self.y.len().min(self.x1.len()).min(self.x2.len());
        if n < 2 {
            return;
        }

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut paint = Paint::default();
        paint.set_color(fill_color.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Build a closed path: forward along (x1, y), then backward along (x2, y)
        let mut pb = PathBuilder::new();

        // Forward along x1
        let (px, py) = transform.transform_xy(self.x1[0], self.y[0]);
        pb.move_to(px, py);
        for i in 1..n {
            let (px, py) = transform.transform_xy(self.x1[i], self.y[i]);
            pb.line_to(px, py);
        }

        // Backward along x2
        for i in (0..n).rev() {
            let (px, py) = transform.transform_xy(self.x2[i], self.y[i]);
            pb.line_to(px, py);
        }

        pb.close();

        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.y.is_empty() || self.x1.is_empty() || self.x2.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.y.len().min(self.x1.len()).min(self.x2.len());
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            if self.y[i] < ymin {
                ymin = self.y[i];
            }
            if self.y[i] > ymax {
                ymax = self.y[i];
            }
            let x_lo = self.x1[i].min(self.x2[i]);
            let x_hi = self.x1[i].max(self.x2[i]);
            if x_lo < xmin {
                xmin = x_lo;
            }
            if x_hi > xmax {
                xmax = x_hi;
            }
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
