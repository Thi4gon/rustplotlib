use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

use crate::artists::Artist;
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;

/// A collection of line segments — renders multiple lines in a single artist.
/// Each segment is a list of (x, y) points.
pub struct LineCollection {
    pub segments: Vec<Vec<(f64, f64)>>,
    pub colors: Vec<Color>,
    pub linewidths: Vec<f32>,
    pub default_color: Color,
    pub default_linewidth: f32,
    pub alpha: f32,
    pub label: Option<String>,
    pub zorder: i32,
}

impl LineCollection {
    pub fn new(segments: Vec<Vec<(f64, f64)>>, color: Color) -> Self {
        Self {
            segments,
            colors: Vec::new(),
            linewidths: Vec::new(),
            default_color: color,
            default_linewidth: 1.0,
            alpha: 1.0,
            label: None,
            zorder: 2,
        }
    }

    fn segment_color(&self, idx: usize) -> Color {
        if idx < self.colors.len() {
            self.colors[idx]
        } else {
            self.default_color
        }
    }

    fn segment_linewidth(&self, idx: usize) -> f32 {
        if idx < self.linewidths.len() {
            self.linewidths[idx]
        } else {
            self.default_linewidth
        }
    }
}

impl Artist for LineCollection {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();

        for (i, segment) in self.segments.iter().enumerate() {
            if segment.len() < 2 {
                continue;
            }

            let mut color = self.segment_color(i);
            color.a = (self.alpha * 255.0) as u8;

            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            let mut pb = PathBuilder::new();
            let (px, py) = transform.transform_xy(segment[0].0, segment[0].1);
            pb.move_to(px, py);
            for pt in &segment[1..] {
                let (px, py) = transform.transform_xy(pt.0, pt.1);
                pb.line_to(px, py);
            }
            if let Some(path) = pb.finish() {
                let mut stroke = Stroke::default();
                stroke.width = self.segment_linewidth(i);
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for segment in &self.segments {
            for (x, y) in segment {
                if *x < xmin { xmin = *x; }
                if *x > xmax { xmax = *x; }
                if *y < ymin { ymin = *y; }
                if *y > ymax { ymax = *y; }
            }
        }

        if xmin > xmax { (0.0, 1.0, 0.0, 1.0) }
        else { (xmin, xmax, ymin, ymax) }
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.default_color
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}
