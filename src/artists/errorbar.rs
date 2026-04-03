use crate::artists::{Artist, LineStyle, MarkerStyle, draw_marker};
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

pub struct ErrorBar {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub yerr: Option<Vec<f64>>,
    pub xerr: Option<Vec<f64>>,
    pub color: Color,
    pub linewidth: f32,
    pub capsize: f32,
    pub marker: MarkerStyle,
    pub marker_size: f32,
    pub label: Option<String>,
    pub alpha: f32,
    pub linestyle: LineStyle,
}

impl ErrorBar {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            yerr: None,
            xerr: None,
            color,
            linewidth: 1.5,
            capsize: 3.0,
            marker: MarkerStyle::None,
            marker_size: 6.0,
            label: None,
            alpha: 1.0,
            linestyle: LineStyle::Solid,
        }
    }
}

impl Artist for ErrorBar {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());

        let mut color = self.color;
        color.a = (self.alpha * 255.0) as u8;
        let ts_color = color.to_tiny_skia();

        let mut paint = Paint::default();
        paint.set_color(ts_color);
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Draw the data line (like Line2D)
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

        // Draw error bars
        let mut err_stroke = Stroke::default();
        err_stroke.width = self.linewidth * 0.75;

        for i in 0..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);

            // Y error bars
            if let Some(ref yerr) = self.yerr {
                if i < yerr.len() {
                    let err = yerr[i];
                    let (_, py_lo) = transform.transform_xy(self.x[i], self.y[i] - err);
                    let (_, py_hi) = transform.transform_xy(self.x[i], self.y[i] + err);

                    // Vertical line
                    let mut pb = PathBuilder::new();
                    pb.move_to(px, py_lo);
                    pb.line_to(px, py_hi);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                    }

                    // Horizontal caps
                    if self.capsize > 0.0 {
                        // Top cap
                        let mut pb = PathBuilder::new();
                        pb.move_to(px - self.capsize, py_hi);
                        pb.line_to(px + self.capsize, py_hi);
                        if let Some(path) = pb.finish() {
                            pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                        }
                        // Bottom cap
                        let mut pb = PathBuilder::new();
                        pb.move_to(px - self.capsize, py_lo);
                        pb.line_to(px + self.capsize, py_lo);
                        if let Some(path) = pb.finish() {
                            pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                        }
                    }
                }
            }

            // X error bars
            if let Some(ref xerr) = self.xerr {
                if i < xerr.len() {
                    let err = xerr[i];
                    let (px_lo, _) = transform.transform_xy(self.x[i] - err, self.y[i]);
                    let (px_hi, _) = transform.transform_xy(self.x[i] + err, self.y[i]);

                    // Horizontal line
                    let mut pb = PathBuilder::new();
                    pb.move_to(px_lo, py);
                    pb.line_to(px_hi, py);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                    }

                    // Vertical caps
                    if self.capsize > 0.0 {
                        // Left cap
                        let mut pb = PathBuilder::new();
                        pb.move_to(px_lo, py - self.capsize);
                        pb.line_to(px_lo, py + self.capsize);
                        if let Some(path) = pb.finish() {
                            pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                        }
                        // Right cap
                        let mut pb = PathBuilder::new();
                        pb.move_to(px_hi, py - self.capsize);
                        pb.line_to(px_hi, py + self.capsize);
                        if let Some(path) = pb.finish() {
                            pixmap.stroke_path(&path, &paint, &err_stroke, ts, None);
                        }
                    }
                }
            }

            // Draw markers
            if self.marker != MarkerStyle::None {
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
            let mut xl = self.x[i];
            let mut xr = self.x[i];
            let mut yl = self.y[i];
            let mut yh = self.y[i];

            if let Some(ref xerr) = self.xerr {
                if i < xerr.len() {
                    xl -= xerr[i];
                    xr += xerr[i];
                }
            }
            if let Some(ref yerr) = self.yerr {
                if i < yerr.len() {
                    yl -= yerr[i];
                    yh += yerr[i];
                }
            }

            if xl < xmin { xmin = xl; }
            if xr > xmax { xmax = xr; }
            if yl < ymin { ymin = yl; }
            if yh > ymax { ymax = yh; }
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
