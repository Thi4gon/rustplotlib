use crate::artists::Artist;
use crate::artists::circle_path;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

pub struct BoxPlot {
    pub data: Vec<Vec<f64>>,
    pub positions: Vec<f64>,
    pub widths: f64,
    pub color: Color,
    pub median_color: Color,
}

/// Statistics for one box in the box plot.
struct BoxStats {
    min_val: f64,
    q1: f64,
    median: f64,
    q3: f64,
    max_val: f64,
    whisker_lo: f64,
    whisker_hi: f64,
    outliers: Vec<f64>,
}

impl BoxPlot {
    pub fn new(data: Vec<Vec<f64>>, positions: Vec<f64>, widths: f64, color: Color, median_color: Color) -> Self {
        Self {
            data,
            positions,
            widths,
            color,
            median_color,
        }
    }

    fn compute_stats(data: &[f64]) -> Option<BoxStats> {
        if data.is_empty() {
            return None;
        }
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = sorted.len();
        let min_val = sorted[0];
        let max_val = sorted[n - 1];

        let median = percentile(&sorted, 50.0);
        let q1 = percentile(&sorted, 25.0);
        let q3 = percentile(&sorted, 75.0);
        let iqr = q3 - q1;

        let whisker_lo_limit = q1 - 1.5 * iqr;
        let whisker_hi_limit = q3 + 1.5 * iqr;

        // Whiskers are clamped to actual data range
        let whisker_lo = sorted.iter().copied()
            .find(|&v| v >= whisker_lo_limit)
            .unwrap_or(min_val);
        let whisker_hi = sorted.iter().rev().copied()
            .find(|&v| v <= whisker_hi_limit)
            .unwrap_or(max_val);

        let outliers: Vec<f64> = sorted.iter().copied()
            .filter(|&v| v < whisker_lo || v > whisker_hi)
            .collect();

        Some(BoxStats {
            min_val,
            q1,
            median,
            q3,
            max_val,
            whisker_lo,
            whisker_hi,
            outliers,
        })
    }
}

