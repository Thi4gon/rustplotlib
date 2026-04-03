use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

pub struct Contour {
    pub x: Vec<Vec<f64>>,  // 2D grid X coords
    pub y: Vec<Vec<f64>>,  // 2D grid Y coords
    pub z: Vec<Vec<f64>>,  // 2D grid Z values
    pub levels: Vec<f64>,
    pub colors: Vec<Color>,
    pub filled: bool,      // true = contourf, false = contour
    pub linewidth: f32,
}

impl Contour {
    pub fn new(
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        levels: Option<Vec<f64>>,
        colors: Option<Vec<Color>>,
        filled: bool,
        linewidth: f32,
    ) -> Self {
        // Auto-generate levels if not provided
        let levels = levels.unwrap_or_else(|| {
            auto_levels(&z, 10)
        });

        // Auto-generate colors if not provided
        let colors = colors.unwrap_or_else(|| {
            let n = if filled { levels.len().saturating_sub(1).max(1) } else { levels.len().max(1) };
            generate_colormap(n)
        });

        Contour {
            x,
            y,
            z,
            levels,
            colors,
            filled,
            linewidth,
        }
    }
}

/// Auto-generate equally spaced levels from the Z data range.
fn auto_levels(z: &[Vec<f64>], n: usize) -> Vec<f64> {
    let mut zmin = f64::MAX;
    let mut zmax = f64::MIN;
    for row in z {
        for &val in row {
            if val.is_finite() {
                if val < zmin { zmin = val; }
                if val > zmax { zmax = val; }
            }
        }
    }
    if zmin >= zmax {
        return vec![zmin];
    }
    let step = (zmax - zmin) / (n as f64 + 1.0);
    (1..=n).map(|i| zmin + step * i as f64).collect()
}

