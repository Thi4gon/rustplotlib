use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

/// A violin plot artist. Each dataset is drawn as a mirrored KDE shape.
pub struct ViolinPlot {
    pub data: Vec<Vec<f64>>,
    pub positions: Vec<f64>,
    pub width: f64,
    pub color: Color,
    pub show_means: bool,
    pub show_medians: bool,
    pub alpha: f32,
    pub label: Option<String>,
}

impl ViolinPlot {
    pub fn new(
        data: Vec<Vec<f64>>,
        positions: Vec<f64>,
        width: f64,
        color: Color,
        show_means: bool,
        show_medians: bool,
        alpha: f32,
    ) -> Self {
        Self {
            data,
            positions,
            width,
            color,
            show_means,
            show_medians,
            alpha,
            label: None,
        }
    }
}

/// Simple Gaussian KDE evaluation.
fn gaussian_kde(data: &[f64], eval_points: &[f64], bandwidth: f64) -> Vec<f64> {
    let n = data.len() as f64;
    if n == 0.0 || bandwidth <= 0.0 {
        return vec![0.0; eval_points.len()];
    }
    eval_points
        .iter()
        .map(|&x| {
            let sum: f64 = data
                .iter()
                .map(|&xi| {
                    let z = (x - xi) / bandwidth;
                    (-0.5 * z * z).exp()
                })
                .sum();
            sum / (n * bandwidth * (2.0 * std::f64::consts::PI).sqrt())
        })
        .collect()
}

/// Silverman's rule of thumb for bandwidth.
fn silverman_bandwidth(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n < 2.0 {
        return 1.0;
    }
    let mean = data.iter().sum::<f64>() / n;
    let var = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = var.sqrt();
    if std_dev < 1e-15 {
        return 1.0;
    }
    // Silverman's rule: h = 0.9 * min(std, IQR/1.34) * n^{-1/5}
    1.06 * std_dev * n.powf(-0.2)
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

impl Artist for ViolinPlot {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.data.is_empty() {
            return;
        }

        let ts = tiny_skia::Transform::identity();
        let n_eval = 50;

        for (i, dataset) in self.data.iter().enumerate() {
            if dataset.is_empty() {
                continue;
            }
            let pos = if i < self.positions.len() {
                self.positions[i]
            } else {
                (i + 1) as f64
            };

            let mut sorted = dataset.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let data_min = sorted[0];
            let data_max = sorted[sorted.len() - 1];

            if (data_max - data_min).abs() < 1e-15 {
                // All values the same — draw a thin horizontal line
                let (px, py) = transform.transform_xy(pos, data_min);
                let hw = 10.0_f32; // half-width in pixels
                let mut pb = PathBuilder::new();
                pb.move_to(px - hw, py);
                pb.line_to(px + hw, py);
                if let Some(path) = pb.finish() {
                    let mut paint = Paint::default();
                    let mut c = self.color;
                    c.a = (self.alpha * 255.0) as u8;
                    paint.set_color(c.to_tiny_skia());
                    paint.anti_alias = true;
                    let mut stroke = Stroke::default();
                    stroke.width = 2.0;
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }
                continue;
            }

            let bandwidth = silverman_bandwidth(&sorted);

            // Evaluation points spanning the data range
            let step = (data_max - data_min) / (n_eval as f64 - 1.0);
            let eval_points: Vec<f64> = (0..n_eval)
                .map(|j| data_min + j as f64 * step)
                .collect();

            let density = gaussian_kde(&sorted, &eval_points, bandwidth);

            // Normalize density to fit within the width
            let max_density = density.iter().cloned().fold(0.0_f64, f64::max);
            if max_density < 1e-15 {
                continue;
            }
            let half_width = self.width / 2.0;

            // Build the violin polygon: right side (positive density), then left side (mirrored)
            let mut pb = PathBuilder::new();

            // Right side: bottom to top
            let (first_px, first_py) = transform.transform_xy(pos, eval_points[0]);
            pb.move_to(first_px, first_py);

            for j in 0..n_eval {
                let d_normalized = density[j] / max_density * half_width;
                let (px, py) = transform.transform_xy(pos + d_normalized, eval_points[j]);
                pb.line_to(px, py);
            }

            // Left side: top to bottom (mirrored)
            for j in (0..n_eval).rev() {
                let d_normalized = density[j] / max_density * half_width;
                let (px, py) = transform.transform_xy(pos - d_normalized, eval_points[j]);
                pb.line_to(px, py);
            }

            pb.close();

            if let Some(path) = pb.finish() {
                let mut fill_paint = Paint::default();
                let mut fill_color = self.color;
                fill_color.a = (self.alpha * 255.0) as u8;
                fill_paint.set_color(fill_color.to_tiny_skia());
                fill_paint.anti_alias = true;
                pixmap.fill_path(&path, &fill_paint, tiny_skia::FillRule::Winding, ts, None);

                // Draw outline
                let mut outline_paint = Paint::default();
                outline_paint.set_color(self.color.to_tiny_skia());
                outline_paint.anti_alias = true;
                let mut outline_stroke = Stroke::default();
                outline_stroke.width = 1.0;
                pixmap.stroke_path(&path, &outline_paint, &outline_stroke, ts, None);
            }

            // Draw median line
            if self.show_medians {
                let med = median(&sorted);
                // Find density at median for the width
                let closest_idx = eval_points
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        ((**a) - med)
                            .abs()
                            .partial_cmp(&((**b) - med).abs())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                let d_at_med = density[closest_idx] / max_density * half_width;

                let (px_left, py) = transform.transform_xy(pos - d_at_med, med);
                let (px_right, _) = transform.transform_xy(pos + d_at_med, med);

                let mut pb = PathBuilder::new();
                pb.move_to(px_left, py);
                pb.line_to(px_right, py);
                if let Some(path) = pb.finish() {
                    let mut paint = Paint::default();
                    paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
                    paint.anti_alias = true;
                    let mut stroke = Stroke::default();
                    stroke.width = 2.0;
                    pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                }
            }

            // Draw mean marker
            if self.show_means {
                let mean_val = sorted.iter().sum::<f64>() / sorted.len() as f64;
                let (px, py) = transform.transform_xy(pos, mean_val);
                if let Some(circle) = crate::artists::circle_path(px, py, 3.0) {
                    let mut paint = Paint::default();
                    paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
                    paint.anti_alias = true;
                    pixmap.fill_path(
                        &circle,
                        &paint,
                        tiny_skia::FillRule::Winding,
                        ts,
                        None,
                    );
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.data.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }

        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        let half_width = self.width / 2.0;

        for (i, dataset) in self.data.iter().enumerate() {
            let pos = if i < self.positions.len() {
                self.positions[i]
            } else {
                (i + 1) as f64
            };

            if pos - half_width < xmin {
                xmin = pos - half_width;
            }
            if pos + half_width > xmax {
                xmax = pos + half_width;
            }

            for &val in dataset {
                if val < ymin {
                    ymin = val;
                }
                if val > ymax {
                    ymax = val;
                }
            }
        }

        if xmin >= xmax {
            xmin = 0.0;
            xmax = 1.0;
        }
        if ymin >= ymax {
            ymin = 0.0;
            ymax = 1.0;
        }

        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
