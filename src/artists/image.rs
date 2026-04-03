use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, Rect, Pixmap};

/// Look up a color in a named colormap at parameter t in [0, 1].
pub fn colormap_lookup(name: &str, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0) as f32;
    match name {
        "gray" | "grey" => {
            let v = (t * 255.0) as u8;
            Color::new(v, v, v, 255)
        }
        "hot" => {
            // Black -> Red -> Yellow -> White
            let r = (t * 3.0).min(1.0);
            let g = ((t - 1.0 / 3.0) * 3.0).clamp(0.0, 1.0);
            let b = ((t - 2.0 / 3.0) * 3.0).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "cool" => {
            // Cyan to Magenta
            Color::from_f32(t, 1.0 - t, 1.0, 1.0)
        }
        "jet" => {
            // Simplified jet colormap
            let r = ((1.5 - (t - 0.75).abs() * 4.0) as f32).clamp(0.0, 1.0);
            let g = ((1.5 - (t - 0.5).abs() * 4.0) as f32).clamp(0.0, 1.0);
            let b = ((1.5 - (t - 0.25).abs() * 4.0) as f32).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "Blues" => {
            Color::from_f32(1.0 - t * 0.8, 1.0 - t * 0.6, 1.0, 1.0)
        }
        "Reds" => {
            Color::from_f32(1.0, 1.0 - t * 0.8, 1.0 - t * 0.9, 1.0)
        }
        "Greens" => {
            Color::from_f32(1.0 - t * 0.8, 1.0 - t * 0.2, 1.0 - t * 0.8, 1.0)
        }
        // "viridis" and default
        _ => {
            // Simplified viridis approximation
            let r = (0.267004 + t * (0.993248 - 0.267004)) as f32;
            let g_val = if t < 0.5 {
                (0.004874 + t * 2.0 * (0.554906 - 0.004874)) as f32
            } else {
                (0.554906 + (t - 0.5) * 2.0 * (0.906157 - 0.554906)) as f32
            };
            let b_val = if t < 0.5 {
                (0.329415 + t * 2.0 * (0.554906 - 0.329415)) as f32
            } else {
                (0.554906 - (t - 0.5) * 2.0 * (0.554906 - 0.143936)) as f32
            };
            Color::from_f32(
                r.clamp(0.0, 1.0),
                g_val.clamp(0.0, 1.0),
                b_val.clamp(0.0, 1.0),
                1.0,
            )
        }
    }
}

pub struct Image {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
    pub cmap: String,
    pub vmin: f64,
    pub vmax: f64,
}

impl Image {
    pub fn new(data: Vec<Vec<f64>>, cmap: String) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        // Compute vmin/vmax from data
        let mut vmin = f64::MAX;
        let mut vmax = f64::MIN;
        for row in &data {
            for &v in row {
                if v < vmin { vmin = v; }
                if v > vmax { vmax = v; }
            }
        }
        if (vmax - vmin).abs() < 1e-15 {
            vmax = vmin + 1.0;
        }

        Image { data, rows, cols, cmap, vmin, vmax }
    }
}

impl Artist for Image {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }

        let ts = tiny_skia::Transform::identity();

        for r in 0..self.rows {
            for c in 0..self.cols {
                let val = self.data[r][c];
                let t = (val - self.vmin) / (self.vmax - self.vmin);
                let color = colormap_lookup(&self.cmap, t);

                let mut paint = Paint::default();
                paint.set_color(color.to_tiny_skia());
                paint.anti_alias = false;

                // Each cell spans from (c - 0.5) to (c + 0.5) in x, (r - 0.5) to (r + 0.5) in y
                let x0 = c as f64 - 0.5;
                let x1 = c as f64 + 0.5;
                let y0 = r as f64 - 0.5;
                let y1 = r as f64 + 0.5;

                let (px0, py0) = transform.transform_xy(x0, y0);
                let (px1, py1) = transform.transform_xy(x1, y1);

                let rx = px0.min(px1);
                let ry = py0.min(py1);
                let rw = (px1 - px0).abs().max(1.0);
                let rh = (py1 - py0).abs().max(1.0);

                if let Some(rect) = Rect::from_xywh(rx, ry, rw, rh) {
                    pixmap.fill_rect(rect, &paint, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        (
            -0.5,
            self.cols as f64 - 0.5,
            -0.5,
            self.rows as f64 - 0.5,
        )
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }
}
