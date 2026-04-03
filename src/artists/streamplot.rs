use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};

pub struct StreamPlot {
    /// The X coordinates of the grid (2D)
    pub x: Vec<Vec<f64>>,
    /// The Y coordinates of the grid (2D)
    pub y: Vec<Vec<f64>>,
    /// The U (x-component) velocities (2D)
    pub u: Vec<Vec<f64>>,
    /// The V (y-component) velocities (2D)
    pub v: Vec<Vec<f64>>,
    pub color: Color,
    pub density: f64,
    pub linewidth: f32,
}

impl StreamPlot {
    pub fn new(
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        u: Vec<Vec<f64>>,
        v: Vec<Vec<f64>>,
        color: Color,
        density: f64,
        linewidth: f32,
    ) -> Self {
        Self { x, y, u, v, color, density, linewidth }
    }

    /// Bilinear interpolation of velocity at a point (px, py) on the grid.
    fn interpolate_velocity(&self, px: f64, py: f64) -> Option<(f64, f64)> {
        let nrows = self.x.len();
        if nrows < 2 { return None; }
        let ncols = self.x[0].len();
        if ncols < 2 { return None; }

        // Find grid bounds
        let x_min = self.x[0][0];
        let x_max = self.x[0][ncols - 1];
        let y_min = self.y[0][0];
        let y_max = self.y[nrows - 1][0];

        if px < x_min || px > x_max || py < y_min || py > y_max {
            return None;
        }

        // Compute fractional indices
        let fx = (px - x_min) / (x_max - x_min) * (ncols - 1) as f64;
        let fy = (py - y_min) / (y_max - y_min) * (nrows - 1) as f64;

        let ix = (fx as usize).min(ncols - 2);
        let iy = (fy as usize).min(nrows - 2);

        let tx = fx - ix as f64;
        let ty = fy - iy as f64;

        // Bilinear interpolation
        let u00 = self.u[iy][ix];
        let u10 = self.u[iy][ix + 1];
        let u01 = self.u[iy + 1][ix];
        let u11 = self.u[iy + 1][ix + 1];

        let v00 = self.v[iy][ix];
        let v10 = self.v[iy][ix + 1];
        let v01 = self.v[iy + 1][ix];
        let v11 = self.v[iy + 1][ix + 1];

        let u_val = u00 * (1.0 - tx) * (1.0 - ty)
            + u10 * tx * (1.0 - ty)
            + u01 * (1.0 - tx) * ty
            + u11 * tx * ty;

        let v_val = v00 * (1.0 - tx) * (1.0 - ty)
            + v10 * tx * (1.0 - ty)
            + v01 * (1.0 - tx) * ty
            + v11 * tx * ty;

        Some((u_val, v_val))
    }

    /// Integrate a streamline from (sx, sy) using Euler's method.
    fn integrate_streamline(&self, sx: f64, sy: f64) -> Vec<(f64, f64)> {
        let nrows = self.x.len();
        if nrows < 2 { return vec![]; }
        let ncols = self.x[0].len();
        if ncols < 2 { return vec![]; }

        let x_min = self.x[0][0];
        let x_max = self.x[0][ncols - 1];
        let y_min = self.y[0][0];
        let y_max = self.y[nrows - 1][0];

        let dx = (x_max - x_min) / ncols as f64;
        let dy = (y_max - y_min) / nrows as f64;
        let step_size = (dx.min(dy)) * 0.5;
        let max_steps = ((nrows + ncols) as f64 * 3.0 / self.density) as usize;
        let max_steps = max_steps.max(50).min(500);

        let mut points = Vec::with_capacity(max_steps);
        let mut px = sx;
        let mut py = sy;

        for _ in 0..max_steps {
            points.push((px, py));

            if let Some((ux, vy)) = self.interpolate_velocity(px, py) {
                let speed = (ux * ux + vy * vy).sqrt();
                if speed < 1e-10 {
                    break;
                }
                // Normalize and step
                let nx = ux / speed;
                let ny = vy / speed;
                px += nx * step_size;
                py += ny * step_size;

                // Check bounds
                if px < x_min || px > x_max || py < y_min || py > y_max {
                    break;
                }
            } else {
                break;
            }
        }

        points
    }
}

impl Artist for StreamPlot {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let nrows = self.x.len();
        if nrows < 2 { return; }
        let ncols = self.x[0].len();
        if ncols < 2 { return; }

        let ts = tiny_skia::Transform::identity();
        let mut paint = Paint::default();
        paint.set_color(self.color.to_tiny_skia());
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = self.linewidth;

        let x_min = self.x[0][0];
        let x_max = self.x[0][ncols - 1];
        let y_min = self.y[0][0];
        let y_max = self.y[nrows - 1][0];

        // Generate seed points based on density
        let n_seeds_x = ((ncols as f64 * self.density).round() as usize).max(2).min(30);
        let n_seeds_y = ((nrows as f64 * self.density).round() as usize).max(2).min(30);

        let dx = (x_max - x_min) / (n_seeds_x + 1) as f64;
        let dy = (y_max - y_min) / (n_seeds_y + 1) as f64;

        for iy in 0..n_seeds_y {
            for ix in 0..n_seeds_x {
                let sx = x_min + (ix + 1) as f64 * dx;
                let sy = y_min + (iy + 1) as f64 * dy;

                let points = self.integrate_streamline(sx, sy);
                if points.len() < 2 {
                    continue;
                }

                // Draw the streamline
                let mut pb = PathBuilder::new();
                let (px0, py0) = transform.transform_xy(points[0].0, points[0].1);
                pb.move_to(px0, py0);
                for pt in &points[1..] {
                    let (px, py) = transform.transform_xy(pt.0, pt.1);
                    pb.line_to(px, py);
                }
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }

                // Draw arrowhead at midpoint
                if points.len() >= 4 {
                    let mid = points.len() / 2;
                    let (mx, my) = transform.transform_xy(points[mid].0, points[mid].1);
                    let (mx_prev, my_prev) = transform.transform_xy(
                        points[mid - 1].0,
                        points[mid - 1].1,
                    );
                    let adx = mx - mx_prev;
                    let ady = my - my_prev;
                    let alen = (adx * adx + ady * ady).sqrt();
                    if alen > 2.0 {
                        let ux = adx / alen;
                        let uy = ady / alen;
                        let head_size = (self.linewidth * 4.0).max(5.0);
                        let perp_x = -uy;
                        let perp_y = ux;
                        let base_x = mx - ux * head_size;
                        let base_y = my - uy * head_size;

                        let mut pb = PathBuilder::new();
                        pb.move_to(mx, my);
                        pb.line_to(
                            base_x + perp_x * head_size * 0.35,
                            base_y + perp_y * head_size * 0.35,
                        );
                        pb.line_to(
                            base_x - perp_x * head_size * 0.35,
                            base_y - perp_y * head_size * 0.35,
                        );
                        pb.close();
                        if let Some(path) = pb.finish() {
                            pixmap.fill_path(
                                &path,
                                &paint,
                                tiny_skia::FillRule::Winding,
                                ts,
                                None,
                            );
                        }
                    }
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let nrows = self.x.len();
        if nrows == 0 { return (0.0, 1.0, 0.0, 1.0); }
        let ncols = self.x[0].len();
        if ncols == 0 { return (0.0, 1.0, 0.0, 1.0); }

        let x_min = self.x[0][0];
        let x_max = self.x[0][ncols - 1];
        let y_min = self.y[0][0];
        let y_max = self.y[nrows - 1][0];

        (x_min, x_max, y_min, y_max)
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