/// Generate a simple viridis-like colormap with n colors.
fn generate_colormap(n: usize) -> Vec<Color> {
    if n == 0 {
        return vec![Color::new(68, 1, 84, 255)];
    }
    let colormap: Vec<(u8, u8, u8)> = vec![
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
    (0..n)
        .map(|i| {
            let t = if n > 1 { i as f64 / (n - 1) as f64 } else { 0.5 };
            let idx = (t * (colormap.len() - 1) as f64).min((colormap.len() - 1) as f64);
            let lo = idx.floor() as usize;
            let hi = (lo + 1).min(colormap.len() - 1);
            let frac = idx - lo as f64;
            let r = (colormap[lo].0 as f64 * (1.0 - frac) + colormap[hi].0 as f64 * frac) as u8;
            let g = (colormap[lo].1 as f64 * (1.0 - frac) + colormap[hi].1 as f64 * frac) as u8;
            let b = (colormap[lo].2 as f64 * (1.0 - frac) + colormap[hi].2 as f64 * frac) as u8;
            Color::new(r, g, b, 255)
        })
        .collect()
}

/// Interpolate the position along an edge where the level crosses.
fn interpolate(v1: f64, v2: f64, level: f64) -> f64 {
    if (v2 - v1).abs() < 1e-15 {
        0.5
    } else {
        (level - v1) / (v2 - v1)
    }
}

impl Artist for Contour {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let nrows = self.z.len();
        if nrows < 2 { return; }
        let ncols = self.z[0].len();
        if ncols < 2 { return; }

        let ts = tiny_skia::Transform::identity();

        if self.filled {
            self.draw_filled(pixmap, transform, nrows, ncols, ts);
        } else {
            self.draw_lines(pixmap, transform, nrows, ncols, ts);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for row in &self.x {
            for &v in row {
                if v < xmin { xmin = v; }
                if v > xmax { xmax = v; }
            }
        }
        for row in &self.y {
            for &v in row {
                if v < ymin { ymin = v; }
                if v > ymax { ymax = v; }
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
        if !self.colors.is_empty() {
            self.colors[0]
        } else {
            Color::new(0, 0, 0, 255)
        }
    }
}

impl Contour {
    /// Draw contour lines using marching squares.
    fn draw_lines(
        &self,
        pixmap: &mut Pixmap,
        transform: &Transform,
        nrows: usize,
        ncols: usize,
        ts: tiny_skia::Transform,
    ) {
        for (level_idx, &level) in self.levels.iter().enumerate() {
            let color = self.colors[level_idx % self.colors.len()];
            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            let mut stroke = Stroke::default();
            stroke.width = self.linewidth;

            let mut pb = PathBuilder::new();
            let mut has_segments = false;

            for row in 0..nrows - 1 {
                for col in 0..ncols - 1 {
                    let z00 = self.z[row][col];
                    let z10 = self.z[row][col + 1];
                    let z01 = self.z[row + 1][col];
                    let z11 = self.z[row + 1][col + 1];

                    let x00 = self.x[row][col];
                    let x10 = self.x[row][col + 1];
                    let x01 = self.x[row + 1][col];
                    let x11 = self.x[row + 1][col + 1];

                    let y00 = self.y[row][col];
                    let y10 = self.y[row][col + 1];
                    let y01 = self.y[row + 1][col];
                    let y11 = self.y[row + 1][col + 1];

                    // Classify corners: bit 0 = TL(00), bit 1 = TR(10), bit 2 = BR(11), bit 3 = BL(01)
                    let mut case = 0u8;
                    if z00 >= level { case |= 1; }
                    if z10 >= level { case |= 2; }
                    if z11 >= level { case |= 4; }
                    if z01 >= level { case |= 8; }

                    // Edge midpoints by interpolation
                    // Top edge: between (00) and (10)
                    let top_t = interpolate(z00, z10, level);
                    let top_x = x00 + (x10 - x00) * top_t;
                    let top_y = y00 + (y10 - y00) * top_t;

                    // Right edge: between (10) and (11)
                    let right_t = interpolate(z10, z11, level);
                    let right_x = x10 + (x11 - x10) * right_t;
                    let right_y = y10 + (y11 - y10) * right_t;

                    // Bottom edge: between (01) and (11)
                    let bottom_t = interpolate(z01, z11, level);
                    let bottom_x = x01 + (x11 - x01) * bottom_t;
                    let bottom_y = y01 + (y11 - y01) * bottom_t;

                    // Left edge: between (00) and (01)
                    let left_t = interpolate(z00, z01, level);
                    let left_x = x00 + (x01 - x00) * left_t;
                    let left_y = y00 + (y01 - y00) * left_t;

                    // Segments based on marching squares case
                    let segments: Vec<((f64, f64), (f64, f64))> = match case {
                        0 | 15 => vec![],
                        1 | 14 => vec![((top_x, top_y), (left_x, left_y))],
                        2 | 13 => vec![((top_x, top_y), (right_x, right_y))],
                        3 | 12 => vec![((left_x, left_y), (right_x, right_y))],
                        4 | 11 => vec![((right_x, right_y), (bottom_x, bottom_y))],
                        5 => vec![
                            ((top_x, top_y), (right_x, right_y)),
                            ((left_x, left_y), (bottom_x, bottom_y)),
                        ],
                        6 | 9 => vec![((top_x, top_y), (bottom_x, bottom_y))],
                        7 | 8 => vec![((left_x, left_y), (bottom_x, bottom_y))],
                        10 => vec![
                            ((top_x, top_y), (left_x, left_y)),
                            ((right_x, right_y), (bottom_x, bottom_y)),
                        ],
                        _ => vec![],
                    };

                    for ((sx, sy), (ex, ey)) in segments {
                        let (px1, py1) = transform.transform_xy(sx, sy);
                        let (px2, py2) = transform.transform_xy(ex, ey);
                        pb.move_to(px1, py1);
                        pb.line_to(px2, py2);
                        has_segments = true;
                    }
                }
            }

            if has_segments {
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }
            }
        }
    }

    /// Draw filled contours: for each pair of adjacent levels, fill the region between them.
    fn draw_filled(
        &self,
        pixmap: &mut Pixmap,
        transform: &Transform,
        nrows: usize,
        ncols: usize,
        ts: tiny_skia::Transform,
    ) {
        if self.levels.len() < 2 { return; }

        for li in 0..self.levels.len() - 1 {
            let lo_level = self.levels[li];
            let hi_level = self.levels[li + 1];
            let color = self.colors[li % self.colors.len()];

            let mut paint = Paint::default();
            paint.set_color(color.to_tiny_skia());
            paint.anti_alias = true;

            // For each cell, draw quads that are between lo_level and hi_level.
            // Simplified approach: draw each cell that has Z values in range as a filled quad.
            for row in 0..nrows - 1 {
                for col in 0..ncols - 1 {
                    let z00 = self.z[row][col];
                    let z10 = self.z[row][col + 1];
                    let z01 = self.z[row + 1][col];
                    let z11 = self.z[row + 1][col + 1];

                    let z_min = z00.min(z10).min(z01).min(z11);
                    let z_max = z00.max(z10).max(z01).max(z11);

                    // Skip cells completely outside this level band
                    if z_max < lo_level || z_min > hi_level {
                        continue;
                    }

                    let x00 = self.x[row][col];
                    let x10 = self.x[row][col + 1];
                    let x01 = self.x[row + 1][col];
                    let x11 = self.x[row + 1][col + 1];

                    let y00 = self.y[row][col];
                    let y10 = self.y[row][col + 1];
                    let y01 = self.y[row + 1][col];
                    let y11 = self.y[row + 1][col + 1];

                    let (p00x, p00y) = transform.transform_xy(x00, y00);
                    let (p10x, p10y) = transform.transform_xy(x10, y10);
                    let (p01x, p01y) = transform.transform_xy(x01, y01);
                    let (p11x, p11y) = transform.transform_xy(x11, y11);

                    let mut pb = PathBuilder::new();
                    pb.move_to(p00x, p00y);
                    pb.line_to(p10x, p10y);
                    pb.line_to(p11x, p11y);
                    pb.line_to(p01x, p01y);
                    pb.close();

                    if let Some(path) = pb.finish() {
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                    }
                }
            }
        }
    }
}
