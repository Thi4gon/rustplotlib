use tiny_skia::Pixmap;

use crate::artists::{MarkerStyle, draw_marker};
use crate::axes3d::{Artist3D, Bounds3D, PlotArea};
use crate::colors::Color;
use crate::projection3d::Camera;

/// 3D scatter plot.
pub struct Scatter3D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub sizes: Vec<f32>,
    pub color: Color,
    pub marker: MarkerStyle,
    pub label: Option<String>,
    pub alpha: f32,
}

impl Artist3D for Scatter3D {
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    ) {
        if self.x.is_empty() {
            return;
        }

        // Project all points with depth for sorting
        let mut points: Vec<(f32, f32, f64, f32)> = self
            .x
            .iter()
            .zip(self.y.iter())
            .zip(self.z.iter())
            .enumerate()
            .map(|(i, ((&x, &y), &z))| {
                let (nx, ny, nz) = bounds.normalize(x, y, z);
                let (sx, sy, depth) = camera.project(nx, ny, nz);
                let (px, py) = plot_area.map_to_pixel(sx, sy);
                let size = if i < self.sizes.len() {
                    self.sizes[i]
                } else if !self.sizes.is_empty() {
                    self.sizes[0]
                } else {
                    6.0
                };
                (px, py, depth, size)
            })
            .collect();

        // Sort by depth (back to front, i.e., smallest depth first)
        points.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        // Draw markers
        for &(px, py, _, size) in &points {
            draw_marker(pixmap, self.marker, px, py, size, self.color, self.alpha);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let xmin = self.x.iter().cloned().fold(f64::INFINITY, f64::min);
        let xmax = self.x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ymin = self.y.iter().cloned().fold(f64::INFINITY, f64::min);
        let ymax = self.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let zmin = self.z.iter().cloned().fold(f64::INFINITY, f64::min);
        let zmax = self.z.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }
}
