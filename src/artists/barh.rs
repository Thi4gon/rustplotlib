use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

pub struct BarH {
    pub y: Vec<f64>,
    pub widths: Vec<f64>,
    pub height: f64,
    pub color: Color,
    pub label: Option<String>,
    pub alpha: f32,
}

impl BarH {
    pub fn new(y: Vec<f64>, widths: Vec<f64>, color: Color) -> Self {
        Self {
            y,
            widths,
            height: 0.8,
            color,
            label: None,
            alpha: 1.0,
        }
    }
}

impl Artist for BarH {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.y.is_empty() || self.widths.is_empty() {
            return;
        }
        let n = self.y.len().min(self.widths.len());

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut fill_paint = Paint::default();
        fill_paint.set_color(fill_color.to_tiny_skia());
        fill_paint.anti_alias = true;

        // Border paint (slightly darker)
        let mut border_paint = Paint::default();
        let border_color = Color::new(
            (fill_color.r as u16 * 7 / 10) as u8,
            (fill_color.g as u16 * 7 / 10) as u8,
            (fill_color.b as u16 * 7 / 10) as u8,
            fill_color.a,
        );
        border_paint.set_color(border_color.to_tiny_skia());
        border_paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();
        let half_h = self.height / 2.0;

        for i in 0..n {
            let w = self.widths[i];
            if w.abs() < 1e-15 {
                continue;
            }

            let y_bottom = self.y[i] - half_h;
            let y_top = self.y[i] + half_h;

            // Bar goes from x=0 to x=width
            let (x_left, x_right) = if w >= 0.0 { (0.0, w) } else { (w, 0.0) };

            let (px_left, py_top) = transform.transform_xy(x_left, y_top);
            let (px_right, py_bottom) = transform.transform_xy(x_right, y_bottom);

            let rect_x = px_left.min(px_right);
            let rect_y = py_top.min(py_bottom);
            let rect_w = (px_right - px_left).abs().max(1.0);
            let rect_h = (py_bottom - py_top).abs().max(1.0);

            if let Some(rect) = Rect::from_xywh(rect_x, rect_y, rect_w, rect_h) {
                // Fill
                pixmap.fill_rect(rect, &fill_paint, ts, None);

                // Border stroke
                let path = PathBuilder::from_rect(rect);
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.y.is_empty() || self.widths.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.y.len().min(self.widths.len());
        let half_h = self.height / 2.0;

        let mut xmin = 0.0_f64;
        let mut xmax = 0.0_f64;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for i in 0..n {
            let yb = self.y[i] - half_h;
            let yt = self.y[i] + half_h;
            if yb < ymin { ymin = yb; }
            if yt > ymax { ymax = yt; }
            if self.widths[i] < xmin { xmin = self.widths[i]; }
            if self.widths[i] > xmax { xmax = self.widths[i]; }
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
