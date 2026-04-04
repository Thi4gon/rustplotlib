use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke};

use crate::artists::Artist;
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;
use crate::text;

/// Visual widget types that can be drawn on axes.
#[derive(Clone, Debug)]
pub enum WidgetKind {
    Slider {
        val: f64,
        valmin: f64,
        valmax: f64,
        label: String,
        orientation: String, // "horizontal" or "vertical"
    },
    Button {
        label: String,
    },
}

/// A widget rendered as an artist on axes.
pub struct WidgetArtist {
    pub kind: WidgetKind,
    pub color: Color,
    pub track_color: Color,
    pub alpha: f32,
    pub zorder: i32,
}

impl WidgetArtist {
    pub fn new_slider(val: f64, valmin: f64, valmax: f64, label: String, color: Color) -> Self {
        Self {
            kind: WidgetKind::Slider {
                val, valmin, valmax, label,
                orientation: "horizontal".to_string(),
            },
            color,
            track_color: Color::new(200, 200, 200, 255),
            alpha: 1.0,
            zorder: 10,
        }
    }

    pub fn new_button(label: String, color: Color) -> Self {
        Self {
            kind: WidgetKind::Button { label },
            color,
            track_color: Color::new(220, 220, 220, 255),
            alpha: 1.0,
            zorder: 10,
        }
    }
}

impl Artist for WidgetArtist {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();

        match &self.kind {
            WidgetKind::Slider { val, valmin, valmax, label, orientation } => {
                // Get axes bounds in pixel space
                let (px_left, py_top) = transform.transform_xy(0.0, 1.0);
                let (px_right, py_bottom) = transform.transform_xy(1.0, 0.0);

                let x0 = px_left.min(px_right);
                let y0 = py_top.min(py_bottom);
                let w = (px_right - px_left).abs();
                let h = (py_bottom - py_top).abs();

                if orientation == "horizontal" {
                    // Track background
                    let track_y = y0 + h * 0.4;
                    let track_h = h * 0.2;
                    if let Some(rect) = Rect::from_xywh(x0, track_y, w, track_h) {
                        let mut paint = Paint::default();
                        paint.set_color(self.track_color.to_tiny_skia());
                        pixmap.fill_rect(rect, &paint, ts, None);
                    }

                    // Filled portion
                    let frac = if valmax > valmin { (val - valmin) / (valmax - valmin) } else { 0.5 };
                    let fill_w = (w * frac as f32).max(0.0);
                    if let Some(rect) = Rect::from_xywh(x0, track_y, fill_w, track_h) {
                        let mut paint = Paint::default();
                        let mut c = self.color;
                        c.a = (self.alpha * 255.0) as u8;
                        paint.set_color(c.to_tiny_skia());
                        pixmap.fill_rect(rect, &paint, ts, None);
                    }

                    // Thumb circle
                    let thumb_x = x0 + fill_w;
                    let thumb_y = track_y + track_h / 2.0;
                    let thumb_r = track_h * 0.8;
                    let mut pb = PathBuilder::new();
                    // Approximate circle with path
                    let steps = 20;
                    for i in 0..=steps {
                        let angle = i as f32 / steps as f32 * std::f32::consts::PI * 2.0;
                        let cx = thumb_x + thumb_r * angle.cos();
                        let cy = thumb_y + thumb_r * angle.sin();
                        if i == 0 {
                            pb.move_to(cx, cy);
                        } else {
                            pb.line_to(cx, cy);
                        }
                    }
                    pb.close();
                    if let Some(path) = pb.finish() {
                        let mut paint = Paint::default();
                        paint.set_color(self.color.to_tiny_skia());
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                        // White border
                        let mut border_paint = Paint::default();
                        border_paint.set_color(tiny_skia::Color::WHITE);
                        let mut stroke = Stroke::default();
                        stroke.width = 1.5;
                        pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
                    }

                    // Label text
                    if !label.is_empty() {
                        let label_x = x0;
                        let label_y = y0 + h * 0.2;
                        text::draw_text(pixmap, label, label_x, label_y, 11.0,
                            Color::new(0, 0, 0, 255),
                            text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
                    }

                    // Value text
                    let val_str = format!("{:.2}", val);
                    let val_x = x0 + w - 40.0;
                    let val_y = y0 + h * 0.2;
                    text::draw_text(pixmap, &val_str, val_x, val_y, 11.0,
                        Color::new(0, 0, 0, 255),
                        text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
                }
            }
            WidgetKind::Button { label } => {
                let (px_left, py_top) = transform.transform_xy(0.0, 1.0);
                let (px_right, py_bottom) = transform.transform_xy(1.0, 0.0);

                let x0 = px_left.min(px_right);
                let y0 = py_top.min(py_bottom);
                let w = (px_right - px_left).abs();
                let h = (py_bottom - py_top).abs();

                // Button background
                let margin = 4.0;
                if let Some(rect) = Rect::from_xywh(x0 + margin, y0 + margin,
                    w - margin * 2.0, h - margin * 2.0)
                {
                    let mut paint = Paint::default();
                    paint.set_color(self.track_color.to_tiny_skia());
                    pixmap.fill_rect(rect, &paint, ts, None);

                    // Border
                    let mut border_paint = Paint::default();
                    border_paint.set_color(Color::new(150, 150, 150, 255).to_tiny_skia());
                    let mut stroke = Stroke::default();
                    stroke.width = 1.0;
                    let mut pb = PathBuilder::new();
                    pb.move_to(rect.left(), rect.top());
                    pb.line_to(rect.right(), rect.top());
                    pb.line_to(rect.right(), rect.bottom());
                    pb.line_to(rect.left(), rect.bottom());
                    pb.close();
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
                    }
                }

                // Button label centered
                if !label.is_empty() {
                    let text_x = x0 + w * 0.3;
                    let text_y = y0 + h * 0.55;
                    text::draw_text(pixmap, label, text_x, text_y, 12.0,
                        Color::new(0, 0, 0, 255),
                        text::TextAnchorX::Left, text::TextAnchorY::Top, 0.0);
                }
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
        self.color
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}
