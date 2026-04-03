use crate::artists::{Artist, LineStyle};
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepWhere {
    Pre,   // step at the left edge
    Post,  // step at the right edge
    Mid,   // step at the middle
}

impl StepWhere {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pre" => StepWhere::Pre,
            "mid" | "middle" => StepWhere::Mid,
            _ => StepWhere::Post,
        }
    }
}

pub struct Step {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Color,
    pub linewidth: f32,
    pub linestyle: LineStyle,
    pub label: Option<String>,
    pub alpha: f32,
    pub where_style: StepWhere,
}

impl Step {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            linewidth: 1.5,
            linestyle: LineStyle::Solid,
            label: None,
            alpha: 1.0,
            where_style: StepWhere::Pre,
        }
    }
}

impl Artist for Step {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());
        if n < 2 {
            return;
        }

        let mut color = self.color;
        color.a = (self.alpha * 255.0) as u8;

        let mut paint = Paint::default();
        paint.set_color(color.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();
        let mut pb = PathBuilder::new();

        match self.where_style {
            StepWhere::Pre => {
                // Step happens at the beginning of each interval:
                // From (x[i-1], y[i-1]) -> (x[i], y[i-1]) -> (x[i], y[i])
                let (px, py) = transform.transform_xy(self.x[0], self.y[0]);
                pb.move_to(px, py);
                for i in 1..n {
                    // Horizontal to new x at old y
                    let (px_new, _) = transform.transform_xy(self.x[i], self.y[i - 1]);
                    let (_, py_old) = transform.transform_xy(self.x[i - 1], self.y[i - 1]);
                    pb.line_to(px_new, py_old);
                    // Vertical to new y
                    let (_, py_new) = transform.transform_xy(self.x[i], self.y[i]);
                    pb.line_to(px_new, py_new);
                }
            }
            StepWhere::Post => {
                // Step happens at the end of each interval:
                // From (x[i], y[i]) -> (x[i], y[i+1]) -> (x[i+1], y[i+1])
                let (px, py) = transform.transform_xy(self.x[0], self.y[0]);
                pb.move_to(px, py);
                for i in 1..n {
                    // Vertical to new y at old x
                    let (px_old, _) = transform.transform_xy(self.x[i - 1], self.y[i - 1]);
                    let (_, py_new) = transform.transform_xy(self.x[i], self.y[i]);
                    pb.line_to(px_old, py_new);
                    // Horizontal to new x
                    let (px_new, _) = transform.transform_xy(self.x[i], self.y[i]);
                    pb.line_to(px_new, py_new);
                }
            }
            StepWhere::Mid => {
                // Step happens at the midpoint between x values
                let (px, py) = transform.transform_xy(self.x[0], self.y[0]);
                pb.move_to(px, py);
                for i in 1..n {
                    let mid_x = (self.x[i - 1] + self.x[i]) / 2.0;
                    // Horizontal to midpoint at old y
                    let (px_mid, py_old) = transform.transform_xy(mid_x, self.y[i - 1]);
                    pb.line_to(px_mid, py_old);
                    // Vertical to new y at midpoint
                    let (_, py_new) = transform.transform_xy(mid_x, self.y[i]);
                    pb.line_to(px_mid, py_new);
                    // Horizontal to next x at new y
                    let (px_new, _) = transform.transform_xy(self.x[i], self.y[i]);
                    pb.line_to(px_new, py_new);
                }
            }
        }

        if let Some(path) = pb.finish() {
            let mut stroke = Stroke::default();
            stroke.width = self.linewidth;
            stroke.dash = self.linestyle.to_dash(self.linewidth);
            pixmap.stroke_path(&path, &paint, &stroke, ts, None);
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

    fn legend_entry(&self) -> Option<LegendEntry> {
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.color,
            line_style: Some(self.linestyle),
            marker: None,
            linewidth: self.linewidth,
        })
    }
}
