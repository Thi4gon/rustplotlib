use crate::artists::{Artist, LineStyle, MarkerStyle, draw_marker};
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

pub struct Line2D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Color,
    pub linewidth: f32,
    pub linestyle: LineStyle,
    pub marker: MarkerStyle,
    pub marker_size: f32,
    pub label: Option<String>,
    pub alpha: f32,
}

impl Line2D {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            linewidth: 1.5,
            linestyle: LineStyle::Solid,
            marker: MarkerStyle::None,
            marker_size: 6.0,
            label: None,
            alpha: 1.0,
        }
    }
}

impl Artist for Line2D {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());

        // Set up paint with alpha
        let mut color = self.color;
        color.a = (self.alpha * 255.0) as u8;
        let ts_color = color.to_tiny_skia();

        let mut paint = Paint::default();
        paint.set_color(ts_color);
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Draw line segments if linestyle is not None
        if self.linestyle != LineStyle::None && n >= 2 {
            let mut pb = PathBuilder::new();
            let (px, py) = transform.transform_xy(self.x[0], self.y[0]);
            pb.move_to(px, py);
            for i in 1..n {
                let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
                pb.line_to(px, py);
            }
            if let Some(path) = pb.finish() {
                let mut stroke = Stroke::default();
                stroke.width = self.linewidth;
                stroke.dash = self.linestyle.to_dash(self.linewidth);
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }

        // Draw markers
        if self.marker != MarkerStyle::None {
            for i in 0..n {
                let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
                draw_marker(pixmap, self.marker, px, py, self.marker_size, self.color, self.alpha);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.x.is_empty() || self.y.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.x.len().min(self.y.len());
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] > ymax { ymax = self.y[i]; }
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
