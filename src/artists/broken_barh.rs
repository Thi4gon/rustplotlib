use tiny_skia::{Paint, Pixmap, PathBuilder, FillRule};

use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;

/// Broken horizontal bar chart artist.
/// Each row has a y-range (y_start, height) and multiple x-segments (x_start, width).
pub struct BrokenBarH {
    pub y_ranges: Vec<(f64, f64)>,        // (y_start, height) per row
    pub x_ranges: Vec<Vec<(f64, f64)>>,   // (x_start, width) segments per row
    pub colors: Vec<Color>,
    pub alpha: f32,
    pub label: Option<String>,
}

impl BrokenBarH {
    pub fn new(
        y_ranges: Vec<(f64, f64)>,
        x_ranges: Vec<Vec<(f64, f64)>>,
        colors: Vec<Color>,
        alpha: f32,
    ) -> Self {
        Self {
            y_ranges,
            x_ranges,
            colors,
            alpha,
            label: None,
        }
    }
}

impl Artist for BrokenBarH {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();

        for (row_idx, (y_start, height)) in self.y_ranges.iter().enumerate() {
            let color = if row_idx < self.colors.len() {
                self.colors[row_idx]
            } else if !self.colors.is_empty() {
                self.colors[row_idx % self.colors.len()]
            } else {
                Color::new(31, 119, 180, 255)
            };

            let segments = if row_idx < self.x_ranges.len() {
                &self.x_ranges[row_idx]
            } else {
                continue;
            };

            for &(x_start, width) in segments {
                let (px1, py1) = transform.transform_xy(x_start, *y_start + *height);
                let (px2, py2) = transform.transform_xy(x_start + width, *y_start);

                let left = px1.min(px2);
                let top = py1.min(py2);
                let w = (px2 - px1).abs();
                let h = (py2 - py1).abs();

                if w < 0.1 || h < 0.1 {
                    continue;
                }

                if let Some(rect) = tiny_skia::Rect::from_xywh(left, top, w, h) {
                    let path = PathBuilder::from_rect(rect);
                    let mut paint = Paint::default();
                    let mut c = color;
                    c.a = (self.alpha * 255.0) as u8;
                    paint.set_color(c.to_tiny_skia());
                    paint.anti_alias = true;
                    pixmap.fill_path(&path, &paint, FillRule::Winding, ts, None);

                    // Thin border
                    let mut edge_paint = Paint::default();
                    edge_paint.set_color_rgba8(
                        (color.r as u16 * 3 / 4) as u8,
                        (color.g as u16 * 3 / 4) as u8,
                        (color.b as u16 * 3 / 4) as u8,
                        (self.alpha * 220.0) as u8,
                    );
                    edge_paint.anti_alias = true;
                    let mut stroke = tiny_skia::Stroke::default();
                    stroke.width = 0.5;
                    pixmap.stroke_path(&path, &edge_paint, &stroke, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for (row_idx, (y_start, height)) in self.y_ranges.iter().enumerate() {
            if *y_start < ymin {
                ymin = *y_start;
            }
            if *y_start + *height > ymax {
                ymax = *y_start + *height;
            }
            if row_idx < self.x_ranges.len() {
                for &(x_start, width) in &self.x_ranges[row_idx] {
                    if x_start < xmin {
                        xmin = x_start;
                    }
                    if x_start + width > xmax {
                        xmax = x_start + width;
                    }
                }
            }
        }

        if xmin > xmax {
            (0.0, 1.0, 0.0, 1.0)
        } else {
            (xmin, xmax, ymin, ymax)
        }
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        if !self.colors.is_empty() {
            self.colors[0]
        } else {
            Color::new(31, 119, 180, 255)
        }
    }
}
