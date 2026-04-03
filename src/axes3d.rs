use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

use crate::colors::Color;
use crate::projection3d::Camera;
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use crate::ticker::{compute_auto_ticks, format_tick_value};

/// Tab10 color palette (same as matplotlib's default).
const TAB10: [(u8, u8, u8); 10] = [
    (31, 119, 180),  // blue
    (255, 127, 14),  // orange
    (44, 160, 44),   // green
    (214, 39, 40),   // red
    (148, 103, 189), // purple
    (140, 86, 75),   // brown
    (227, 119, 194), // pink
    (127, 127, 127), // gray
    (188, 189, 34),  // olive
    (23, 190, 207),  // cyan
];

/// Bounding box in 3D data space.
pub struct Bounds3D {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl Bounds3D {
    /// Normalize a 3D point into [-1, 1] range.
    pub fn normalize(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let xr = self.xmax - self.xmin;
        let yr = self.ymax - self.ymin;
        let zr = self.zmax - self.zmin;
        let nx = if xr > 0.0 { 2.0 * (x - self.xmin) / xr - 1.0 } else { 0.0 };
        let ny = if yr > 0.0 { 2.0 * (y - self.ymin) / yr - 1.0 } else { 0.0 };
        let nz = if zr > 0.0 { 2.0 * (z - self.zmin) / zr - 1.0 } else { 0.0 };
        (nx, ny, nz)
    }
}

/// The plot area in pixel coordinates.
pub struct PlotArea {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl PlotArea {
    /// Map projected 2D coordinates (in roughly [-1.8, 1.8] range) to pixel coordinates.
    pub fn map_to_pixel(&self, sx: f64, sy: f64) -> (f32, f32) {
        let cx = (self.left + self.right) / 2.0;
        let cy = (self.top + self.bottom) / 2.0;
        let w = (self.right - self.left) * 0.40; // scale factor
        let h = (self.bottom - self.top) * 0.40;
        let scale = w.min(h);
        let px = cx + sx as f32 * scale;
        let py = cy - sy as f32 * scale; // flip y: screen y goes down
        (px, py)
    }
}

/// Trait for 3D drawable artists.
pub trait Artist3D: Send {
    /// Draw this artist onto the pixmap.
    fn draw(
        &self,
        pixmap: &mut Pixmap,
        camera: &Camera,
        bounds: &Bounds3D,
        plot_area: &PlotArea,
    );

    /// Return the data-space bounding box (xmin, xmax, ymin, ymax, zmin, zmax).
    fn data_bounds(&self) -> (f64, f64, f64, f64, f64, f64);

    /// Return the legend label, if any.
    fn legend_label(&self) -> Option<&str>;

    /// Return the representative color for this artist.
    fn legend_color(&self) -> Color;
}

/// 3D axes — manages camera, artists, labels, and rendering.
pub struct Axes3D {
    pub camera: Camera,
    pub artists: Vec<Box<dyn Artist3D>>,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub zlabel: Option<String>,
    pub xlim: Option<(f64, f64)>,
    pub ylim: Option<(f64, f64)>,
    pub zlim: Option<(f64, f64)>,
    pub grid_visible: bool,
    pub show_legend: bool,
    color_cycle_idx: usize,
    pub bg_color: Color,
    pub text_color: Color,
}

impl Axes3D {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            artists: Vec::new(),
            title: None,
            xlabel: None,
            ylabel: None,
            zlabel: None,
            xlim: None,
            ylim: None,
            zlim: None,
            grid_visible: true,
            show_legend: false,
            color_cycle_idx: 0,
            bg_color: Color::new(240, 240, 240, 255),
            text_color: Color::new(0, 0, 0, 255),
        }
    }

    /// Get the next color from the Tab10 cycle.
    pub fn next_color(&mut self) -> Color {
        let (r, g, b) = TAB10[self.color_cycle_idx % TAB10.len()];
        self.color_cycle_idx += 1;
        Color::new(r, g, b, 255)
    }

