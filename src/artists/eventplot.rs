use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;

/// Event/raster plot artist.
/// Draws short vertical (or horizontal) lines at event positions.
pub struct EventPlot {
    pub positions: Vec<Vec<f64>>,  // events for each row
    pub orientation: String,       // "horizontal" or "vertical"
    pub linewidths: f32,
    pub colors: Vec<Color>,
    pub linelength: f64,
}

/// Tab10 palette for default colors.
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

impl EventPlot {
    pub fn new(
        positions: Vec<Vec<f64>>,
        orientation: String,
        linewidths: f32,
        colors: Vec<Color>,
    ) -> Self {
        Self {
            positions,
            orientation,
            linewidths,
            colors,
            linelength: 0.8,
        }
    }
}

impl Artist for EventPlot {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();
        let horizontal = self.orientation != "vertical";

        for (row_idx, events) in self.positions.iter().enumerate() {
            let color = if row_idx < self.colors.len() {
                self.colors[row_idx]
            } else {
                let (r, g, b) = TAB10[row_idx % TAB10.len()];
                Color::new(r, g, b, 255)
            };

            let row_center = (row_idx as f64) + 1.0; // rows at 1, 2, 3, ...
            let half_len = self.linelength / 2.0;

            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            let mut stroke = Stroke::default();
            stroke.width = self.linewidths;

            for &pos in events {
                let mut pb = PathBuilder::new();
                if horizontal {
                    // Horizontal: events along x, rows along y
                    let (px, py_top) = transform.transform_xy(pos, row_center + half_len);
                    let (_, py_bot) = transform.transform_xy(pos, row_center - half_len);
                    pb.move_to(px, py_top);
                    pb.line_to(px, py_bot);
                } else {
                    // Vertical: events along y, rows along x
                    let (px_left, py) = transform.transform_xy(row_center - half_len, pos);
                    let (px_right, _) = transform.transform_xy(row_center + half_len, pos);
                    pb.move_to(px_left, py);
                    pb.line_to(px_right, py);
                }

                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let horizontal = self.orientation != "vertical";
        let n_rows = self.positions.len();

        let mut ev_min = f64::MAX;
        let mut ev_max = f64::MIN;

        for events in &self.positions {
            for &pos in events {
                if pos < ev_min {
                    ev_min = pos;
                }
                if pos > ev_max {
                    ev_max = pos;
                }
            }
        }

        if ev_min > ev_max {
            return (0.0, 1.0, 0.0, 1.0);
        }

        let row_min = 0.5;
        let row_max = n_rows as f64 + 0.5;

        if horizontal {
            (ev_min, ev_max, row_min, row_max)
        } else {
            (row_min, row_max, ev_min, ev_max)
        }
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
