use crate::artists::{Artist, LineStyle, MarkerStyle, draw_marker};
use crate::artists::legend::LegendEntry;
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
    pub marker_every: usize,
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
            marker_every: 1,
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

        // Draw line segments if linestyle is not None (with NaN gap handling)
        if self.linestyle != LineStyle::None && n >= 2 {
            let mut pb = PathBuilder::new();
            let mut in_path = false;
            for i in 0..n {
                let x = self.x[i];
                let y = self.y[i];

                if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                    in_path = false;
                    continue;
                }

                let (px, py) = transform.transform_xy(x, y);
                if !in_path {
                    pb.move_to(px, py);
                    in_path = true;
                } else {
                    pb.line_to(px, py);
                }
            }
            if let Some(path) = pb.finish() {
                let mut stroke = Stroke::default();
                stroke.width = self.linewidth;
                stroke.dash = self.linestyle.to_dash(self.linewidth);
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }

        // Draw markers (respecting marker_every, skipping NaN/infinite points)
        if self.marker != MarkerStyle::None {
            let every = self.marker_every.max(1);
            for i in 0..n {
                if i % every == 0 {
                    let x = self.x[i];
                    let y = self.y[i];
                    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                        continue;
                    }
                    let (px, py) = transform.transform_xy(x, y);
                    draw_marker(pixmap, self.marker, px, py, self.marker_size, self.color, self.alpha);
                }
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
            let x = self.x[i];
            let y = self.y[i];
            if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                continue;
            }
            if x < xmin { xmin = x; }
            if x > xmax { xmax = x; }
            if y < ymin { ymin = y; }
            if y > ymax { ymax = y; }
        }
        if xmin == f64::MAX {
            return (0.0, 1.0, 0.0, 1.0);
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }

    fn legend_entry(&self) -> Option<LegendEntry> {
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.color,
            line_style: Some(self.linestyle),
            marker: if self.marker != MarkerStyle::None { Some(self.marker) } else { None },
            linewidth: self.linewidth,
        })
    }
}
