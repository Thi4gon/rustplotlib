use crate::artists::{LineStyle, MarkerStyle, draw_marker};
use crate::colors::Color;
use crate::text::{draw_text, measure_text, TextAnchorX, TextAnchorY};
use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

pub struct LegendEntry {
    pub label: String,
    pub color: Color,
    pub line_style: Option<LineStyle>,
    pub marker: Option<MarkerStyle>,
    pub linewidth: f32,
}

/// Draw a legend on the pixmap at position (x, y) (top-left corner of legend box).
pub fn draw_legend(pixmap: &mut Pixmap, entries: &[LegendEntry], x: f32, y: f32) {
    if entries.is_empty() {
        return;
    }

    let font_size = 11.0_f32;
    let swatch_size = 20.0_f32;
    let padding = 6.0_f32;
    let line_height = font_size + 4.0;
    let swatch_text_gap = 5.0_f32;

    // Measure all entries to find the widest
    let mut max_label_width = 0.0_f32;
    for entry in entries {
        let (w, _) = measure_text(&entry.label, font_size);
        if w > max_label_width {
            max_label_width = w;
        }
    }

    let box_width = padding * 2.0 + swatch_size + swatch_text_gap + max_label_width;
    let box_height = padding * 2.0 + entries.len() as f32 * line_height;

    let ts = tiny_skia::Transform::identity();

    // Draw background box
    if let Some(rect) = Rect::from_xywh(x, y, box_width, box_height) {
        let mut bg_paint = Paint::default();
        bg_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 230));
        bg_paint.anti_alias = true;
        pixmap.fill_rect(rect, &bg_paint, ts, None);

        // Draw border
        let border_path = PathBuilder::from_rect(rect);
        let mut border_paint = Paint::default();
        border_paint.set_color(tiny_skia::Color::from_rgba8(180, 180, 180, 255));
        border_paint.anti_alias = true;
        let mut stroke = Stroke::default();
        stroke.width = 0.8;
        pixmap.stroke_path(&border_path, &border_paint, &stroke, ts, None);
    }

    // Draw each entry
    for (i, entry) in entries.iter().enumerate() {
        let entry_y = y + padding + i as f32 * line_height;

        let swatch_x = x + padding;
        let swatch_cy = entry_y + line_height / 2.0;

        let has_line = entry.line_style.is_some()
            && entry.line_style != Some(LineStyle::None);
        let has_marker = entry.marker.is_some()
            && entry.marker != Some(MarkerStyle::None);

        if has_line || has_marker {
            // Draw a short horizontal line segment
            if has_line {
                let line_style = entry.line_style.unwrap();
                let mut paint = Paint::default();
                paint.set_color(entry.color.to_tiny_skia());
                paint.anti_alias = true;

                let mut pb = PathBuilder::new();
                pb.move_to(swatch_x, swatch_cy);
                pb.line_to(swatch_x + swatch_size, swatch_cy);
                if let Some(path) = pb.finish() {
                    let mut stroke = Stroke::default();
                    stroke.width = entry.linewidth;
                    stroke.dash = line_style.to_dash(entry.linewidth);
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }
            }

            // Draw a marker in the middle of the swatch
            if has_marker {
                let marker = entry.marker.unwrap();
                let mx = swatch_x + swatch_size / 2.0;
                draw_marker(pixmap, marker, mx, swatch_cy, 6.0, entry.color, 1.0);
            }
        } else {
            // Fallback: draw a filled square (for bar/hist)
            let sq_size = 10.0_f32;
            let sq_y = swatch_cy - sq_size / 2.0;
            let sq_x = swatch_x + (swatch_size - sq_size) / 2.0;
            if let Some(rect) = Rect::from_xywh(sq_x, sq_y, sq_size, sq_size) {
                let mut swatch_paint = Paint::default();
                swatch_paint.set_color(entry.color.to_tiny_skia());
                swatch_paint.anti_alias = true;
                pixmap.fill_rect(rect, &swatch_paint, ts, None);
            }
        }

        // Label text
        let text_x = swatch_x + swatch_size + swatch_text_gap;
        let text_y = swatch_cy;
        draw_text(
            pixmap,
            &entry.label,
            text_x,
            text_y,
            font_size,
            Color::new(0, 0, 0, 255),
            TextAnchorX::Left,
            TextAnchorY::Center,
            0.0,
        );
    }
}