/// Linear interpolation percentile (same as numpy's "linear" method).
fn percentile(sorted: &[f64], pct: f64) -> f64 {
    let n = sorted.len();
    if n == 1 {
        return sorted[0];
    }
    let rank = pct / 100.0 * (n - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - lo as f64;
    if lo == hi || hi >= n {
        sorted[lo]
    } else {
        sorted[lo] * (1.0 - frac) + sorted[hi] * frac
    }
}

impl Artist for BoxPlot {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.data.is_empty() {
            return;
        }

        let ts = tiny_skia::Transform::identity();
        let half_w = self.widths / 2.0;

        for (idx, dataset) in self.data.iter().enumerate() {
            let pos = if idx < self.positions.len() {
                self.positions[idx]
            } else {
                (idx + 1) as f64
            };

            let stats = match BoxPlot::compute_stats(dataset) {
                Some(s) => s,
                None => continue,
            };

            let mut box_color = self.color;
            box_color.a = 255;
            let mut box_paint = Paint::default();
            box_paint.set_color(box_color.to_tiny_skia());
            box_paint.anti_alias = true;

            let mut stroke = Stroke::default();
            stroke.width = 1.5;

            // Draw the box (Q1 to Q3)
            let (px_left, py_q3) = transform.transform_xy(pos - half_w, stats.q3);
            let (px_right, py_q1) = transform.transform_xy(pos + half_w, stats.q1);

            let rect_x = px_left.min(px_right);
            let rect_y = py_q3.min(py_q1);
            let rect_w = (px_right - px_left).abs().max(1.0);
            let rect_h = (py_q1 - py_q3).abs().max(1.0);

            if let Some(rect) = Rect::from_xywh(rect_x, rect_y, rect_w, rect_h) {
                // White fill
                let mut fill_paint = Paint::default();
                fill_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
                pixmap.fill_rect(rect, &fill_paint, ts, None);

                // Border
                let path = PathBuilder::from_rect(rect);
                pixmap.stroke_path(&path, &box_paint, &stroke, ts, None);
            }

            // Draw median line
            let mut median_paint = Paint::default();
            let mut mc = self.median_color;
            mc.a = 255;
            median_paint.set_color(mc.to_tiny_skia());
            median_paint.anti_alias = true;

            let (px_ml, py_med) = transform.transform_xy(pos - half_w, stats.median);
            let (px_mr, _) = transform.transform_xy(pos + half_w, stats.median);
            let mut pb = PathBuilder::new();
            pb.move_to(px_ml, py_med);
            pb.line_to(px_mr, py_med);
            if let Some(path) = pb.finish() {
                let mut med_stroke = Stroke::default();
                med_stroke.width = 2.0;
                pixmap.stroke_path(&path, &median_paint, &med_stroke, ts, None);
            }

            // Draw whiskers
            let cap_half = half_w * 0.5;

            // Lower whisker
            let (px_center, py_wl) = transform.transform_xy(pos, stats.whisker_lo);
            let (_, py_q1_w) = transform.transform_xy(pos, stats.q1);
            let mut pb = PathBuilder::new();
            pb.move_to(px_center, py_q1_w);
            pb.line_to(px_center, py_wl);
            if let Some(path) = pb.finish() {
                let mut whisker_stroke = Stroke::default();
                whisker_stroke.width = 1.0;
                whisker_stroke.dash = tiny_skia::StrokeDash::new(vec![4.0, 2.0], 0.0);
                pixmap.stroke_path(&path, &box_paint, &whisker_stroke, ts, None);
            }
            // Lower cap
            let (px_cl, _) = transform.transform_xy(pos - cap_half, stats.whisker_lo);
            let (px_cr, _) = transform.transform_xy(pos + cap_half, stats.whisker_lo);
            let mut pb = PathBuilder::new();
            pb.move_to(px_cl, py_wl);
            pb.line_to(px_cr, py_wl);
            if let Some(path) = pb.finish() {
                let mut cap_stroke = Stroke::default();
                cap_stroke.width = 1.5;
                pixmap.stroke_path(&path, &box_paint, &cap_stroke, ts, None);
            }

            // Upper whisker
            let (_, py_wh) = transform.transform_xy(pos, stats.whisker_hi);
            let (_, py_q3_w) = transform.transform_xy(pos, stats.q3);
            let mut pb = PathBuilder::new();
            pb.move_to(px_center, py_q3_w);
            pb.line_to(px_center, py_wh);
            if let Some(path) = pb.finish() {
                let mut whisker_stroke = Stroke::default();
                whisker_stroke.width = 1.0;
                whisker_stroke.dash = tiny_skia::StrokeDash::new(vec![4.0, 2.0], 0.0);
                pixmap.stroke_path(&path, &box_paint, &whisker_stroke, ts, None);
            }
            // Upper cap
            let (px_cl, _) = transform.transform_xy(pos - cap_half, stats.whisker_hi);
            let (px_cr, _) = transform.transform_xy(pos + cap_half, stats.whisker_hi);
            let mut pb = PathBuilder::new();
            pb.move_to(px_cl, py_wh);
            pb.line_to(px_cr, py_wh);
            if let Some(path) = pb.finish() {
                let mut cap_stroke = Stroke::default();
                cap_stroke.width = 1.5;
                pixmap.stroke_path(&path, &box_paint, &cap_stroke, ts, None);
            }

            // Draw outliers
            for &outlier in &stats.outliers {
                let (px, py) = transform.transform_xy(pos, outlier);
                if let Some(path) = circle_path(px, py, 2.5) {
                    let mut outlier_paint = Paint::default();
                    outlier_paint.set_color(box_color.to_tiny_skia());
                    outlier_paint.anti_alias = true;
                    let mut outlier_stroke = Stroke::default();
                    outlier_stroke.width = 1.0;
                    pixmap.stroke_path(&path, &outlier_paint, &outlier_stroke, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.data.is_empty() || self.positions.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let half_w = self.widths / 2.0;
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for (idx, dataset) in self.data.iter().enumerate() {
            let pos = if idx < self.positions.len() {
                self.positions[idx]
            } else {
                (idx + 1) as f64
            };
            if pos - half_w < xmin { xmin = pos - half_w; }
            if pos + half_w > xmax { xmax = pos + half_w; }

            for &val in dataset {
                if val < ymin { ymin = val; }
                if val > ymax { ymax = val; }
            }
        }

        if xmin >= xmax { xmin = 0.0; xmax = 1.0; }
        if ymin >= ymax { ymin = 0.0; ymax = 1.0; }

        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
