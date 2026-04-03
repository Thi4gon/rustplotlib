use crate::artists::{Artist, MarkerStyle, draw_marker};
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

pub struct Stem {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Color,
    pub linewidth: f32,
    pub marker: MarkerStyle,
    pub marker_size: f32,
    pub label: Option<String>,
    pub baseline: f64,
}

impl Stem {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            linewidth: 1.0,
            marker: MarkerStyle::Circle,
            marker_size: 6.0,
            label: None,
            baseline: 0.0,
        }
    }
}

impl Artist for Stem {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());

        let mut color = self.color;
        color.a = 255;
        let ts_color = color.to_tiny_skia();

        let mut paint = Paint::default();
        paint.set_color(ts_color);
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        let mut stroke = Stroke::default();
        stroke.width = self.linewidth;

        // Draw baseline
        if n >= 1 {
            let xmin = self.x.iter().copied().fold(f64::MAX, f64::min);
            let xmax = self.x.iter().copied().fold(f64::MIN, f64::max);
            let (px_left, py_base) = transform.transform_xy(xmin, self.baseline);
            let (px_right, _) = transform.transform_xy(xmax, self.baseline);

            let mut baseline_paint = Paint::default();
            baseline_paint.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 180));
            baseline_paint.anti_alias = true;

            let mut baseline_stroke = Stroke::default();
            baseline_stroke.width = 1.0;

            let mut pb = PathBuilder::new();
            pb.move_to(px_left, py_base);
            pb.line_to(px_right, py_base);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &baseline_paint, &baseline_stroke, ts, None);
            }
        }

        // Draw stems (vertical lines from baseline to data points)
        for i in 0..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
            let (_, py_base) = transform.transform_xy(self.x[i], self.baseline);

            // Vertical line
            let mut pb = PathBuilder::new();
            pb.move_to(px, py_base);
            pb.line_to(px, py);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Marker at the data point
            if self.marker != MarkerStyle::None {
                draw_marker(pixmap, self.marker, px, py, self.marker_size, self.color, 1.0);
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
        let mut ymin = self.baseline;
        let mut ymax = self.baseline;
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

    fn legend_entry(&self) -> Option<LegendEntry> {
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.color,
            line_style: None,
            marker: if self.marker != MarkerStyle::None { Some(self.marker) } else { None },
            linewidth: self.linewidth,
        })
    }
}