    /// Compute merged data bounds from all artists and user-set limits.
    fn compute_bounds(&self) -> Bounds3D {
        let mut xmin = f64::INFINITY;
        let mut xmax = f64::NEG_INFINITY;
        let mut ymin = f64::INFINITY;
        let mut ymax = f64::NEG_INFINITY;
        let mut zmin = f64::INFINITY;
        let mut zmax = f64::NEG_INFINITY;

        for a in &self.artists {
            let (x0, x1, y0, y1, z0, z1) = a.data_bounds();
            if x0 < xmin { xmin = x0; }
            if x1 > xmax { xmax = x1; }
            if y0 < ymin { ymin = y0; }
            if y1 > ymax { ymax = y1; }
            if z0 < zmin { zmin = z0; }
            if z1 > zmax { zmax = z1; }
        }

        // Apply user limits
        if let Some((lo, hi)) = self.xlim { xmin = lo; xmax = hi; }
        if let Some((lo, hi)) = self.ylim { ymin = lo; ymax = hi; }
        if let Some((lo, hi)) = self.zlim { zmin = lo; zmax = hi; }

        // Avoid zero-range
        if (xmax - xmin).abs() < 1e-10 { xmin -= 0.5; xmax += 0.5; }
        if (ymax - ymin).abs() < 1e-10 { ymin -= 0.5; ymax += 0.5; }
        if (zmax - zmin).abs() < 1e-10 { zmin -= 0.5; zmax += 0.5; }

        Bounds3D { xmin, xmax, ymin, ymax, zmin, zmax }
    }

    /// Draw the 3D axes and all artists.
    pub fn draw(&self, pixmap: &mut Pixmap, left: f32, top: f32, right: f32, bottom: f32) {
        let plot_area = PlotArea { left, right, top, bottom };
        let bounds = self.compute_bounds();

        // Draw background
        self.draw_background(pixmap, &plot_area);

        // Draw 3D frame/grid
        self.draw_frame(pixmap, &bounds, &plot_area);

        // Draw all artists
        for artist in &self.artists {
            artist.draw(pixmap, &self.camera, &bounds, &plot_area);
        }

        // Draw legend
        if self.show_legend {
            self.draw_legend(pixmap, &plot_area);
        }

        // Draw title
        if let Some(ref title) = self.title {
            let cx = (plot_area.left + plot_area.right) / 2.0;
            let ty = plot_area.top + 5.0;
            draw_text(
                pixmap,
                title,
                cx,
                ty,
                14.0,
                self.text_color,
                TextAnchorX::Center,
                TextAnchorY::Top,
                0.0,
            );
        }
    }

    fn draw_background(&self, pixmap: &mut Pixmap, area: &PlotArea) {
        let mut paint = Paint::default();
        paint.set_color(self.bg_color.to_tiny_skia());
        if let Some(rect) = tiny_skia::Rect::from_ltrb(area.left, area.top, area.right, area.bottom) {
            pixmap.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
        }
    }

