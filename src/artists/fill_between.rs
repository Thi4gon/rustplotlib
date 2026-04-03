use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap};

pub struct FillBetween {
    pub x: Vec<f64>,
    pub y1: Vec<f64>,
    pub y2: Vec<f64>,
    pub color: Color,
    pub alpha: f32,
    pub label: Option<String>,
}

impl FillBetween {
    pub fn new(x: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>, color: Color, alpha: f32) -> Self {
        Self {
            x,
            y1,
            y2,
            color,
            alpha,
            label: None,
        }
    }
}

impl Artist for FillBetween {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y1.is_empty() || self.y2.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y1.len()).min(self.y2.len());
        if n < 2 {
            return;
        }

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut paint = Paint::default();
        paint.set_color(fill_color.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Build a closed path: forward along (x, y1), then backward along (x, y2)
        let mut pb = PathBuilder::new();

        // Forward along y1
        let (px, py) = transform.transform_xy(self.x[0], self.y1[0]);
        pb.move_to(px, py);
        for i in 1..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y1[i]);
            pb.line_to(px, py);
        }

        // Backward along y2
        for i in (0..n).rev() {
            let (px, py) = transform.transform_xy(self.x[i], self.y2[i]);
            pb.line_to(px, py);
        }

        pb.close();

        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.x.is_empty() || self.y1.is_empty() || self.y2.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.x.len().min(self.y1.len()).min(self.y2.len());
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            let y_lo = self.y1[i].min(self.y2[i]);
            let y_hi = self.y1[i].max(self.y2[i]);
            if y_lo < ymin { ymin = y_lo; }
            if y_hi > ymax { ymax = y_hi; }
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
