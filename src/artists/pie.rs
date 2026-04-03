use crate::artists::Artist;
use crate::colors::Color;
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap};

/// Default color palette for pie chart wedges.
const PIE_COLORS: [(u8, u8, u8); 10] = [
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

pub struct PieChart {
    pub sizes: Vec<f64>,
    pub labels: Vec<String>,
    pub colors: Vec<Color>,
    pub start_angle: f32, // degrees
}

impl PieChart {
    pub fn new(sizes: Vec<f64>, labels: Vec<String>, colors: Vec<Color>, start_angle: f32) -> Self {
        Self {
            sizes,
            labels,
            colors,
            start_angle,
        }
    }

    /// Get default colors for wedges.
    pub fn default_colors(count: usize) -> Vec<Color> {
        (0..count)
            .map(|i| {
                let (r, g, b) = PIE_COLORS[i % PIE_COLORS.len()];
                Color::new(r, g, b, 255)
            })
            .collect()
    }
}

/// Approximate an arc using cubic Bezier curves.
/// Draws an arc from angle `start` to `end` (in radians) on a circle centered at (cx, cy) with radius r.
/// Appends segments to the PathBuilder (does NOT call move_to for the first point).
fn arc_bezier(pb: &mut PathBuilder, cx: f32, cy: f32, r: f32, start: f32, end: f32) {
    // Split arc into segments of at most PI/2 (90 degrees)
    let mut a = start;
    while a < end - 1e-6 {
        let b = (a + std::f32::consts::FRAC_PI_2).min(end);
        let angle = b - a;
        let half = angle / 2.0;

        // Bezier approximation of circular arc
        let alpha = (4.0 / 3.0) * (half / 2.0).tan();

        let cos_a = a.cos();
        let sin_a = a.sin();
        let cos_b = b.cos();
        let sin_b = b.sin();

        let p1x = cx + r * cos_a;
        let p1y = cy + r * sin_a;
        let p2x = p1x - alpha * r * sin_a;
        let p2y = p1y + alpha * r * cos_a;
        let p4x = cx + r * cos_b;
        let p4y = cy + r * sin_b;
        let p3x = p4x + alpha * r * sin_b;
        let p3y = p4y - alpha * r * cos_b;

        // If this is the very first segment we need line_to the start
        // (the caller already did move_to center)
        if (a - start).abs() < 1e-6 {
            pb.line_to(p1x, p1y);
        }

        pb.cubic_to(p2x, p2y, p3x, p3y, p4x, p4y);

        a = b;
    }
}

impl Artist for PieChart {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.sizes.is_empty() {
            return;
        }

        let total: f64 = self.sizes.iter().sum();
        if total <= 0.0 {
            return;
        }

        // Compute center and radius in pixel space
        // Use the center of the plot area
        let cx = ((transform.pixel_left + transform.pixel_right) / 2.0) as f32;
        let cy = ((transform.pixel_top + transform.pixel_bottom) / 2.0) as f32;
        let plot_w = (transform.pixel_right - transform.pixel_left) as f32;
        let plot_h = (transform.pixel_bottom - transform.pixel_top) as f32;
        let radius = (plot_w.min(plot_h) / 2.0) * 0.8; // 80% of half the smaller dimension

        let ts = tiny_skia::Transform::identity();

        // Start angle in radians (matplotlib default is 90 degrees = top)
        let start_rad = self.start_angle.to_radians();

        let mut current_angle = start_rad;

        for (i, &size) in self.sizes.iter().enumerate() {
            let sweep = (size / total) as f32 * std::f32::consts::TAU;
            let end_angle = current_angle + sweep;

            // Get color for this wedge
            let color = if i < self.colors.len() {
                self.colors[i]
            } else {
                let (r, g, b) = PIE_COLORS[i % PIE_COLORS.len()];
                Color::new(r, g, b, 255)
            };

            // Build wedge path: center -> arc -> center
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy);
            arc_bezier(&mut pb, cx, cy, radius, current_angle, end_angle);
            pb.line_to(cx, cy);
            pb.close();

            if let Some(path) = pb.finish() {
                // Fill
                let mut fill_paint = Paint::default();
                fill_paint.set_color(color.to_tiny_skia());
                fill_paint.anti_alias = true;
                pixmap.fill_path(&path, &fill_paint, tiny_skia::FillRule::Winding, ts, None);

                // Border
                let mut border_paint = Paint::default();
                border_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
                border_paint.anti_alias = true;
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = 1.5;
                pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
            }

            // Draw label at midpoint of the arc, outside
            if i < self.labels.len() && !self.labels[i].is_empty() {
                let mid_angle = current_angle + sweep / 2.0;
                let label_r = radius * 1.15;
                let lx = cx + label_r * mid_angle.cos();
                let ly = cy + label_r * mid_angle.sin();

                let anchor_x = if mid_angle.cos() >= 0.0 {
                    TextAnchorX::Left
                } else {
                    TextAnchorX::Right
                };

                draw_text(
                    pixmap,
                    &self.labels[i],
                    lx,
                    ly,
                    10.0,
                    Color::new(0, 0, 0, 255),
                    anchor_x,
                    TextAnchorY::Center,
                    0.0,
                );
            }

            current_angle = end_angle;
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        // Pie chart doesn't use data coordinates, but we need valid bounds
        // so the transform gives us the full plot area.
        (-1.0, 1.0, -1.0, 1.0)
    }

    fn legend_label(&self) -> Option<&str> {
        None // Pie charts use labels per-wedge, not a single legend label
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }
}