    /// Draw the 3D bounding box frame with axis labels and ticks.
    fn draw_frame(&self, pixmap: &mut Pixmap, bounds: &Bounds3D, area: &PlotArea) {
        // The 8 corners of the unit box in normalized [-1,1] space
        let corners: [(f64, f64, f64); 8] = [
            (-1.0, -1.0, -1.0), // 0: front-left-bottom
            ( 1.0, -1.0, -1.0), // 1: front-right-bottom
            ( 1.0,  1.0, -1.0), // 2: back-right-bottom
            (-1.0,  1.0, -1.0), // 3: back-left-bottom
            (-1.0, -1.0,  1.0), // 4: front-left-top
            ( 1.0, -1.0,  1.0), // 5: front-right-top
            ( 1.0,  1.0,  1.0), // 6: back-right-top
            (-1.0,  1.0,  1.0), // 7: back-left-top
        ];

        // Project each corner
        let proj: Vec<(f32, f32)> = corners
            .iter()
            .map(|&(x, y, z)| {
                let (sx, sy, _) = self.camera.project(x, y, z);
                area.map_to_pixel(sx, sy)
            })
            .collect();

        // Define all 12 edges
        let edges: [(usize, usize); 12] = [
            // Bottom face
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Top face
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Vertical edges
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];

        let mut paint = Paint::default();
        paint.set_color_rgba8(160, 160, 160, 255);
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = 1.0;

        let ts = tiny_skia::Transform::identity();

        for &(a, b) in &edges {
            let mut pb = PathBuilder::new();
            pb.move_to(proj[a].0, proj[a].1);
            pb.line_to(proj[b].0, proj[b].1);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }

        // Draw grid lines on the bottom face (z = -1)
        if self.grid_visible {
            let mut gpaint = Paint::default();
            gpaint.set_color_rgba8(200, 200, 200, 200);
            gpaint.anti_alias = true;

            let mut gstroke = Stroke::default();
            gstroke.width = 0.5;

            let grid_n = 5;
            for i in 1..grid_n {
                let t = -1.0 + 2.0 * i as f64 / grid_n as f64;

                // Lines parallel to x-axis (varying y at each grid step along x)
                let (sx1, sy1, _) = self.camera.project(t, -1.0, -1.0);
                let (sx2, sy2, _) = self.camera.project(t, 1.0, -1.0);
                let p1 = area.map_to_pixel(sx1, sy1);
                let p2 = area.map_to_pixel(sx2, sy2);
                let mut pb = PathBuilder::new();
                pb.move_to(p1.0, p1.1);
                pb.line_to(p2.0, p2.1);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &gpaint, &gstroke, ts, None);
                }

                // Lines parallel to y-axis
                let (sx1, sy1, _) = self.camera.project(-1.0, t, -1.0);
                let (sx2, sy2, _) = self.camera.project(1.0, t, -1.0);
                let p1 = area.map_to_pixel(sx1, sy1);
                let p2 = area.map_to_pixel(sx2, sy2);
                let mut pb = PathBuilder::new();
                pb.move_to(p1.0, p1.1);
                pb.line_to(p2.0, p2.1);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &gpaint, &gstroke, ts, None);
                }
            }
        }

        // Draw axis ticks and labels
        self.draw_axis_ticks(pixmap, bounds, area);
    }

    fn draw_axis_ticks(&self, pixmap: &mut Pixmap, bounds: &Bounds3D, area: &PlotArea) {
        let tick_size = 10.0;
        let label_color = self.text_color;
        let font_size = 10.0_f32;

        // X-axis ticks along edge 0->1 (front bottom, y=-1, z=-1)
        let xticks = compute_auto_ticks(bounds.xmin, bounds.xmax, 5);
        for &tv in &xticks {
            let nx = if (bounds.xmax - bounds.xmin).abs() > 1e-10 {
                2.0 * (tv - bounds.xmin) / (bounds.xmax - bounds.xmin) - 1.0
            } else {
                0.0
            };
            let (sx, sy, _) = self.camera.project(nx, -1.0, -1.0);
            let (px, py) = area.map_to_pixel(sx, sy);
            let label = format_tick_value(tv);
            draw_text(
                pixmap, &label, px, py + tick_size,
                font_size, label_color,
                TextAnchorX::Center, TextAnchorY::Top, 0.0,
            );
        }

        // X-axis label
        if let Some(ref xlabel) = self.xlabel {
            let (sx, sy, _) = self.camera.project(0.0, -1.0, -1.0);
            let (px, py) = area.map_to_pixel(sx, sy);
            draw_text(
                pixmap, xlabel, px, py + tick_size + 18.0,
                12.0, label_color,
                TextAnchorX::Center, TextAnchorY::Top, 0.0,
            );
        }

        // Y-axis ticks along edge 0->3 (left bottom, x=-1, z=-1)
        let yticks = compute_auto_ticks(bounds.ymin, bounds.ymax, 5);
        for &tv in &yticks {
            let ny = if (bounds.ymax - bounds.ymin).abs() > 1e-10 {
                2.0 * (tv - bounds.ymin) / (bounds.ymax - bounds.ymin) - 1.0
            } else {
                0.0
            };
            let (sx, sy, _) = self.camera.project(-1.0, ny, -1.0);
            let (px, py) = area.map_to_pixel(sx, sy);
            let label = format_tick_value(tv);
            draw_text(
                pixmap, &label, px - tick_size, py,
                font_size, label_color,
                TextAnchorX::Right, TextAnchorY::Center, 0.0,
            );
        }

        // Y-axis label
        if let Some(ref ylabel) = self.ylabel {
            let (sx, sy, _) = self.camera.project(-1.0, 0.0, -1.0);
            let (px, py) = area.map_to_pixel(sx, sy);
            draw_text(
                pixmap, ylabel, px - tick_size - 25.0, py,
                12.0, label_color,
                TextAnchorX::Center, TextAnchorY::Center, 0.0,
            );
        }

        // Z-axis ticks along edge 0->4 (front-left vertical, x=-1, y=-1)
        let zticks = compute_auto_ticks(bounds.zmin, bounds.zmax, 5);
        for &tv in &zticks {
            let nz = if (bounds.zmax - bounds.zmin).abs() > 1e-10 {
                2.0 * (tv - bounds.zmin) / (bounds.zmax - bounds.zmin) - 1.0
            } else {
                0.0
            };
            let (sx, sy, _) = self.camera.project(-1.0, -1.0, nz);
            let (px, py) = area.map_to_pixel(sx, sy);
            let label = format_tick_value(tv);
            draw_text(
                pixmap, &label, px - tick_size, py,
                font_size, label_color,
                TextAnchorX::Right, TextAnchorY::Center, 0.0,
            );
        }

        // Z-axis label
        if let Some(ref zlabel) = self.zlabel {
            let (sx, sy, _) = self.camera.project(-1.0, -1.0, 0.0);
            let (px, py) = area.map_to_pixel(sx, sy);
            draw_text(
                pixmap, zlabel, px - tick_size - 30.0, py,
                12.0, label_color,
                TextAnchorX::Center, TextAnchorY::Center, 0.0,
            );
        }
    }

    fn draw_legend(&self, pixmap: &mut Pixmap, area: &PlotArea) {
        let entries: Vec<(&str, Color)> = self
            .artists
            .iter()
            .filter_map(|a| {
                a.legend_label().map(|label| (label, a.legend_color()))
            })
            .collect();

        if entries.is_empty() {
            return;
        }

        let font_size = 11.0_f32;
        let line_h = font_size + 4.0;
        let swatch_w = 16.0_f32;
        let padding = 8.0_f32;
        let entry_count = entries.len() as f32;

        // Compute legend box dimensions
        let max_label_w = entries.iter().map(|(l, _)| l.len() as f32 * font_size * 0.55).fold(0.0_f32, f32::max);
        let box_w = swatch_w + padding * 3.0 + max_label_w;
        let box_h = padding * 2.0 + entry_count * line_h;

        let bx = area.right - box_w - 10.0;
        let by = area.top + 25.0;

        // Background
        let mut bg_paint = Paint::default();
        bg_paint.set_color_rgba8(255, 255, 255, 220);
        if let Some(rect) = tiny_skia::Rect::from_xywh(bx, by, box_w, box_h) {
            pixmap.fill_rect(rect, &bg_paint, tiny_skia::Transform::identity(), None);
        }

        // Border
        let mut border_paint = Paint::default();
        border_paint.set_color_rgba8(180, 180, 180, 255);
        border_paint.anti_alias = true;
        let mut border_stroke = Stroke::default();
        border_stroke.width = 1.0;
        let mut pb = PathBuilder::new();
        pb.move_to(bx, by);
        pb.line_to(bx + box_w, by);
        pb.line_to(bx + box_w, by + box_h);
        pb.line_to(bx, by + box_h);
        pb.close();
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &border_paint, &border_stroke, tiny_skia::Transform::identity(), None);
        }

        // Entries
        for (i, (label, color)) in entries.iter().enumerate() {
            let ey = by + padding + i as f32 * line_h;

            // Swatch
            let mut sw_paint = Paint::default();
            sw_paint.set_color(color.to_tiny_skia());
            if let Some(rect) = tiny_skia::Rect::from_xywh(bx + padding, ey, swatch_w, font_size) {
                pixmap.fill_rect(rect, &sw_paint, tiny_skia::Transform::identity(), None);
            }

            // Label text
            draw_text(
                pixmap,
                label,
                bx + padding + swatch_w + padding,
                ey + font_size * 0.5,
                font_size,
                self.text_color,
                TextAnchorX::Left,
                TextAnchorY::Center,
                0.0,
            );
        }
    }
}
