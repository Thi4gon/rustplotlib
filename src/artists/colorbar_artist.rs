use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke};

use crate::artists::Artist;
use crate::artists::image::colormap_lookup;
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::text::{self, TextAnchorX, TextAnchorY};
use crate::ticker;
use crate::transforms::Transform;

/// A standalone colorbar artist that renders a color gradient with ticks and labels.
/// Designed to be placed in its own Axes (separate from the plot axes).
pub struct ColorbarArtist {
    pub cmap: String,
    pub vmin: f64,
    pub vmax: f64,
    pub orientation: String,  // "vertical" or "horizontal"
    pub label: Option<String>,
    pub n_ticks: usize,
    pub zorder: i32,
}

impl ColorbarArtist {
    pub fn new(cmap: String, vmin: f64, vmax: f64, orientation: String) -> Self {
        Self {
            cmap,
            vmin,
            vmax,
            orientation,
            label: None,
            n_ticks: 5,
            zorder: 1,
        }
    }
}

impl Artist for ColorbarArtist {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();

        // Get pixel bounds from the transform (axes area)
        let (px_left, py_top) = transform.transform_xy(0.0, 1.0);
        let (px_right, py_bottom) = transform.transform_xy(1.0, 0.0);

        let left = px_left.min(px_right);
        let top = py_top.min(py_bottom);
        let width = (px_right - px_left).abs();
        let height = (py_bottom - py_top).abs();

        let horizontal = self.orientation == "horizontal";

        if horizontal {
            // Horizontal: gradient left→right, ticks below
            let bar_height = height * 0.4;
            let bar_top = top + height * 0.1;
            let bar_left = left + width * 0.05;
            let bar_width = width * 0.9;

            // Draw gradient
            let n_steps = (bar_width as usize).max(2);
            for i in 0..n_steps {
                let frac = i as f32 / (n_steps - 1) as f32;
                let t = frac as f64;
                let color = colormap_lookup(&self.cmap, t);

                let x = bar_left + frac * bar_width;
                let col_w = (bar_width / n_steps as f32).max(1.0);

                if let Some(rect) = Rect::from_xywh(x, bar_top, col_w, bar_height) {
                    let mut paint = Paint::default();
                    paint.set_color(color.to_tiny_skia());
                    paint.anti_alias = false;
                    pixmap.fill_rect(rect, &paint, ts, None);
                }
            }

            // Border
            if let Some(rect) = Rect::from_xywh(bar_left, bar_top, bar_width, bar_height) {
                let path = PathBuilder::from_rect(rect);
                let mut paint = Paint::default();
                paint.set_color(Color::new(0, 0, 0, 255).to_tiny_skia());
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Tick labels
            let tick_y = bar_top + bar_height + 3.0;
            let label_y = tick_y + 12.0;
            for i in 0..self.n_ticks {
                let frac = i as f32 / (self.n_ticks - 1) as f32;
                let x = bar_left + frac * bar_width;
                let val = self.vmin + frac as f64 * (self.vmax - self.vmin);
                let tick_label = ticker::format_tick_value(val);

                // Tick mark
                let mut pb = PathBuilder::new();
                pb.move_to(x, bar_top + bar_height);
                pb.line_to(x, tick_y);
                if let Some(path) = pb.finish() {
                    let mut tp = Paint::default();
                    tp.set_color(Color::new(0, 0, 0, 255).to_tiny_skia());
                    let mut s = Stroke::default();
                    s.width = 0.5;
                    pixmap.stroke_path(&path, &tp, &s, ts, None);
                }

                text::draw_text(pixmap, &tick_label, x, label_y, 8.0,
                    Color::new(0, 0, 0, 255), TextAnchorX::Center, TextAnchorY::Top, 0.0);
            }

            // Label
            if let Some(ref lbl) = self.label {
                let cx = bar_left + bar_width / 2.0;
                text::draw_text(pixmap, lbl, cx, label_y + 14.0, 10.0,
                    Color::new(0, 0, 0, 255), TextAnchorX::Center, TextAnchorY::Top, 0.0);
            }
        } else {
            // Vertical: gradient top(vmax)→bottom(vmin), ticks right
            let bar_width = width * 0.3;
            let bar_left = left + width * 0.1;
            let bar_top_px = top + height * 0.05;
            let bar_height_px = height * 0.9;

            // Draw gradient
            let n_steps = (bar_height_px as usize).max(2);
            for i in 0..n_steps {
                let frac = i as f32 / (n_steps - 1) as f32;
                let t = 1.0 - frac as f64; // top=vmax, bottom=vmin
                let color = colormap_lookup(&self.cmap, t);

                let y = bar_top_px + frac * bar_height_px;
                let row_h = (bar_height_px / n_steps as f32).max(1.0);

                if let Some(rect) = Rect::from_xywh(bar_left, y, bar_width, row_h) {
                    let mut paint = Paint::default();
                    paint.set_color(color.to_tiny_skia());
                    paint.anti_alias = false;
                    pixmap.fill_rect(rect, &paint, ts, None);
                }
            }

            // Border
            if let Some(rect) = Rect::from_xywh(bar_left, bar_top_px, bar_width, bar_height_px) {
                let path = PathBuilder::from_rect(rect);
                let mut paint = Paint::default();
                paint.set_color(Color::new(0, 0, 0, 255).to_tiny_skia());
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Tick labels
            let tick_x = bar_left + bar_width + 3.0;
            for i in 0..self.n_ticks {
                let frac = i as f32 / (self.n_ticks - 1) as f32;
                let y = bar_top_px + frac * bar_height_px;
                let val = self.vmax - frac as f64 * (self.vmax - self.vmin); // top=max
                let tick_label = ticker::format_tick_value(val);

                // Tick mark
                let mut pb = PathBuilder::new();
                pb.move_to(bar_left + bar_width, y);
                pb.line_to(tick_x, y);
                if let Some(path) = pb.finish() {
                    let mut tp = Paint::default();
                    tp.set_color(Color::new(0, 0, 0, 255).to_tiny_skia());
                    let mut s = Stroke::default();
                    s.width = 0.5;
                    pixmap.stroke_path(&path, &tp, &s, ts, None);
                }

                text::draw_text(pixmap, &tick_label, tick_x + 2.0, y, 8.0,
                    Color::new(0, 0, 0, 255), TextAnchorX::Left, TextAnchorY::Center, 0.0);
            }

            // Label (rotated would be ideal, but for now draw horizontally above)
            if let Some(ref lbl) = self.label {
                let cx = bar_left + bar_width / 2.0;
                text::draw_text(pixmap, lbl, cx, bar_top_px - 12.0, 10.0,
                    Color::new(0, 0, 0, 255), TextAnchorX::Center, TextAnchorY::Bottom, 0.0);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        (0.0, 1.0, 0.0, 1.0)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}
