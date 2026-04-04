use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, FillRule};

use crate::artists::Artist;
use crate::colors::Color;
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use crate::transforms::Transform;

/// Radar / spider chart artist.
pub struct Radar {
    pub categories: Vec<String>,
    pub values: Vec<Vec<f64>>,  // multiple series
    pub colors: Vec<Color>,
    pub labels: Vec<String>,
    pub alpha: f32,
    pub fill: bool,
}

impl Radar {
    pub fn new(
        categories: Vec<String>,
        values: Vec<Vec<f64>>,
        colors: Vec<Color>,
        labels: Vec<String>,
        alpha: f32,
        fill: bool,
    ) -> Self {
        Self {
            categories,
            values,
            colors,
            labels,
            alpha,
            fill,
        }
    }
}

/// Tab10 color palette for default colors.
const TAB10: [(u8, u8, u8); 10] = [
    (31, 119, 180),
    (255, 127, 14),
    (44, 160, 44),
    (214, 39, 40),
    (148, 103, 189),
    (140, 86, 75),
    (227, 119, 194),
    (127, 127, 127),
    (188, 189, 34),
    (23, 190, 207),
];

impl Artist for Radar {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let n = self.categories.len();
        if n < 3 {
            return;
        }

        // Compute the center and radius from the transform's plot area
        let (cx_px, cy_px) = transform.transform_xy(0.5, 0.5);
        let (edge_x, _) = transform.transform_xy(1.0, 0.5);
        let (_, edge_y) = transform.transform_xy(0.5, 1.0);
        let radius = ((edge_x - cx_px).abs().min((cy_px - edge_y).abs())) * 0.40;

        let ts = tiny_skia::Transform::identity();
        let angle_step = 2.0 * std::f64::consts::PI / n as f64;

        // Find max value for normalization
        let max_val = self.values.iter()
            .flat_map(|v| v.iter())
            .cloned()
            .fold(0.0f64, f64::max)
            .max(1.0);

        // Draw concentric grid circles (5 levels)
        for level in 1..=5 {
            let r = radius * (level as f32 / 5.0);
            let mut pb = PathBuilder::new();
            for i in 0..=n {
                let angle = -std::f64::consts::FRAC_PI_2 + (i % n) as f64 * angle_step;
                let px = cx_px + r * angle.cos() as f32;
                let py = cy_px + r * angle.sin() as f32;
                if i == 0 {
                    pb.move_to(px, py);
                } else {
                    pb.line_to(px, py);
                }
            }
            pb.close();
            if let Some(path) = pb.finish() {
                let mut paint = Paint::default();
                paint.set_color_rgba8(200, 200, 200, 150);
                paint.anti_alias = true;
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }

        // Draw radial lines and category labels
        for i in 0..n {
            let angle = -std::f64::consts::FRAC_PI_2 + i as f64 * angle_step;
            let end_x = cx_px + radius * angle.cos() as f32;
            let end_y = cy_px + radius * angle.sin() as f32;

            // Radial line
            let mut pb = PathBuilder::new();
            pb.move_to(cx_px, cy_px);
            pb.line_to(end_x, end_y);
            if let Some(path) = pb.finish() {
                let mut paint = Paint::default();
                paint.set_color_rgba8(180, 180, 180, 200);
                paint.anti_alias = true;
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Category label
            let label_r = radius + 15.0;
            let lx = cx_px + label_r * angle.cos() as f32;
            let ly = cy_px + label_r * angle.sin() as f32;
            if i < self.categories.len() {
                let anchor_x = if (angle.cos()).abs() < 0.1 {
                    TextAnchorX::Center
                } else if angle.cos() > 0.0 {
                    TextAnchorX::Left
                } else {
                    TextAnchorX::Right
                };
                draw_text(
                    pixmap,
                    &self.categories[i],
                    lx,
                    ly,
                    10.0,
                    Color::new(0, 0, 0, 255),
                    anchor_x,
                    TextAnchorY::Center,
                    0.0,
                );
            }
        }

        // Draw each series
        for (si, series) in self.values.iter().enumerate() {
            let color = if si < self.colors.len() {
                self.colors[si]
            } else {
                let (r, g, b) = TAB10[si % TAB10.len()];
                Color::new(r, g, b, 255)
            };

            let mut pb = PathBuilder::new();
            for i in 0..=n {
                let idx = i % n;
                let val = if idx < series.len() { series[idx] } else { 0.0 };
                let r = radius * (val / max_val) as f32;
                let angle = -std::f64::consts::FRAC_PI_2 + idx as f64 * angle_step;
                let px = cx_px + r * angle.cos() as f32;
                let py = cy_px + r * angle.sin() as f32;
                if i == 0 {
                    pb.move_to(px, py);
                } else {
                    pb.line_to(px, py);
                }
            }
            pb.close();

            if let Some(path) = pb.finish() {
                // Fill if requested
                if self.fill {
                    let mut fill_paint = Paint::default();
                    let mut fc = color;
                    fc.a = (self.alpha * 255.0 * 0.3) as u8;
                    fill_paint.set_color(fc.to_tiny_skia());
                    fill_paint.anti_alias = true;
                    pixmap.fill_path(&path, &fill_paint, FillRule::Winding, ts, None);
                }

                // Outline
                let mut line_paint = Paint::default();
                let mut lc = color;
                lc.a = (self.alpha * 255.0) as u8;
                line_paint.set_color(lc.to_tiny_skia());
                line_paint.anti_alias = true;
                let mut stroke = Stroke::default();
                stroke.width = 2.0;
                pixmap.stroke_path(&path, &line_paint, &stroke, ts, None);
            }

            // Draw markers at vertices
            for i in 0..n {
                let val = if i < series.len() { series[i] } else { 0.0 };
                let r = radius * (val / max_val) as f32;
                let angle = -std::f64::consts::FRAC_PI_2 + i as f64 * angle_step;
                let px = cx_px + r * angle.cos() as f32;
                let py = cy_px + r * angle.sin() as f32;

                if let Some(circle) = crate::artists::circle_path(px, py, 3.0) {
                    let mut paint = Paint::default();
                    let mut mc = color;
                    mc.a = (self.alpha * 255.0) as u8;
                    paint.set_color(mc.to_tiny_skia());
                    paint.anti_alias = true;
                    pixmap.fill_path(&circle, &paint, FillRule::Winding, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        // Radar charts use their own coordinate system; provide a unit square.
        (0.0, 1.0, 0.0, 1.0)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        if !self.colors.is_empty() {
            self.colors[0]
        } else {
            Color::new(31, 119, 180, 255)
        }
    }
}
