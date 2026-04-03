use std::collections::HashMap;

use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap};

pub struct HexBin {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub gridsize: usize, // default 20
    pub cmap: String,
    pub mincnt: usize, // minimum count to show hex
}

impl HexBin {
    pub fn new(x: Vec<f64>, y: Vec<f64>, gridsize: usize, cmap: String, mincnt: usize) -> Self {
        HexBin {
            x,
            y,
            gridsize,
            cmap,
            mincnt,
        }
    }
}

/// Map a hex grid (col, row) to the center (x, y) in data space.
fn hex_center(col: i64, row: i64, hex_w: f64, hex_h: f64, xmin: f64, ymin: f64) -> (f64, f64) {
    let offset = if row % 2 != 0 { hex_w * 0.5 } else { 0.0 };
    let cx = xmin + col as f64 * hex_w + offset + hex_w * 0.5;
    let cy = ymin + row as f64 * hex_h * 0.75 + hex_h * 0.5;
    (cx, cy)
}

/// Find which hex a data point falls into.
fn point_to_hex(px: f64, py: f64, hex_w: f64, hex_h: f64, xmin: f64, ymin: f64) -> (i64, i64) {
    // Approximate row
    let row_f = (py - ymin) / (hex_h * 0.75);
    let row = row_f.floor() as i64;

    let offset = if row % 2 != 0 { hex_w * 0.5 } else { 0.0 };
    let col_f = (px - xmin - offset) / hex_w;
    let col = col_f.floor() as i64;

    // Check this hex and neighbors to find closest center
    let mut best_col = col;
    let mut best_row = row;
    let mut best_dist = f64::MAX;

    for dr in -1..=1 {
        for dc in -1..=1 {
            let r = row + dr;
            let c = col + dc;
            let (cx, cy) = hex_center(c, r, hex_w, hex_h, xmin, ymin);
            let dist = (px - cx).powi(2) + (py - cy).powi(2);
            if dist < best_dist {
                best_dist = dist;
                best_col = c;
                best_row = r;
            }
        }
    }

    (best_col, best_row)
}

/// Generate colormap value (viridis-like) from t in [0, 1].
fn viridis_color(t: f64) -> Color {
    let colormap: [(u8, u8, u8); 11] = [
        (68, 1, 84),
        (72, 35, 116),
        (64, 67, 135),
        (52, 94, 141),
        (41, 120, 142),
        (32, 144, 140),
        (34, 167, 132),
        (68, 190, 112),
        (121, 209, 81),
        (189, 222, 38),
        (253, 231, 37),
    ];
    let t = t.clamp(0.0, 1.0);
    let idx = t * (colormap.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = (lo + 1).min(colormap.len() - 1);
    let frac = idx - lo as f64;
    let r = (colormap[lo].0 as f64 * (1.0 - frac) + colormap[hi].0 as f64 * frac) as u8;
    let g = (colormap[lo].1 as f64 * (1.0 - frac) + colormap[hi].1 as f64 * frac) as u8;
    let b = (colormap[lo].2 as f64 * (1.0 - frac) + colormap[hi].2 as f64 * frac) as u8;
    Color::new(r, g, b, 255)
}

impl Artist for HexBin {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }

        let n = self.x.len().min(self.y.len());

        // Compute data range
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] > ymax { ymax = self.y[i]; }
        }

        let data_w = xmax - xmin;
        let data_h = ymax - ymin;
        if data_w <= 0.0 || data_h <= 0.0 {
            return;
        }

        let gridsize = self.gridsize.max(1);
        let hex_w = data_w / gridsize as f64;
        let hex_h = hex_w * 2.0 / 3.0_f64.sqrt(); // regular hexagon ratio

        // Bin data points
        let mut counts: HashMap<(i64, i64), usize> = HashMap::new();
        for i in 0..n {
            let (col, row) = point_to_hex(self.x[i], self.y[i], hex_w, hex_h, xmin, ymin);
            *counts.entry((col, row)).or_insert(0) += 1;
        }

        // Find max count for color normalization
        let max_count = counts.values().copied().max().unwrap_or(1) as f64;

        let ts = tiny_skia::Transform::identity();

        // Draw hexagons
        for (&(col, row), &count) in &counts {
            if count < self.mincnt {
                continue;
            }

            let t = count as f64 / max_count;
            let color = viridis_color(t);

            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            let (cx, cy) = hex_center(col, row, hex_w, hex_h, xmin, ymin);

            // Build hexagon path in data coords, then transform each vertex
            let r = hex_w * 0.5; // horizontal radius
            let mut pb = PathBuilder::new();

            for i in 0..6 {
                let angle = std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 6.0;
                let hx = cx + r * angle.cos();
                let hy = cy + r * angle.sin();
                let (px, py) = transform.transform_xy(hx, hy);
                if i == 0 {
                    pb.move_to(px, py);
                } else {
                    pb.line_to(px, py);
                }
            }
            pb.close();

            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
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
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] > ymax { ymax = self.y[i]; }
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(68, 1, 84, 255)
    }
}
